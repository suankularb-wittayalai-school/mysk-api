use crate::{
    helpers::date::get_current_academic_year,
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::{
        ActionType, Authorizable, authorize_default_read_only, authorize_read_only, deny,
    },
    prelude::*,
};
use sqlx::{PgPool, query};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TeacherRole {
    id: Uuid,
    user_id: Uuid,
    source: String,
}

impl Authorizable for TeacherRole {
    async fn authorize_classroom(
        &self,
        classroom: &DbClassroom,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // Teachers can update the classroom if they're an advisor
        if matches!(action, ActionType::Update) {
            let advisor_at_classroom_id = DbTeacher::get_teacher_advisor_at(pool, self.id, None)
                .await?
                .ok_or(deny(&self.source).unwrap_err())?;

            return if advisor_at_classroom_id == classroom.id {
                Ok(())
            } else {
                deny(&self.source)
            };
        }

        authorize_read_only(action, &self.source)
    }

    async fn authorize_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // Due to certain constraints and to limit code complexity, this function will not perform
        // any authorization on the creation of a contact. Instead, the permissions check should
        // be performed using the proper route extractors and any other clause guards in-route.
        assert!(
            !matches!(action, ActionType::Create),
            "
            `ActionType::Create` can't be handled by `TeacherRole::authorize_contact`. See comment \
            at function body for more information\
            ",
        );

        let contact_belongs_to = query!(
            "\
            SELECT COALESCE(\
                CASE WHEN ac.contact_id IS NOT NULL THEN 'classroom' END,\
                CASE WHEN uc.contact_id IS NOT NULL THEN 'club' END,\
                CASE WHEN pc.contact_id IS NOT NULL THEN 'person' END,\
                'none'\
            ) AS role FROM contacts AS c \
            LEFT JOIN classroom_contacts AS ac ON ac.contact_id = c.id \
            LEFT JOIN club_contacts AS uc ON uc.contact_id = c.id \
            LEFT JOIN person_contacts AS pc ON pc.contact_id = c.id \
            WHERE c.id = $1\
            ",
            contact.id,
        )
        .fetch_one(pool)
        .await?
        .role
        .unwrap();

        match contact_belongs_to.as_str() {
            "classroom" => {
                self.authorize_classroom_contact(contact, pool, action)
                    .await
            }
            "person" => self.authorize_person_contact(contact, pool, action).await,
            // These are "ghost" contacts in the database, a data mishandling issue so they're only
            // allowed to be read from but not written to
            "club" | "none" => authorize_read_only(action, &self.source),
            _ => unreachable!(),
        }
    }

    async fn authorize_student(&self, _: &DbStudent, _: &PgPool, action: ActionType) -> Result<()> {
        authorize_default_read_only(action, &self.source)
    }

    async fn authorize_subject(
        &self,
        subject: &DbSubject,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        if matches!(action, ActionType::Update) {
            let is_subject_teacher = query!(
                "\
                SELECT EXISTS (\
                    SELECT FROM subject_teachers \
                    WHERE teacher_id = $1 AND subject_id = $2 AND year = $3 UNION \
                    SELECT FROM subject_co_teachers \
                    WHERE teacher_id = $1 AND subject_id = $2 AND year = $3\
                )\
                ",
                self.id,
                subject.id,
                get_current_academic_year(None),
            )
            .fetch_one(pool)
            .await?
            .exists
            .unwrap_or(false);

            return if is_subject_teacher {
                Ok(())
            } else {
                deny(&self.source)
            };
        }

        authorize_read_only(action, &self.source)
    }

    async fn authorize_teacher(
        &self,
        teacher: &DbTeacher,
        _: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // Teachers can get their own private details and update themselves
        if matches!(action, ActionType::ReadDetailed | ActionType::Update) {
            let teacher_user_id = teacher.user_id.ok_or(deny(&self.source).unwrap_err())?;

            return if self.user_id == teacher_user_id {
                Ok(())
            } else {
                deny(&self.source)
            };
        }

        authorize_read_only(action, &self.source)
    }
}

impl TeacherRole {
    pub fn new(id: Uuid, user_id: Uuid, source: String) -> Self {
        Self {
            id,
            user_id,
            source,
        }
    }

    async fn authorize_classroom_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        if matches!(action, ActionType::Update | ActionType::Delete) {
            let advisor_at_classroom_id = DbTeacher::get_teacher_advisor_at(pool, self.id, None)
                .await?
                .ok_or(deny(&self.source).unwrap_err())?;

            let contact_classroom_id = query!(
                "SELECT classroom_id FROM classroom_contacts WHERE contact_id = $1",
                contact.id,
            )
            .fetch_one(pool)
            .await?
            .classroom_id;

            // Check if `self` is an advisor at the given contact's classroom
            return if advisor_at_classroom_id == contact_classroom_id {
                Ok(())
            } else {
                // Teachers can't write to classroom contacts outside of their advising classroom
                deny(&self.source)
            };
        }

        authorize_read_only(action, &self.source)
    }

    async fn authorize_person_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        let owned = query!(
            "\
            SELECT EXISTS (\
                SELECT FROM contacts AS c \
                JOIN person_contacts AS pc ON pc.contact_id = c.id \
                JOIN teachers AS t ON t.person_id = pc.person_id \
                WHERE t.id = $1 AND c.id = $2\
            )\
            ",
            self.id,
            contact.id,
        )
        .fetch_one(pool)
        .await?
        .exists
        .unwrap_or(false);

        // Teachers can always do any action on their own contact
        if owned {
            return Ok(());
        }

        let is_ghost_contact = query!(
            "\
            SELECT (s.person_id IS NOT NULL OR t.person_id IS NOT NULL) AS is_ghost_contact \
            FROM person_contacts AS pc \
            LEFT JOIN students AS s ON pc.person_id = s.person_id \
            LEFT JOIN teachers AS t ON pc.person_id = t.person_id WHERE pc.contact_id = $1\
            ",
            contact.id,
        )
        .fetch_one(pool)
        .await?
        .is_ghost_contact
        .unwrap_or(true);

        // Teachers can read all contacts except for "ghost" contacts (which every user can't read)
        if is_ghost_contact {
            deny(&self.source)
        } else {
            authorize_read_only(action, &self.source)
        }
    }
}

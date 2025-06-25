use crate::{
    models::{
        classroom::db::DbClassroom, club::db::DbClub, contact::db::DbContact,
        online_teaching_reports::db::DbOnlineTeachingReports, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::{
        ActionType, Authorizable, authorize_default_read_only, authorize_read_only, deny,
    },
    prelude::*,
};
use sqlx::{PgConnection, query};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct StudentRole {
    id: Uuid,
    user_id: Uuid,
    source: String,
}

impl Authorizable for StudentRole {
    async fn authorize_classroom(
        &self,
        _: &DbClassroom,
        _: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        authorize_read_only(action, &self.source)
    }

    #[allow(clippy::too_many_lines)]
    async fn authorize_contact(
        &self,
        contact: &DbContact,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        // Due to certain constraints and to limit code complexity, this function will not perform
        // any authorization on the creation of a contact. Instead, the permissions check should
        // be performed using the proper route extractors and any other clause guards in-route.
        assert!(
            !matches!(action, ActionType::Create),
            "
            `ActionType::Create` can't be handled by `StudentRole::authorize_contact`. See comment \
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
        .fetch_one(&mut *conn)
        .await?
        .role
        .unwrap();

        match contact_belongs_to.as_str() {
            "classroom" => {
                self.authorize_classroom_contact(contact, conn, action)
                    .await
            }
            "club" => self.authorize_club_contact(contact, conn, action).await,
            "person" => self.authorize_person_contact(contact, conn, action).await,
            // These are "ghost" contacts in the database, a data mishandling issue so they're only
            // allowed to be read from but not written to
            "none" => authorize_read_only(action, &self.source),
            _ => unreachable!(),
        }
    }

    async fn authorize_online_teaching_reports(
        &self,
        _: &DbOnlineTeachingReports,
        _: &mut PgConnection,
        _: ActionType,
    ) -> Result<()> {
        Err(Error::InvalidPermission(
            "Insufficient permissions to perform this action".to_string(),
            self.source.to_string(),
        ))
    }

    async fn authorize_student(
        &self,
        student: &DbStudent,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        // Students can get their own private details and update themselves
        if matches!(action, ActionType::ReadDetailed | ActionType::Update) {
            let student_user_id = student.user_id.ok_or(deny(&self.source).unwrap_err())?;

            return if self.user_id == student_user_id {
                Ok(())
            } else {
                deny(&self.source)
            };
        }

        // Students can get read default variants of their classmates
        if matches!(action, ActionType::ReadDefault) {
            let self_classroom = DbStudent::get_student_classroom(&mut *conn, self.id, None)
                .await?
                .ok_or(deny(&self.source).unwrap_err())?;
            let student_classroom = DbStudent::get_student_classroom(conn, student.id, None)
                .await?
                .ok_or(deny(&self.source).unwrap_err())?;

            return if self_classroom.id == student_classroom.id {
                Ok(())
            } else {
                deny(&self.source)
            };
        }

        authorize_read_only(action, &self.source)
    }

    async fn authorize_subject(
        &self,
        _: &DbSubject,
        _: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        authorize_read_only(action, &self.source)
    }

    async fn authorize_teacher(
        &self,
        _: &DbTeacher,
        _: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        authorize_default_read_only(action, &self.source)
    }
}

impl StudentRole {
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
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        let student_classroom = DbStudent::get_student_classroom(&mut *conn, self.id, None)
            .await?
            // If the student doesn't belong to a classroom, deny access for classroom contacts
            .ok_or(deny(&self.source).unwrap_err())?;

        let contact_classroom_id = query!(
            "SELECT classroom_id FROM classroom_contacts WHERE contact_id = $1",
            contact.id,
        )
        .fetch_one(conn)
        .await?
        .classroom_id;

        // Check if `self` is in the given contact's classroom
        if student_classroom.id == contact_classroom_id {
            authorize_read_only(action, &self.source)
        } else {
            // Students can't access classroom contacts outside of their own classroom
            deny(&self.source)
        }
    }

    async fn authorize_club_contact(
        &self,
        contact: &DbContact,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        let club_contact = query!(
            "SELECT club_id FROM club_contacts WHERE contact_id = $1",
            contact.id,
        )
        .fetch_one(&mut *conn)
        .await?;

        // Everyone can read club contacts
        if matches!(
            action,
            ActionType::ReadIdOnly
                | ActionType::ReadCompact
                | ActionType::ReadDefault
                | ActionType::ReadDetailed,
        ) {
            return Ok(());
        }

        // Check if student is a club staff of the club that has the given contact, if not then
        // deny access for update and delete
        let club_staffs = DbClub::get_club_staffs(conn, club_contact.club_id).await?;
        if club_staffs.contains(&self.id) {
            Ok(())
        } else {
            deny(&self.source)
        }
    }

    async fn authorize_person_contact(
        &self,
        contact: &DbContact,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        let owned = query!(
            "\
            SELECT EXISTS (\
                SELECT FROM contacts \
                JOIN person_contacts ON person_contacts.contact_id = contacts.id \
                JOIN students ON students.person_id = person_contacts.person_id \
                WHERE students.id = $1 AND contacts.id = $2\
            )\
            ",
            self.id,
            contact.id,
        )
        .fetch_one(&mut *conn)
        .await?
        .exists
        .unwrap_or(false);

        // Students can always do any action on their own contact
        if owned {
            return Ok(());
        }

        let contact_belongs_to = query!(
            "\
            SELECT COALESCE(\
                CASE WHEN s.person_id IS NOT NULL THEN 'student' END,\
                CASE WHEN t.person_id IS NOT NULL THEN 'teacher' END,\
                'none'\
            ) AS role FROM person_contacts AS pc \
            LEFT JOIN students AS s ON pc.person_id = s.person_id \
            LEFT JOIN teachers AS t ON pc.person_id = t.person_id WHERE pc.contact_id = $1\
            ",
            contact.id,
        )
        .fetch_one(&mut *conn)
        .await?
        .role
        .unwrap_or("none".to_string());

        match contact_belongs_to.as_str() {
            // Classmate contacts
            "student" => {
                let student_classroom = DbStudent::get_student_classroom(&mut *conn, self.id, None)
                    .await?
                    // If the student doesn't belong to a classroom, deny access for person contacts
                    .ok_or(deny(&self.source).unwrap_err())?;

                let contact_student = query!(
                    "\
                    SELECT students.id FROM person_contacts \
                    JOIN students ON students.person_id = person_contacts.person_id \
                    WHERE contact_id = $1\
                    ",
                    contact.id,
                )
                .fetch_one(&mut *conn)
                .await?;

                let contact_student_classroom =
                    DbStudent::get_student_classroom(conn, contact_student.id, None)
                        .await?
                        .ok_or(deny(&self.source).unwrap_err())?;

                // Check if `self` and the given contact's student are classmates
                if student_classroom.id == contact_student_classroom.id {
                    authorize_read_only(action, &self.source)
                } else {
                    // Students can't access contacts that are not of their classmates'
                    deny(&self.source)
                }
            }

            // Students can't access contacts of other non-students
            "teacher" | "none" => deny(&self.source),
            _ => unreachable!(),
        }
    }
}

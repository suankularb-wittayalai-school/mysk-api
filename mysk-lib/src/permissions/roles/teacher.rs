use crate::{
    helpers::date::get_current_academic_year,
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::authorizer::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct TeacherRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub source: String,
}

#[async_trait]
impl Authorizer for TeacherRole {
    async fn authorize_classroom(
        &self,
        classroom: &DbClassroom,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        let advisor_at = DbTeacher::get_teacher_advisor_at(pool, self.id, None).await?;
        let owned = advisor_at.is_some();

        match action {
            // Owned
            // Unwrap-safe because it has already been checked if it is owned
            ActionType::Update if owned && advisor_at.unwrap() == classroom.id => Ok(()),
            // Others
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed => Ok(()),
            ActionType::Create | ActionType::Update | ActionType::Delete => {
                Err(Error::InvalidPermission(
                    "Insufficient permissions to perform this action".to_string(),
                    self.source.to_string(),
                ))
            }
        }
    }

    async fn authorize_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        let owned = query!(
            "
            SELECT EXISTS (
                SELECT FROM contacts
                INNER JOIN person_contacts ON person_contacts.contact_id = contacts.id
                INNER JOIN people ON people.id = person_contacts.person_id
                INNER JOIN teachers ON teachers.person_id = people.id
                WHERE teachers.id = $1 AND contacts.id = $2
            )
            ",
            self.id,
            contact.id,
        )
        .fetch_one(pool)
        .await?
        .exists
        .unwrap_or(false);

        match action {
            // Owned
            _ if owned => Ok(()),
            // Others
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed => Ok(()),
            ActionType::Create | ActionType::Update | ActionType::Delete => {
                Err(Error::InvalidPermission(
                    "Insufficient permissions to perform this action".to_string(),
                    self.source.to_string(),
                ))
            }
        }
    }

    async fn authorize_student(&self, _: &DbStudent, _: &PgPool, action: ActionType) -> Result<()> {
        match action {
            ActionType::ReadIdOnly | ActionType::ReadCompact | ActionType::ReadDefault => Ok(()),
            ActionType::Create
            | ActionType::ReadDetailed
            | ActionType::Update
            | ActionType::Delete => Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
                self.source.to_string(),
            )),
        }
    }

    async fn authorize_subject(
        &self,
        subject: &DbSubject,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        let owned = query!(
            "
            SELECT EXISTS (
                SELECT FROM subject_teachers
                WHERE teacher_id = $1 AND subject_id = $2 AND year = $3
                UNION
                SELECT FROM subject_co_teachers
                WHERE teacher_id = $1 AND subject_id = $2 AND year = $3
            )
            ",
            self.id,
            subject.id,
            get_current_academic_year(None),
        )
        .fetch_one(pool)
        .await?
        .exists
        .unwrap_or(false);

        match action {
            // Owned
            ActionType::Update if owned => Ok(()),
            // Others
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed => Ok(()),
            ActionType::Create | ActionType::Update | ActionType::Delete => {
                Err(Error::InvalidPermission(
                    "Insufficient permissions to perform this action".to_string(),
                    self.source.to_string(),
                ))
            }
        }
    }

    async fn authorize_teacher(
        &self,
        teacher: &DbTeacher,
        _: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // Unwrap-safe because it is guaranteed prior by get_authorizer
        let owned = self.user_id == teacher.user_id.unwrap();

        match action {
            // Owned
            ActionType::ReadDetailed | ActionType::Update if owned => Ok(()),
            // Others
            ActionType::ReadIdOnly | ActionType::ReadCompact | ActionType::ReadDefault => Ok(()),
            ActionType::Create
            | ActionType::ReadDetailed
            | ActionType::Update
            | ActionType::Delete => Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
                self.source.to_string(),
            )),
        }
    }
}

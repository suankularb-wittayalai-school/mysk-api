use crate::{
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::traits::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use sqlx::{query, PgPool};

#[async_trait]
impl Authorizer for DbStudent {
    async fn authorize_classroom(
        &self,
        _: &DbClassroom,
        _: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()> {
        match action {
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed => Ok(()),
            ActionType::Create | ActionType::Update | ActionType::Delete => {
                Err(Error::InvalidPermission(
                    "Insufficient permissions to perform this action".to_string(),
                    source,
                ))
            }
        }
    }

    async fn authorize_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()> {
        let owned = query!(
            "
            SELECT EXISTS (
                SELECT FROM contacts
                INNER JOIN person_contacts ON person_contacts.contact_id = contacts.id
                INNER JOIN people ON people.id = person_contacts.person_id
                INNER JOIN students ON students.person_id = people.id
                WHERE students.id = $1 AND contacts.id = $2
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
                    source,
                ))
            }
        }
    }

    async fn authorize_student(
        &self,
        student: &DbStudent,
        pool: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()> {
        let owned = self.id == student.id;
        let self_class = DbStudent::get_student_classroom(pool, self.id, None).await?;
        let student_class = DbStudent::get_student_classroom(pool, student.id, None).await?;
        let same_class = self_class.is_some()
            && student_class.is_some()
            && self_class.unwrap().id == student_class.unwrap().id;

        match action {
            // Owned
            ActionType::ReadDetailed | ActionType::Update if owned => Ok(()),
            // Same Class
            ActionType::ReadDefault if same_class => Ok(()),
            // Others
            ActionType::ReadIdOnly | ActionType::ReadCompact => Ok(()),
            ActionType::Create
            | ActionType::ReadDefault
            | ActionType::ReadDetailed
            | ActionType::Update
            | ActionType::Delete => Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
                source,
            )),
        }
    }

    async fn authorize_subject(
        &self,
        _: &DbSubject,
        _: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()> {
        match action {
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed => Ok(()),
            ActionType::Create | ActionType::Update | ActionType::Delete => {
                Err(Error::InvalidPermission(
                    "Insufficient permissions to perform this action".to_string(),
                    source,
                ))
            }
        }
    }

    async fn authorize_teacher(
        &self,
        _: &DbTeacher,
        _: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()> {
        match action {
            ActionType::ReadIdOnly | ActionType::ReadCompact | ActionType::ReadDefault => Ok(()),
            ActionType::Create
            | ActionType::ReadDetailed
            | ActionType::Update
            | ActionType::Delete => Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
                source,
            )),
        }
    }
}

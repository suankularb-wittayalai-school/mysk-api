use crate::{
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::authorizer::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ManagementRole {
    pub user_id: Uuid,
    pub source: String,
}

#[async_trait]
impl Authorizer for ManagementRole {
    async fn authorize_classroom(
        &self,
        _: &DbClassroom,
        _: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        match action {
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

    async fn authorize_contact(&self, _: &DbContact, _: &PgPool, action: ActionType) -> Result<()> {
        match action {
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

    async fn authorize_subject(&self, _: &DbSubject, _: &PgPool, action: ActionType) -> Result<()> {
        match action {
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

    async fn authorize_teacher(&self, _: &DbTeacher, _: &PgPool, action: ActionType) -> Result<()> {
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
}

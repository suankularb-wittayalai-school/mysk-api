use crate::{
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::{authorize_default_read_only, authorize_read_only, ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ManagementRole {
    #[allow(dead_code)]
    user_id: Uuid,
    source: String,
}

#[async_trait]
impl Authorizer for ManagementRole {
    async fn authorize_classroom(
        &self,
        _: &DbClassroom,
        _: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        authorize_read_only(action, &self.source)
    }

    async fn authorize_contact(&self, _: &DbContact, _: &PgPool, action: ActionType) -> Result<()> {
        authorize_read_only(action, &self.source)
    }

    async fn authorize_student(&self, _: &DbStudent, _: &PgPool, action: ActionType) -> Result<()> {
        authorize_default_read_only(action, &self.source)
    }

    async fn authorize_subject(&self, _: &DbSubject, _: &PgPool, action: ActionType) -> Result<()> {
        authorize_read_only(action, &self.source)
    }

    async fn authorize_teacher(&self, _: &DbTeacher, _: &PgPool, action: ActionType) -> Result<()> {
        authorize_default_read_only(action, &self.source)
    }

    fn clone_to_arc(&self) -> Arc<dyn Authorizer> {
        Arc::new(self.clone())
    }
}

impl ManagementRole {
    pub fn new(user_id: Uuid, source: String) -> Self {
        Self { user_id, source }
    }
}

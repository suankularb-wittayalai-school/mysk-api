use crate::{
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::{ActionType, Authorizable, authorize_default_read_only, authorize_read_only},
    prelude::*,
};
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ManagementRole {
    #[allow(dead_code)]
    user_id: Uuid,
    source: String,
}

impl Authorizable for ManagementRole {
    async fn authorize_classroom(
        &self,
        _: &DbClassroom,
        _: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        authorize_read_only(action, &self.source)
    }

    async fn authorize_contact(
        &self,
        _: &DbContact,
        _: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        authorize_read_only(action, &self.source)
    }

    async fn authorize_student(
        &self,
        _: &DbStudent,
        _: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        authorize_default_read_only(action, &self.source)
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

impl ManagementRole {
    pub fn new(user_id: Uuid, source: String) -> Self {
        Self { user_id, source }
    }
}

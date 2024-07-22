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

#[derive(Clone)]
pub struct AdminRole;

#[async_trait]
impl Authorizer for AdminRole {
    async fn authorize_classroom(&self, _: &DbClassroom, _: &PgPool, _: ActionType) -> Result<()> {
        Ok(())
    }

    async fn authorize_contact(&self, _: &DbContact, _: &PgPool, _: ActionType) -> Result<()> {
        Ok(())
    }

    async fn authorize_student(&self, _: &DbStudent, _: &PgPool, _: ActionType) -> Result<()> {
        Ok(())
    }

    async fn authorize_subject(&self, _: &DbSubject, _: &PgPool, _: ActionType) -> Result<()> {
        Ok(())
    }

    async fn authorize_teacher(&self, _: &DbTeacher, _: &PgPool, _: ActionType) -> Result<()> {
        Ok(())
    }
}

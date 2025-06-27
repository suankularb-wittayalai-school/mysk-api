use crate::{
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::authorizer::{ActionType, Authorizable},
    prelude::*,
};
use sqlx::PgConnection;

#[derive(Clone, Debug)]
pub struct AdminRole;

impl Authorizable for AdminRole {
    async fn authorize_classroom(
        &self,
        _: &DbClassroom,
        _: &mut PgConnection,
        _: ActionType,
    ) -> Result<()> {
        Ok(())
    }

    async fn authorize_contact(
        &self,
        _: &DbContact,
        _: &mut PgConnection,
        _: ActionType,
    ) -> Result<()> {
        Ok(())
    }

    async fn authorize_student(
        &self,
        _: &DbStudent,
        _: &mut PgConnection,
        _: ActionType,
    ) -> Result<()> {
        Ok(())
    }

    async fn authorize_subject(
        &self,
        _: &DbSubject,
        _: &mut PgConnection,
        _: ActionType,
    ) -> Result<()> {
        Ok(())
    }

    async fn authorize_teacher(
        &self,
        _: &DbTeacher,
        _: &mut PgConnection,
        _: ActionType,
    ) -> Result<()> {
        Ok(())
    }
}

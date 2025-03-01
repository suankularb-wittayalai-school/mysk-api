use crate::{
    models::{
        certificate::db::DbCertificate, classroom::db::DbClassroom, club::db::DbClub,
        club_request::db::DbClubRequest, contact::db::DbContact,
        elective_subject::db::DbElectiveSubject, elective_trade_offer::db::DbElectiveTradeOffer,
        enums::UserRole, online_teaching_reports::db::DbOnlineTeachingReports,
        organization::db::DbOrganization, person::db::DbPerson, student::db::DbStudent,
        subject::db::DbSubject, subject_group::db::DbSubjectGroup, teacher::db::DbTeacher,
        user::User,
    },
    permissions::roles::{AdminRole, ManagementRole, StudentRole, TeacherRole},
    prelude::*,
};
use async_trait::async_trait;
use sqlx::{query, PgPool};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub enum ActionType {
    Create,
    ReadIdOnly,
    ReadCompact,
    ReadDefault,
    ReadDetailed,
    Update,
    Delete,
}

#[allow(unused_variables)]
#[async_trait]
pub trait Authorizer: Send + Sync {
    async fn authorize_certificate(
        &self,
        certificate: &DbCertificate,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_classroom(
        &self,
        classroom: &DbClassroom,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()>;

    async fn authorize_club(&self, club: &DbClub, pool: &PgPool, action: ActionType) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_club_request(
        &self,
        club_request: &DbClubRequest,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()>;

    async fn authorize_elective_subject(
        &self,
        elective_subject: &DbElectiveSubject,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_elective_trade_offer(
        &self,
        elective_trade_offer: &DbElectiveTradeOffer,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_online_teaching_reports(
        &self,
        online_teaching_reports: &DbOnlineTeachingReports,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_organization(
        &self,
        organization: &DbOrganization,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_person(
        &self,
        person: &DbPerson,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_student(
        &self,
        student: &DbStudent,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()>;

    async fn authorize_subject(
        &self,
        subject: &DbSubject,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()>;

    async fn authorize_subject_group(
        &self,
        subject_group: &DbSubjectGroup,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Unimplemented
        Ok(())
    }

    async fn authorize_teacher(
        &self,
        teacher: &DbTeacher,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()>;

    fn clone_to_arc(&self) -> Arc<dyn Authorizer>;
}

pub fn authorize_read_only(action: ActionType, source: &str) -> Result<()> {
    match action {
        ActionType::ReadIdOnly
        | ActionType::ReadCompact
        | ActionType::ReadDefault
        | ActionType::ReadDetailed => Ok(()),
        ActionType::Create | ActionType::Update | ActionType::Delete => deny(source),
    }
}

pub fn authorize_default_read_only(action: ActionType, source: &str) -> Result<()> {
    match action {
        ActionType::ReadIdOnly | ActionType::ReadCompact | ActionType::ReadDefault => Ok(()),
        ActionType::Create | ActionType::ReadDetailed | ActionType::Update | ActionType::Delete => {
            deny(source)
        }
    }
}

pub fn deny(source: &str) -> Result<()> {
    Err(Error::InvalidPermission(
        "Insufficient permissions to perform this action".to_string(),
        source.to_string(),
    ))
}

pub async fn get_authorizer(
    pool: &PgPool,
    user: &User,
    source: String,
) -> Result<Box<dyn Authorizer>> {
    match user.role {
        _ if user.is_admin => Ok(Box::new(AdminRole)),

        UserRole::Student => Ok(Box::new(StudentRole::new(
            query!(
                "\
                SELECT s.id FROM students AS s JOIN users AS u ON u.id = s.user_id WHERE u.id = $1\
                ",
                user.id,
            )
            .fetch_one(pool)
            .await?
            .id,
            user.id,
            source,
        ))),

        UserRole::Teacher => Ok(Box::new(TeacherRole::new(
            query!(
                "\
                SELECT t.id FROM teachers AS t JOIN users AS u ON u.id = t.user_id WHERE u.id = $1\
                ",
                user.id,
            )
            .fetch_one(pool)
            .await?
            .id,
            user.id,
            source,
        ))),

        UserRole::Management => Ok(Box::new(ManagementRole::new(user.id, source))),

        _ => unimplemented!(),
    }
}

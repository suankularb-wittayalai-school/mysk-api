use crate::{
    // Don't remove the unused imports yet!!! Sorry for the squiggly lines!
    models::{
        classroom::db::DbClassroom, club::db::DbClub, club_request::db::DbClubRequest,
        contact::db::DbContact, elective_subject::db::DbElectiveSubject,
        elective_trade_offer::db::DbElectiveTradeOffer, organization::db::DbOrganization,
        person::db::DbPerson, student::db::DbStudent, subject::db::DbSubject,
        subject_group::db::DbSubjectGroup, teacher::db::DbTeacher, user::db::DbUser,
    },
    prelude::*,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub enum ActionType {
    Create,
    ReadIdOnly,
    ReadCompact,
    ReadDefault,
    ReadDetailed,
    Update,
    Delete,
}

#[async_trait]
pub trait Authorizer {
    async fn authorize_classroom(
        &self,
        classroom: &DbClassroom,
        pool: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()>;

    // async fn authorize_club(
    //     &self,
    //     club: &DbClub,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;

    // async fn authorize_club_request(
    //     &self,
    //     club_request: &DbClubRequest,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;

    async fn authorize_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()>;

    // async fn authorize_elective_subject(
    //     &self,
    //     elective_subject: &DbElectiveSubject,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;

    // async fn authorize_elective_trade_offer(
    //     &self,
    //     elective_trade_offer: &DbElectiveTradeOffer,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;

    // async fn authorize_organization(
    //     &self,
    //     organization: &DbOrganization,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;

    // async fn authorize_person(
    //     &self,
    //     person: &DbPerson,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;

    async fn authorize_student(
        &self,
        student: &DbStudent,
        pool: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()>;

    async fn authorize_subject(
        &self,
        subject: &DbSubject,
        pool: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()>;

    // async fn authorize_subject_group(
    //     &self,
    //     subject_group: &DbSubjectGroup,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;

    async fn authorize_teacher(
        &self,
        teacher: &DbTeacher,
        pool: &PgPool,
        action: ActionType,
        source: String,
    ) -> Result<()>;

    // async fn authorize_user(
    //     &self,
    //     user: &DbUser,
    //     pool: &PgPool,
    //     action: ActionType,
    //     source: String,
    // ) -> Result<()>;
}

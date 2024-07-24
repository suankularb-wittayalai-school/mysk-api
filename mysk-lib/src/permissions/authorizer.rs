use crate::{
    // Don't remove the unused imports yet!!! Sorry for the squiggly lines!
    models::{
        classroom::db::DbClassroom, club::db::DbClub, club_request::db::DbClubRequest,
        contact::db::DbContact, elective_subject::db::DbElectiveSubject,
        elective_trade_offer::db::DbElectiveTradeOffer, enums::UserRole,
        organization::db::DbOrganization, person::db::DbPerson, student::db::DbStudent,
        subject::db::DbSubject, subject_group::db::DbSubjectGroup, teacher::db::DbTeacher,
        user::User,
    },
    permissions::roles::{AdminRole, ManagementRole, StudentRole, TeacherRole},
    prelude::*,
};
use async_trait::async_trait;
use dyn_clone::DynClone;
use sqlx::{query, PgPool};

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
pub trait Authorizer
where
    Self: DynClone + Send + Sync + 'static,
{
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
}

pub async fn get_authorizer(
    pool: &PgPool,
    user: &User,
    source: String,
) -> Result<Box<dyn Authorizer>> {
    match user.role {
        _ if user.is_admin => Ok(Box::new(AdminRole)),
        UserRole::Student => Ok(Box::new(StudentRole {
            id: query!(
                "
                SELECT students.id FROM students
                INNER JOIN users ON users.id = students.user_id
                WHERE users.id = $1
                ",
                user.id,
            )
            .fetch_one(pool)
            .await?
            .id,
            user_id: user.id,
            source,
        })),
        UserRole::Teacher => Ok(Box::new(TeacherRole {
            id: query!(
                "
                SELECT teachers.id FROM teachers
                INNER JOIN users ON users.id = teachers.user_id
                WHERE users.id = $1
                ",
                user.id,
            )
            .fetch_one(pool)
            .await?
            .id,
            user_id: user.id,
            source,
        })),

        UserRole::Management => Ok(Box::new(ManagementRole {
            user_id: user.id,
            source,
        })),
        _ => unimplemented!(),
    }
}

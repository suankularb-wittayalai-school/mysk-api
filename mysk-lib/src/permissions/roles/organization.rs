use crate::{
    models::{
        classroom::db::DbClassroom, contact::db::DbContact, student::db::DbStudent,
        subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::{ActionType, Authorizable, authorize_read_only},
    prelude::*,
};
use sqlx::{PgConnection, query};
use uuid::Uuid;

const KORNOR_EMAIL: &str = "kornor@sk.ac.th";

#[derive(Clone, Debug)]
pub struct OrganizationRole {
    #[allow(dead_code)]
    id: Uuid,
    user_id: Uuid,
    source: String,
}

impl Authorizable for OrganizationRole {
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
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        // The Kornor organization is permitted to update the `club_quota` field of a student,
        // this must be enforced at the service layer since the `authorize_student` method is called
        // for all student-related actions.
        if matches!(action, ActionType::Update) && self.is_kornor(&mut *conn).await? {
            return Ok(());
        }

        authorize_read_only(action, &self.source)
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
        authorize_read_only(action, &self.source)
    }
}

impl OrganizationRole {
    pub fn new(id: Uuid, user_id: Uuid, source: String) -> Self {
        Self {
            id,
            user_id,
            source,
        }
    }

    pub async fn is_kornor(&self, conn: &mut PgConnection) -> Result<bool> {
        Ok(query!(
            "SELECT EXISTS( \
                SELECT 1 FROM users \
                WHERE id = $1 AND email = $2
                AND role = 'organization'::user_role
            ) AS exists",
            self.user_id,
            KORNOR_EMAIL
        )
        .fetch_one(conn)
        .await?
        .exists
        .unwrap_or(false))
    }
}

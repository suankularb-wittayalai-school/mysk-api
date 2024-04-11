use self::{
    db::DbStudent,
    fetch_levels::{
        compact::CompactStudent, default::DefaultStudent, detailed::DetailedStudent,
        id_only::IdOnlyStudent,
    },
};
use crate::{models::top_level_variant::TopLevelVariant, prelude::*};
use sqlx::PgPool;
use uuid::Uuid;

pub mod db;
pub mod fetch_levels;

pub type Student =
    TopLevelVariant<DbStudent, IdOnlyStudent, CompactStudent, DefaultStudent, DetailedStudent>;

impl Student {
    pub async fn get_student_from_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Uuid>> {
        DbStudent::get_student_from_user_id(pool, user_id).await
    }
}

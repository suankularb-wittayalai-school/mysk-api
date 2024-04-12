use uuid::Uuid;

use self::{
    db::DbElectiveSubject,
    fetch_levels::{
        compact::CompactElectiveSubject, default::DefaultElectiveSubject,
        detailed::DetailedElectiveSubject, id_only::IdOnlyElectiveSubject,
    },
    request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
};
use crate::{
    common::requests::FetchLevel,
    models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery},
    prelude::*,
};

use super::traits::TopLevelFromTable;

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveSubject = TopLevelVariant<
    DbElectiveSubject,
    IdOnlyElectiveSubject,
    CompactElectiveSubject,
    DefaultElectiveSubject,
    DetailedElectiveSubject,
>;

impl TopLevelQuery<DbElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>
    for ElectiveSubject
{
}

impl ElectiveSubject {
    pub async fn get_by_session_code(
        pool: &sqlx::PgPool,
        session_code: i64,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let elective = DbElectiveSubject::get_by_session_code(pool, session_code).await?;

        match elective {
            Some(elective) => Ok(ElectiveSubject::from_table(
                pool,
                elective,
                fetch_level,
                descendant_fetch_level,
            )
            .await?),
            None => Err(Error::EntityNotFound(
                "Elective subject with given session code does not exist".to_string(),
                "ElectiveSubject::get_by_session_code".to_string(),
            )),
        }
    }

    /// # Get elective subject by id with student context
    /// This function is the extension of the `get_by_id` function
    ///
    /// Since an elective subject can be enrolled by students in different classrooms and taught in different sessions
    ///
    /// This function will return the elective subject object which is available for the student which will always be unique
    ///
    /// If the student is not eligible for the elective subject, it will return None
    ///
    /// If the student is not in any classroom, it will return an error
    pub async fn get_by_id_with_student_context(
        pool: &sqlx::PgPool,
        id: Uuid,
        student_id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let elective =
            DbElectiveSubject::get_by_id_with_student_context(pool, id, student_id).await?;
        match elective {
            Some(elective) => Ok(ElectiveSubject::from_table(
                pool,
                elective,
                fetch_level,
                descendant_fetch_level,
            )
            .await?),
            None => Err(Error::EntityNotFound(
                "Elective subject with given id does not exist or is not eligible for the student"
                    .to_string(),
                "ElectiveSubject::get_by_id".to_string(),
            )),
        }
    }
}

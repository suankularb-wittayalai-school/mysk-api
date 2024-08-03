use crate::{
    common::requests::{QueryParam, SqlSection},
    models::{enums::ContactType, traits::Queryable},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryableContact {
    pub ids: Option<Vec<Uuid>>,
    pub name: Option<String>,
    pub r#type: Option<ContactType>,
    pub value: Option<String>,
    pub classroom_ids: Option<Vec<Uuid>>,
    pub club_ids: Option<Vec<Uuid>>,
    pub student_ids: Option<Vec<Uuid>>,
    pub teacher_ids: Option<Vec<Uuid>>,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}

impl Queryable for QueryableContact {
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections = Vec::<SqlSection>::new();

        // WHERE id = ANY($1)
        if let Some(ids) = &self.ids {
            where_sections.push(SqlSection {
                sql: vec!["id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(ids.clone())],
            });
        }

        // WHERE name_th ILIKE $1 OR name_en ILIKE $1
        // https://stackoverflow.com/questions/77625753/using-ilike-in-rust-sqlx-with-push-bind
        if let Some(name) = &self.name {
            where_sections.push(SqlSection {
                sql: vec![
                    "(name_th ILIKE ".to_string(),
                    " OR name_en ILIKE ".to_string(),
                    ")".to_string(),
                ],
                params: vec![
                    QueryParam::String(name.clone()),
                    QueryParam::String(name.clone()),
                ],
            });
        }

        // WHERE type = $1
        if let Some(r#type) = &self.r#type {
            where_sections.push(SqlSection {
                sql: vec!["type = ".to_string()],
                params: vec![QueryParam::ContactType(*r#type)],
            });
        }

        // WHERE value ILIKE $1
        if let Some(value) = &self.value {
            where_sections.push(SqlSection {
                sql: vec!["value ILIKE ".to_string()],
                params: vec![QueryParam::String(value.clone())],
            });
        }

        // WHERE id IN (SELECT contact_id FROM classroom_contacts WHERE classroom_id = ANY($1))
        if let Some(classroom_ids) = &self.classroom_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT contact_id FROM classroom_contacts WHERE classroom_id = ANY("
                        .to_string(),
                    "))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(classroom_ids.clone())],
            });
        }

        // WHERE id IN (SELECT contact_id FROM club_contacts WHERE club_id = ANY($1))
        if let Some(club_ids) = &self.club_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT contact_id FROM club_contacts WHERE club_id = ANY(".to_string(),
                    "))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(club_ids.clone())],
            });
        }

        // WHERE id IN (SELECT contact_id FROM person_contacts WHERE person_id = ANY(SELECT
        // person_id FROM students WHERE id = ANY($1)))
        if let Some(student_ids) = &self.student_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT contact_id FROM person_contacts WHERE person_id = ANY(SELECT person_id FROM students WHERE id = ANY("
                        .to_string(),
                    ")))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(student_ids.clone())],
            });
        }

        // WHERE id IN (SELECT contact_id FROM person_contacts WHERE person_id = ANY(SELECT
        // person_id FROM teachers WHERE id = ANY($1)))
        if let Some(teacher_ids) = &self.teacher_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT contact_id FROM person_contacts WHERE person_id = ANY(SELECT person_id FROM teachers WHERE id = ANY("
                        .to_string(),
                    ")))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(teacher_ids.clone())],
            });
        }

        // WHERE include_students = $1
        if let Some(include_students) = &self.include_students {
            where_sections.push(SqlSection {
                sql: vec!["include_students ILIKE ".to_string()],
                params: vec![QueryParam::Bool(*include_students)],
            });
        }

        // WHERE include_teachers = $1
        if let Some(include_teachers) = &self.include_teachers {
            where_sections.push(SqlSection {
                sql: vec!["include_teachers ILIKE ".to_string()],
                params: vec![QueryParam::Bool(*include_teachers)],
            });
        }

        // WHERE include_parents = $1
        if let Some(include_parents) = &self.include_parents {
            where_sections.push(SqlSection {
                sql: vec!["include_parents ILIKE ".to_string()],
                params: vec![QueryParam::Bool(*include_parents)],
            });
        }

        where_sections
    }
}

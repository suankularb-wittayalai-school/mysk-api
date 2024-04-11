use crate::models::common::{
    requests::{QueryParam, SqlSection},
    traits::Queryable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableElectiveSubject {
    pub ids: Option<Vec<Uuid>>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub teacher_ids: Option<Vec<Uuid>>,
    pub co_teacher_ids: Option<Vec<Uuid>>,
    pub subject_group_id: Option<Vec<i64>>,
    pub credit: Option<f64>,
    pub is_full: Option<bool>,
    pub applicable_classroom_ids: Option<Vec<Uuid>>,
    pub room: Option<String>,
    pub student_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableElectiveSubject {
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(ids) = &self.ids {
            // WHERE id = ANY($1)
            where_sections.push(SqlSection {
                sql: vec!["id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(ids.clone())],
            });
        }

        if let Some(name) = &self.name {
            // WHERE name_th ILIKE $1 OR name_en ILIKE $1
            // https://stackoverflow.com/questions/77625753/using-ilike-in-rust-sqlx-with-push-bind
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

        if let Some(code) = &self.code {
            // WHERE code_th ILIKE $1 OR code_en ILIKE $1
            where_sections.push(SqlSection {
                sql: vec![
                    "(code_th ILIKE ".to_string(),
                    " OR code_en ILIKE ".to_string(),
                    ")".to_string(),
                ],
                params: vec![
                    QueryParam::String(code.clone()),
                    QueryParam::String(code.clone()),
                ],
            });
        }

        if let Some(description) = &self.description {
            // WHERE description_th ILIKE $1 OR description_en ILIKE $1
            where_sections.push(SqlSection {
                sql: vec![
                    "(description_th ILIKE ".to_string(),
                    " OR description_en ILIKE ".to_string(),
                    ")".to_string(),
                ],
                params: vec![
                    QueryParam::String(description.clone()),
                    QueryParam::String(description.clone()),
                ],
            });
        }

        if let Some(teacher_ids) = &self.teacher_ids {
            // WHERE subject_id IN (SELECT subject_id FROM subject_teachers WHERE teacher_id IN ANY($1))
            where_sections.push(SqlSection {
                sql: vec![
                    "subject_id IN (SELECT subject_id FROM subject_teachers WHERE teacher_id = ANY("
                        .to_string(),
                    "))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(teacher_ids.clone())],
            });
        }

        if let Some(co_teacher_ids) = &self.co_teacher_ids {
            // WHERE subject_id IN (SELECT subject_id FROM subject_co_teachers WHERE teacher_id IN ANY($1))
            where_sections.push(SqlSection {
                sql: vec![
                    "subject_id IN (SELECT subject_id FROM subject_co_teachers WHERE teacher_id = ANY("
                        .to_string(),
                    "))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(co_teacher_ids.clone())],
            });
        }

        if let Some(subject_group_id) = &self.subject_group_id {
            // WHERE subject_group_id IN ANY($1)
            where_sections.push(SqlSection {
                sql: vec!["subject_group_id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayInt(subject_group_id.clone())],
            });
        }

        if let Some(credit) = &self.credit {
            // WHERE credit = $1
            where_sections.push(SqlSection {
                sql: vec!["credit = ".to_string()],
                params: vec![QueryParam::Float(*credit)],
            });
        }

        if let Some(is_full) = &self.is_full {
            // WHERE cap_size = class_size
            // WHERE class_size < cap_size
            if *is_full {
                where_sections.push(SqlSection {
                    sql: vec!["cap_size = class_size".to_string()],
                    params: vec![],
                });
            } else {
                where_sections.push(SqlSection {
                    sql: vec!["class_size < cap_size".to_string()],
                    params: vec![],
                });
            }
        }

        if let Some(applicable_classroom_ids) = &self.applicable_classroom_ids {
            // WHERE id IN (SELECT elective_subject_id FROM elective_subject_classrooms WHERE classroom_id IN ANY($1))
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT elective_subject_id FROM".to_string(),
                    " elective_subject_classrooms WHERE classroom_id = ANY())".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(applicable_classroom_ids.clone())],
            });
        }

        if let Some(room) = &self.room {
            // WHERE room ILIKE $1
            where_sections.push(SqlSection {
                sql: vec!["room ILIKE ".to_string()],
                params: vec![QueryParam::String(room.clone())],
            });
        }

        if let Some(student_ids) = &self.student_ids {
            // WHERE id IN (SELECT elective_subject_id FROM student_elective_subjects WHERE student_id IN ANY($1))
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT elective_subject_id FROM student_elective_subjects".to_string(),
                    " WHERE student_id = ANY())".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(student_ids.clone())],
            });
        }

        where_sections
    }
}

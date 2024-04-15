use crate::{
    common::requests::{QueryParam, SqlSection},
    helpers::date::{get_current_academic_year, get_current_semester},
    models::traits::Queryable,
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
    pub as_student_id: Option<Uuid>,
}

impl Queryable for QueryableElectiveSubject {
    #[allow(clippy::too_many_lines)]
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

        // WHERE code_th ILIKE $1 OR code_en ILIKE $1
        if let Some(code) = &self.code {
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

        // WHERE description_th ILIKE $1 OR description_en ILIKE $1
        if let Some(description) = &self.description {
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

        // WHERE subject_id IN (SELECT subject_id FROM subject_teachers WHERE teacher_id IN
        // ANY($1))
        if let Some(teacher_ids) = &self.teacher_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "subject_id IN (SELECT subject_id FROM subject_teachers WHERE teacher_id = ANY("
                        .to_string(),
                    "))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(teacher_ids.clone())],
            });
        }

        // WHERE subject_id IN (SELECT subject_id FROM subject_co_teachers WHERE teacher_id IN
        // ANY($1))
        if let Some(co_teacher_ids) = &self.co_teacher_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    concat!(
                        "subject_id IN (SELECT subject_id FROM subject_co_teachers WHERE",
                        " teacher_id = ANY(",
                    )
                    .to_string(),
                    "))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(co_teacher_ids.clone())],
            });
        }

        // WHERE subject_group_id IN ANY($1)
        if let Some(subject_group_id) = &self.subject_group_id {
            where_sections.push(SqlSection {
                sql: vec!["subject_group_id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayInt(subject_group_id.clone())],
            });
        }

        // WHERE credit = $1
        if let Some(credit) = &self.credit {
            where_sections.push(SqlSection {
                sql: vec!["credit = ".to_string()],
                params: vec![QueryParam::Float(*credit)],
            });
        }

        // WHERE cap_size = class_size
        // WHERE class_size < cap_size
        if let Some(is_full) = &self.is_full {
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

        // WHERE id IN (SELECT elective_subject_id FROM elective_subject_classrooms WHERE
        // classroom_id IN ANY($1))
        if let Some(applicable_classroom_ids) = &self.applicable_classroom_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT elective_subject_id FROM".to_string(),
                    " elective_subject_classrooms WHERE classroom_id = ANY())".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(applicable_classroom_ids.clone())],
            });
        }

        // WHERE room ILIKE $1
        if let Some(room) = &self.room {
            where_sections.push(SqlSection {
                sql: vec!["room ILIKE ".to_string()],
                params: vec![QueryParam::String(room.clone())],
            });
        }

        // WHERE id IN (SELECT elective_subject_id FROM student_elective_subjects WHERE student_id
        // IN ANY($1))
        if let Some(student_ids) = &self.student_ids {
            // WHERE session_code IN (SELECT session_code FROM elective_subject_classrooms AS esc
            // INNER JOIN student_elective_subjects AS ses ON
            // esc.elective_subject_id = ses.elective_subject_id INNER JOIN classroom_students AS
            // cs ON cs.student_id = ses.student_id WHERE cs.classroom_id = esc.classroom_id AND
            // ses.student_id = ANY($1) AND year = $2 AND semester = $3)
            where_sections.push(SqlSection {
                sql: vec![
                    concat!(
                        "session_code IN (SELECT session_code FROM elective_subject_classrooms AS",
                        " esc INNER JOIN student_elective_subjects AS ses ON",
                        " esc.elective_subject_id = ses.elective_subject_id INNER JOIN",
                        " classroom_students AS cs ON cs.student_id = ses.student_id WHERE",
                        " cs.classroom_id = esc.classroom_id AND ses.student_id = ANY(",
                    )
                    .to_string(),
                    ") AND year = ".to_string(),
                    " AND semester = ".to_string(),
                    ")".to_string(),
                ],
                params: vec![
                    QueryParam::ArrayUuid(student_ids.clone()),
                    QueryParam::Int(get_current_academic_year(None)),
                    QueryParam::Int(get_current_semester(None)),
                ],
            });
        }

        if let Some(as_student_id) = &self.as_student_id {
            // WHERE id IN (SELECT elective_subject_id FROM elective_subject_classrooms WHERE
            // classroom_id IN (SELECT classroom_id FROM classroom_students INNER JOIN classrooms
            // ON classrooms.id = classroom_students.classroom_id WHERE student_id = $1 AND
            // year = $2))
            where_sections.push(SqlSection {
                sql: vec![
                    concat!(
                        "session_code IN (SELECT session_code FROM elective_subject_classrooms",
                        " WHERE classroom_id IN (SELECT classroom_id FROM classroom_students",
                        " INNER JOIN classrooms ON classrooms.id = classroom_students.classroom_id",
                        " WHERE student_id = ",
                    )
                    .to_string(),
                    " AND year = ".to_string(),
                    "))".to_string(),
                ],
                params: vec![
                    QueryParam::Uuid(*as_student_id),
                    QueryParam::Int(get_current_academic_year(None)),
                ],
            });
        }

        where_sections
    }
}

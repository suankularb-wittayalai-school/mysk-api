use crate::query::{QueryParam, Queryable, SqlWhereClause};
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
    pub year: Option<i64>,
    pub semester: Option<i64>,
    pub applicable_classroom_ids: Option<Vec<Uuid>>,
    pub room: Option<String>,
    pub student_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableElectiveSubject {
    #[allow(clippy::too_many_lines)]
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new_empty();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.name, |mut f, name| {
            f.push_sql("(name_th ILIKE ('%' || ")
                .push_param(QueryParam::String(name))
                .push_sql(" || '%') OR name_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.code, |mut f, code| {
            f.push_sql("(code_th ILIKE ('%' || ")
                .push_param(QueryParam::String(code))
                .push_sql(" || '%') OR code_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.description, |mut f, description| {
            f.push_sql("(description_th ILIKE ('%' || ")
                .push_param(QueryParam::String(description))
                .push_sql(" || '%') OR description_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.teacher_ids, |mut f, teacher_ids| {
            f.push_sql(
                "subject_id IN (SELECT subject_id FROM subject_teachers WHERE teacher_id = ANY(",
            )
            .push_param(QueryParam::ArrayUuid(teacher_ids))
            .push_sql("))");

            f
        })
        .push_if_some(self.co_teacher_ids, |mut f, co_teacher_ids| {
            f.push_sql(
                "subject_id IN (SELECT subject_id FROM subject_co_teachers WHERE teacher_id = ANY(",
            )
            .push_param(QueryParam::ArrayUuid(co_teacher_ids))
            .push_sql("))");

            f
        })
        .push_if_some(self.subject_group_id, |mut f, subject_group_id| {
            f.push_sql("subject_group_id = ANY(")
                .push_param(QueryParam::ArrayInt(subject_group_id))
                .push_sql(")");

            f
        })
        .push_if_some(self.credit, |mut f, credit| {
            f.push_sql("credit = ")
                .push_param(QueryParam::Float(credit));

            f
        })
        .push_if_some(self.is_full, |mut f, is_full| {
            if !is_full {
                return f;
            }

            f.push_sql("cap_size = class_size")
                .push_sep()
                .push_sql("class_size < cap_size");

            f
        })
        .push_if_some(self.year, |mut f, year| {
            f.push_sql("year = ").push_param(QueryParam::Int(year));

            f
        })
        .push_if_some(self.semester, |mut f, semester| {
            f.push_sql("semester = ")
                .push_param(QueryParam::Int(semester));

            f
        })
        .push_if_some(
            self.applicable_classroom_ids,
            |mut f, applicable_classroom_ids| {
                f.push_sql(
                    "
                    id IN (SELECT elective_subject_session_id FROM \
                    elective_subject_session_classrooms WHERE classroom_id = ANY(
                    ",
                )
                .push_param(QueryParam::ArrayUuid(applicable_classroom_ids))
                .push_sql("))");

                f
            },
        )
        .push_if_some(self.room, |mut f, room| {
            f.push_sql("room ILIKE ('%' || ")
                .push_param(QueryParam::String(room))
                .push_sql(" || '%')");

            f
        })
        .push_if_some(self.student_ids, |mut f, student_ids| {
            f.push_sql(
                "
                id IN (SELECT elective_subject_session_id FROM \
                elective_subject_session_enrolled_students WHERE student_id = ANY(
                ",
            )
            .push_param(QueryParam::ArrayUuid(student_ids))
            .push_sql("))");

            f
        });

        wc
    }
}

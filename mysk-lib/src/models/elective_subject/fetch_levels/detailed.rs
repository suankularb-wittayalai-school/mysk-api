use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        classroom::Classroom, elective_subject::db::DbElectiveSubject, enums::SubjectType,
        student::Student, subject::db::DbSubject, subject_group::SubjectGroup, teacher::Teacher,
        traits::FetchVariant,
    },
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedElectiveSubject {
    pub id: Uuid,
    pub name: MultiLangString,
    pub short_name: MultiLangString,
    pub code: MultiLangString,
    pub description: Option<FlexibleMultiLangString>,
    pub teachers: Vec<Teacher>,
    pub co_teachers: Vec<Teacher>,
    pub subject_group: SubjectGroup,
    pub syllabus: Option<String>,
    pub credit: f64,
    pub class_size: i64,
    pub cap_size: i64,
    pub room: String,
    pub r#type: SubjectType,
    pub year: Option<i64>,
    pub semester: Option<i64>,
    pub applicable_classrooms: Vec<Classroom>,
    pub students: Vec<Student>,
    pub randomized_students: Vec<Student>,
    pub session_code: String,
    pub requirements: Vec<MultiLangString>,
}

impl FetchVariant for DetailedElectiveSubject {
    type Relation = DbElectiveSubject;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let mut conn = pool.acquire().await?;
        let subject_group = SubjectGroup::get_by_id(
            pool,
            relation.subject_group_id,
            FetchLevel::IdOnly,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;

        let teacher_ids =
            DbSubject::get_subject_teachers(&mut conn, relation.subject_id, None).await?;
        let co_teacher_ids =
            DbSubject::get_subject_co_teachers(&mut conn, relation.subject_id, None).await?;
        let applicable_classroom_ids = relation
            .get_subject_applicable_classrooms(&mut conn)
            .await?;
        let student_ids = relation.get_enrolled_students(&mut conn).await?;
        let randomized_students_ids = relation.get_randomized_students(&mut conn).await?;

        let description = match (relation.description_th, relation.description_en) {
            (Some(description_th), Some(description_en)) => Some(FlexibleMultiLangString {
                th: Some(description_th),
                en: Some(description_en),
            }),
            (Some(description_th), None) => Some(FlexibleMultiLangString {
                th: Some(description_th),
                en: None,
            }),
            (None, Some(description_en)) => Some(FlexibleMultiLangString {
                th: None,
                en: Some(description_en),
            }),
            (None, None) => None,
        };

        Ok(Self {
            id: relation.id,
            name: MultiLangString::new(relation.name_th, Some(relation.name_en)),
            code: MultiLangString::new(relation.code_th, Some(relation.code_en)),
            short_name: MultiLangString::new(
                relation.short_name_th.unwrap_or_default(),
                relation.short_name_en,
            ),
            r#type: relation.r#type,
            credit: relation.credit,
            description,
            year: relation.year,
            semester: relation.semester,
            subject_group,
            syllabus: relation.syllabus,
            teachers: Teacher::get_by_ids(
                pool,
                &teacher_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            co_teachers: Teacher::get_by_ids(
                pool,
                &co_teacher_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            class_size: relation.class_size,
            cap_size: relation.cap_size,
            room: relation.room,
            applicable_classrooms: Classroom::get_by_ids(
                pool,
                &applicable_classroom_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            students: Student::get_by_ids(
                pool,
                &student_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            session_code: relation.session_code,
            requirements: DbSubject::get_requirements(&mut conn, relation.id).await?,
            randomized_students: Student::get_by_ids(
                pool,
                &randomized_students_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
        })
    }
}

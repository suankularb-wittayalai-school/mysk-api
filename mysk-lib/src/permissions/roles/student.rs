use crate::{
    models::{
        classroom::db::DbClassroom, club::db::DbClub, contact::db::DbContact,
        student::db::DbStudent, subject::db::DbSubject, teacher::db::DbTeacher,
    },
    permissions::authorizer::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct StudentRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub source: String,
}

#[async_trait]
impl Authorizer for StudentRole {
    async fn authorize_classroom(
        &self,
        _: &DbClassroom,
        _: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        match action {
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed => Ok(()),
            ActionType::Create | ActionType::Update | ActionType::Delete => {
                Err(Error::InvalidPermission(
                    "Insufficient permissions to perform this action".to_string(),
                    self.source.to_string(),
                ))
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn authorize_contact(
        &self,
        contact: &DbContact,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // TODO: Fix for create student contacts
        let owned = query!(
            "
            SELECT EXISTS (
                SELECT FROM contacts
                INNER JOIN person_contacts ON person_contacts.contact_id = contacts.id
                INNER JOIN people ON people.id = person_contacts.person_id
                INNER JOIN students ON students.person_id = people.id
                WHERE students.id = $1 AND contacts.id = $2
            )
            ",
            self.id,
            contact.id,
        )
        .fetch_one(pool)
        .await?
        .exists
        .unwrap_or(false);
        let mut same_class = false;
        let mut teacher_contact = false;
        let mut club_contact = false;
        let mut club_staff = false;

        if !owned {
            let self_class = DbStudent::get_student_classroom(pool, self.id, None).await?;
            let classroom_contact = if self_class.is_some() {
                query!(
                    "
                    SELECT contact_id FROM classroom_contacts
                    WHERE contact_id = $1 AND classroom_id = $2
                    ",
                    contact.id,
                    // Unwrap-safe because it is checked by the prior if statements
                    self_class.clone().unwrap().id,
                )
                .fetch_optional(pool)
                .await?
            } else {
                None
            };
            let student_contact = if classroom_contact.is_some() {
                None
            } else {
                query!(
                    "
                    SELECT students.id FROM students
                    JOIN person_contacts ON person_contacts.person_id = students.person_id
                    WHERE person_contacts.contact_id = $1
                    ",
                    contact.id,
                )
                .fetch_optional(pool)
                .await?
            };
            teacher_contact = if classroom_contact.is_some() || student_contact.is_some() {
                false
            } else {
                query!(
                    "
                    SELECT EXISTS (
                        SELECT FROM teachers
                        JOIN person_contacts ON person_contacts.person_id = teachers.person_id
                        JOIN contacts ON contacts.id = person_contacts.contact_id
                        WHERE person_contacts.contact_id = $1
                        AND (contacts.include_students = true OR contacts.include_students IS NULL)
                    )
                    ",
                    contact.id,
                )
                .fetch_one(pool)
                .await?
                .exists
                .unwrap_or(false)
            };
            let mut club_id = Uuid::nil();
            if classroom_contact.is_some() || student_contact.is_some() || teacher_contact {
                club_contact = false;
            } else {
                club_id = query!(
                    "SELECT club_id FROM club_contacts WHERE contact_id = $1",
                    contact.id,
                )
                .fetch_one(pool)
                .await?
                .club_id;
            };

            if classroom_contact.is_some() {
                same_class = true;
            }
            if let Some(student) = student_contact {
                let student_class =
                    DbStudent::get_student_classroom(pool, student.id, None).await?;
                same_class = self_class.is_some()
                    && student_class.is_some()
                    // Unwrap-safe because it is checked by the prior if statements
                    && self_class.unwrap().id == student_class.unwrap().id;
            }
            if club_contact {
                let club_staffs = DbClub::get_club_staffs(pool, club_id).await?;
                if club_staffs.iter().any(|staff_id| *staff_id == self.id) {
                    club_staff = true;
                }
            }
        }

        match action {
            // Owned
            _ if owned => Ok(()),
            // Classroom / Student (Same Class) / Teacher / Club (Read)
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed
                if !owned && (same_class || teacher_contact || club_contact) =>
            {
                Ok(())
            }
            // Club
            ActionType::Create | ActionType::Update | ActionType::Delete
                if !owned && club_staff =>
            {
                Ok(())
            }
            // Others
            ActionType::Create
            | ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed
            | ActionType::Update
            | ActionType::Delete => Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
                self.source.to_string(),
            )),
        }
    }

    async fn authorize_student(
        &self,
        student: &DbStudent,
        pool: &PgPool,
        action: ActionType,
    ) -> Result<()> {
        // Unwrap-safe because it is guaranteed prior by get_authorizer
        let owned = self.user_id == student.user_id.unwrap();
        let self_class = DbStudent::get_student_classroom(pool, self.id, None).await?;
        let student_class = DbStudent::get_student_classroom(pool, student.id, None).await?;
        let same_class = self_class.is_some()
            && student_class.is_some()
            // Unwrap-safe because it is checked by the prior if statements
            && self_class.unwrap().id == student_class.unwrap().id;

        match action {
            // Owned
            ActionType::ReadDetailed | ActionType::Update if owned => Ok(()),
            // Same Class
            ActionType::ReadDefault if same_class => Ok(()),
            // Others
            ActionType::ReadIdOnly | ActionType::ReadCompact => Ok(()),
            ActionType::Create
            | ActionType::ReadDefault
            | ActionType::ReadDetailed
            | ActionType::Update
            | ActionType::Delete => Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
                self.source.to_string(),
            )),
        }
    }

    async fn authorize_subject(&self, _: &DbSubject, _: &PgPool, action: ActionType) -> Result<()> {
        match action {
            ActionType::ReadIdOnly
            | ActionType::ReadCompact
            | ActionType::ReadDefault
            | ActionType::ReadDetailed => Ok(()),
            ActionType::Create | ActionType::Update | ActionType::Delete => {
                Err(Error::InvalidPermission(
                    "Insufficient permissions to perform this action".to_string(),
                    self.source.to_string(),
                ))
            }
        }
    }

    async fn authorize_teacher(&self, _: &DbTeacher, _: &PgPool, action: ActionType) -> Result<()> {
        match action {
            ActionType::ReadIdOnly | ActionType::ReadCompact | ActionType::ReadDefault => Ok(()),
            ActionType::Create
            | ActionType::ReadDetailed
            | ActionType::Update
            | ActionType::Delete => Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
                self.source.to_string(),
            )),
        }
    }
}

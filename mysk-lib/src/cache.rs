use futures::TryStreamExt as _;
use scc::TreeIndex;
use sqlx::{Error as SqlxError, PgPool, query_scalar};
use std::sync::Arc;
use uuid::Uuid;

/// The shared **global** cache of the application.
pub struct GlobalCache {
    cheer_staff_members: TreeIndex<Uuid, ()>,
    cheer_staff_teachers: TreeIndex<Uuid, ()>,
}

impl GlobalCache {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            cheer_staff_members: TreeIndex::new(),
            cheer_staff_teachers: TreeIndex::new(),
        })
    }

    pub async fn populate_cache(self: Arc<Self>, pool: &PgPool) -> Result<Arc<Self>, SqlxError> {
        query_scalar!("SELECT student_id FROM cheer_practice_staffs")
            .fetch(pool)
            .try_for_each(async |student_id| {
                self.cheer_staff_members
                    .insert_async(student_id, ())
                    .await
                    .ok();

                Ok(())
            })
            .await?;

        query_scalar!("SELECT teacher_id FROM cheer_practice_teachers")
            .fetch(pool)
            .try_for_each(async |teacher_id| {
                self.cheer_staff_teachers
                    .insert_async(teacher_id, ())
                    .await
                    .ok();

                Ok(())
            })
            .await?;

        Ok(self)
    }

    pub fn contains_cheer_staff(&self, student_id: Uuid) -> bool {
        self.cheer_staff_members.contains(&student_id)
    }

    pub fn contains_cheer_teacher(&self, teacher_id: Uuid) -> bool {
        self.cheer_staff_teachers.contains(&teacher_id)
    }
}

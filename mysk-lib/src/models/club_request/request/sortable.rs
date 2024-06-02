use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortableClubRequest {
    Id,
    ClubId,
    StudentId,
    Year,
    MembershipStatus,
    CreatedAt,
}

impl Default for SortableClubRequest {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableClubRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableClubRequest::Id => write!(f, "id"),
            SortableClubRequest::ClubId => write!(f, "club_id"),
            SortableClubRequest::StudentId => write!(f, "student_id"),
            SortableClubRequest::Year => write!(f, "year"),
            SortableClubRequest::MembershipStatus => write!(f, "membership_status"),
            SortableClubRequest::CreatedAt => write!(f, "created_at"),
        }
    }
}

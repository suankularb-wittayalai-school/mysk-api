use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableElectiveSubject {
    Id,
    CodeTh,
    CodeEn,
    NameTh,
    NameEn,
    SubjectGroupId,
    CapSize,
    ClassSize,
    SessionCode,
}

impl Default for SortableElectiveSubject {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableElectiveSubject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableElectiveSubject::Id => write!(f, "id"),
            SortableElectiveSubject::CodeTh => write!(f, "code_th"),
            SortableElectiveSubject::CodeEn => write!(f, "code_en"),
            SortableElectiveSubject::NameTh => write!(f, "name_th"),
            SortableElectiveSubject::NameEn => write!(f, "name_en"),
            SortableElectiveSubject::SubjectGroupId => {
                write!(f, "subject_group_id")
            }
            SortableElectiveSubject::CapSize => write!(f, "cap_size"),
            SortableElectiveSubject::ClassSize => write!(f, "class_size"),
            SortableElectiveSubject::SessionCode => write!(f, "session_code"),
        }
    }
}

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SortableCertificate {
    #[default]
    Id,
    Year,
    ReceivingOrderNumber,
    StudentId,
    CertificateType,
    SeatCode,
}

impl Display for SortableCertificate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableCertificate::Id => write!(f, "id"),
            SortableCertificate::Year => write!(f, "year"),
            SortableCertificate::ReceivingOrderNumber => write!(f, "receiving_order_number"),
            SortableCertificate::StudentId => write!(f, "student_id"),
            SortableCertificate::CertificateType => write!(f, "certificate_type"),
            SortableCertificate::SeatCode => write!(f, "seat_code"),
        }
    }
}
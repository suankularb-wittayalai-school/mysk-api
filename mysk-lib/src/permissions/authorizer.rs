use crate::{
    models::{
        certificate::db::DbCertificate, cheer_practice_attendance::db::DbCheerPracticeAttendance,
        cheer_practice_period::db::DbCheerPracticePeriod, classroom::db::DbClassroom,
        club::db::DbClub, club_request::db::DbClubRequest, contact::db::DbContact,
        elective_subject::db::DbElectiveSubject, elective_trade_offer::db::DbElectiveTradeOffer,
        enums::UserRole, online_teaching_reports::db::DbOnlineTeachingReports,
        organization::db::DbOrganization, person::db::DbPerson, student::db::DbStudent,
        subject::db::DbSubject, subject_group::db::DbSubjectGroup, teacher::db::DbTeacher,
        user::User,
    },
    permissions::roles::{AdminRole, ManagementRole, StudentRole, TeacherRole},
    prelude::*,
};
use futures::future;
use sqlx::{PgConnection, query};

#[derive(Clone, Copy)]
pub enum ActionType {
    Create,
    ReadIdOnly,
    ReadCompact,
    ReadDefault,
    ReadDetailed,
    Update,
    Delete,
}

#[allow(unused_variables)]
pub trait Authorizable {
    fn authorize_certificate(
        &self,
        certificate: &DbCertificate,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_cheer_practice_attendance(
        &self,
        cheer_practice_period: &DbCheerPracticeAttendance,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_cheer_practice_period(
        &self,
        cheer_practice_period: &DbCheerPracticePeriod,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_classroom(
        &self,
        classroom: &DbClassroom,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>>;

    fn authorize_club(
        &self,
        club: &DbClub,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_club_request(
        &self,
        club_request: &DbClubRequest,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_contact(
        &self,
        contact: &DbContact,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>>;

    fn authorize_elective_subject(
        &self,
        elective_subject: &DbElectiveSubject,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_elective_trade_offer(
        &self,
        elective_trade_offer: &DbElectiveTradeOffer,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_online_teaching_reports(
        &self,
        online_teaching_reports: &DbOnlineTeachingReports,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_organization(
        &self,
        organization: &DbOrganization,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_person(
        &self,
        person: &DbPerson,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_student(
        &self,
        student: &DbStudent,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>>;

    fn authorize_subject(
        &self,
        subject: &DbSubject,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>>;

    fn authorize_subject_group(
        &self,
        subject_group: &DbSubjectGroup,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>> {
        // TODO: Unimplemented
        future::ok(())
    }

    fn authorize_teacher(
        &self,
        teacher: &DbTeacher,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> impl Future<Output = Result<()>>;
}

#[derive(Debug)]
pub enum Authorizer {
    Admin(AdminRole),
    Management(ManagementRole),
    Student(StudentRole),
    Teacher(TeacherRole),
}

impl Authorizer {
    pub async fn new(conn: &mut PgConnection, user: &User, source: String) -> Result<Self> {
        match user.role {
            _ if user.is_admin => Ok(Self::Admin(AdminRole)),
            UserRole::Student => Ok(Self::Student(StudentRole::new(
                query!(
                    "\
                    SELECT s.id \
                    FROM students AS s JOIN users AS u ON u.id = s.user_id \
                    WHERE u.id = $1\
                    ",
                    user.id,
                )
                .fetch_one(conn)
                .await?
                .id,
                user.id,
                source,
            ))),
            UserRole::Teacher => Ok(Self::Teacher(TeacherRole::new(
                query!(
                    "\
                    SELECT t.id \
                    FROM teachers AS t JOIN users AS u ON u.id = t.user_id \
                    WHERE u.id = $1\
                    ",
                    user.id,
                )
                .fetch_one(conn)
                .await?
                .id,
                user.id,
                source,
            ))),
            UserRole::Management => Ok(Self::Management(ManagementRole::new(user.id, source))),
            _ => unimplemented!(),
        }
    }
}

impl Authorizable for Authorizer {
    async fn authorize_classroom(
        &self,
        classroom: &DbClassroom,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        match self {
            Self::Admin(a) => a.authorize_classroom(classroom, conn, action).await,
            Self::Management(a) => a.authorize_classroom(classroom, conn, action).await,
            Self::Student(a) => a.authorize_classroom(classroom, conn, action).await,
            Self::Teacher(a) => a.authorize_classroom(classroom, conn, action).await,
        }
    }

    async fn authorize_contact(
        &self,
        contact: &DbContact,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        match self {
            Self::Admin(a) => a.authorize_contact(contact, conn, action).await,
            Self::Management(a) => a.authorize_contact(contact, conn, action).await,
            Self::Student(a) => a.authorize_contact(contact, conn, action).await,
            Self::Teacher(a) => a.authorize_contact(contact, conn, action).await,
        }
    }

    async fn authorize_student(
        &self,
        student: &DbStudent,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        match self {
            Self::Admin(a) => a.authorize_student(student, conn, action).await,
            Self::Management(a) => a.authorize_student(student, conn, action).await,
            Self::Student(a) => a.authorize_student(student, conn, action).await,
            Self::Teacher(a) => a.authorize_student(student, conn, action).await,
        }
    }

    async fn authorize_subject(
        &self,
        subject: &DbSubject,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        match self {
            Self::Admin(a) => a.authorize_subject(subject, conn, action).await,
            Self::Management(a) => a.authorize_subject(subject, conn, action).await,
            Self::Student(a) => a.authorize_subject(subject, conn, action).await,
            Self::Teacher(a) => a.authorize_subject(subject, conn, action).await,
        }
    }

    async fn authorize_teacher(
        &self,
        teacher: &DbTeacher,
        conn: &mut PgConnection,
        action: ActionType,
    ) -> Result<()> {
        match self {
            Self::Admin(a) => a.authorize_teacher(teacher, conn, action).await,
            Self::Management(a) => a.authorize_teacher(teacher, conn, action).await,
            Self::Student(a) => a.authorize_teacher(teacher, conn, action).await,
            Self::Teacher(a) => a.authorize_teacher(teacher, conn, action).await,
        }
    }
}

pub fn authorize_read_only(action: ActionType, source: &str) -> Result<()> {
    match action {
        ActionType::ReadIdOnly
        | ActionType::ReadCompact
        | ActionType::ReadDefault
        | ActionType::ReadDetailed => Ok(()),
        ActionType::Create | ActionType::Update | ActionType::Delete => deny(source),
    }
}

pub fn authorize_default_read_only(action: ActionType, source: &str) -> Result<()> {
    match action {
        ActionType::ReadIdOnly | ActionType::ReadCompact | ActionType::ReadDefault => Ok(()),
        ActionType::Create | ActionType::ReadDetailed | ActionType::Update | ActionType::Delete => {
            deny(source)
        }
    }
}

pub fn deny(source: &str) -> Result<()> {
    Err(Error::InvalidPermission(
        "Insufficient permissions to perform this action".to_string(),
        source.to_string(),
    ))
}

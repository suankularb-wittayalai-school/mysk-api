use crate::{
    models::{enums::UserRole, user::User},
    permissions::Authorizer,
};

mod student;
mod teacher;

pub use student::StudentRole;
pub use teacher::TeacherRole;

pub fn get_authorizer(user: &User) -> Box<dyn Authorizer> {
    match user.role {
        UserRole::Student | UserRole::Management => Box::new(StudentRole { id: user.id }),
        UserRole::Teacher => Box::new(TeacherRole { id: user.id }),
        _ => unimplemented!(),
    }
}

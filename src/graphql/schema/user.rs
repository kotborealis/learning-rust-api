use juniper::{FieldError, FieldResult};

use crate::db::models::user::{NewUser, User};
use crate::graphql::Context;

use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Clone, GraphQLObject)]
pub struct UserObject {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

impl From<&User> for UserObject {
    fn from(user: &User) -> Self {
        UserObject {
            id: user.id,
            username: user.username.clone(),
            password: user.password.clone(),
            first_name: user.first_name.clone(),
        }
    }
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        UserObject {
            id: user.id,
            username: user.username.clone(),
            password: user.password.clone(),
            first_name: user.first_name.clone(),
        }
    }
}

#[derive(Debug, GraphQLInputObject)]
pub struct NewUserInput {
    pub username: String,
    pub password: String,
    pub first_name: String,
}

impl Context {
    pub fn get_user(&self, id: i32) -> FieldResult<UserObject> {
        let user = User::get_by_id(&self.conn, id)?;
        Ok(user.into())
    }

    pub fn all_users(&self) -> FieldResult<Vec<UserObject>> {
        let users = User::get_all(&self.conn)?;
        let users = users
            .iter()
            .map(|user| UserObject::from(user.clone()))
            .collect();
        Ok(users)
    }

    pub fn add_user(&self, user: NewUserInput) -> FieldResult<UserObject> {
        let new_user = NewUser {
            username: user.username,
            password: user.password,
            first_name: user.first_name,
        };

        let new_user = User::create(&self.conn, &new_user)
            .map_err(|_| FieldError::from("Cannot create user"))?;

        Ok(new_user.into())
    }
}

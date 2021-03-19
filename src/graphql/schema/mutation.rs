use juniper::FieldResult;

use crate::graphql::schema::{NewUserInput, UserObject};
use crate::graphql::Context;

#[derive(Debug)]
pub struct Mutation {}

juniper::graphql_object!(Mutation: Context |&self| {
    field add_user(&executor, input: NewUserInput) -> FieldResult<UserObject> {
        executor.context().add_user(input)
    }
});

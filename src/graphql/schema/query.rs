use juniper::FieldResult;

use crate::graphql::schema::UserObject;
use crate::graphql::Context;

#[derive(Debug)]
pub struct Query {}

juniper::graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field user(&executor, id: i32) -> FieldResult<UserObject> {
        executor.context().get_user(id)
    }

    field all_users(&executor) -> FieldResult<Vec<UserObject>> {
        executor.context().all_users()
    }
});

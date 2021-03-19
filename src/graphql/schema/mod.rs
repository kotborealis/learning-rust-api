mod mutation;
mod query;
mod user;

use mutation::*;
use query::*;
pub use user::*;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

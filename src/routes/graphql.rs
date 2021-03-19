use crate::db::Conn;
use crate::graphql::{Context, Schema};
use juniper_rocket;
use rocket::response::content;
use rocket::State;

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
pub fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    db: Conn,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { conn: db })
}

#[rocket::post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    db: Conn,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { conn: db })
}

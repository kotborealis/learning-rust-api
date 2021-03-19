use super::db::Conn as DbConn;
use super::db::models::users::{User, NewUserInput};
use diesel::result::Error;
use rocket::http::Status;
use rocket::State;
use rocket::response::{status, content};
use rocket_contrib::json::Json;
use juniper_rocket;

#[get("/")]
pub fn get_all(conn: DbConn) -> Result<status::Accepted<Json<Vec<User>>>, Status> {
    User::get_all(&conn)
        .map(|users| status::Accepted(Some(Json(users))))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn find_user(
    conn: DbConn,
    id: i32,
) -> Result<status::Accepted<Json<Vec<User>>>, Status> {
    User::get_by_id(&conn, id)
        .map(|users| status::Accepted(Some(Json(users))))
        .map_err(error_status)
}

#[post("/", data = "<new_user_input>")]
pub fn new_user(
    conn: DbConn,
    new_user_input: Json<NewUserInput>,
) -> Result<status::Created<Json<User>>, Status> {
    let new_user_input = new_user_input.into_inner();

    User::create(&conn, &new_user_input)
        .map(|new_user| status::Created("Created user".to_string(), Some(Json(new_user))))
        .map_err(error_status)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphiql")
}
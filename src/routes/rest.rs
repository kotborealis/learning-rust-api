use crate::db::models::user::{NewUser, User};
use crate::db::Conn;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_all(conn: Conn) -> Result<status::Accepted<Json<Vec<User>>>, Status> {
    User::get_all(&conn)
        .map(|users| status::Accepted(Some(Json(users))))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn find_user(conn: Conn, id: i32) -> Result<status::Accepted<Json<User>>, Status> {
    User::get_by_id(&conn, id)
        .map(|user| status::Accepted(Some(Json(user))))
        .map_err(error_status)
}

#[post("/", data = "<new_user>")]
pub fn new_user(
    conn: Conn,
    new_user: Json<NewUser>,
) -> Result<status::Created<Json<User>>, Status> {
    let new_user = new_user.into_inner();

    User::create(&conn, &new_user)
        .map(|new_user| status::Created("Created user".to_string(), Some(Json(new_user))))
        .map_err(error_status)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

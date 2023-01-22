mod db;
mod entry;
mod schema;

use crate::db::DbConn;
use crate::entry::Entry;

use diesel::result::Error as DieselError;
use entry::{Entries, EntryRequest};
use rocket::{
    error, get,
    http::Status,
    post, routes,
    serde::{json::Json, uuid::Uuid},
};

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .manage(db::establish_connection())
        .mount("/", routes![create, list, resolve])
        .launch()
        .await
        .unwrap();
}

#[post("/create", data = "<entry>")]
pub async fn create(entry: Json<EntryRequest>, connection: DbConn) -> Status { // return UUID
    match db::create(connection, Entry::new(entry.0)) {
        Ok(_) => Status::Ok,
        Err(err) => {
            error!("{}:{} db::create failed: {err}", line!(), file!());
            Status::InternalServerError
        }
    }
}

#[get("/list")]
async fn list(connection: DbConn) -> Result<Entries, Status> {
    match db::list(connection) {
        Ok(entries) => Ok(entries),
        Err(err) => {
            error!("{}:{} db::list failed: {err}", line!(), file!());
            Err(Status::InternalServerError)
        }
    }
}

#[post("/resolve", data = "<uuid>")]
pub async fn resolve(uuid: Json<Uuid>, connection: DbConn) -> Result<(), Status> {
    match db::resolve(connection, uuid.0) {
        Ok(_) => Ok(()),
        Err(err) if err.eq(&DieselError::NotFound) => Err(Status::NotFound),
        Err(err) => {
            error!("{}:{} db::resolve failed: {err}", line!(), file!());
            Err(Status::InternalServerError)
        }
    }
}

#[post("/delete", data = "<uuid>")]
pub async fn delete(uuid: Json<Uuid>, connection: DbConn) -> Result<(), Status> {
    match db::delete(connection, uuid.0) {
        Ok(_) => Ok(()),
        Err(err) if err.eq(&DieselError::NotFound) => Err(Status::NotFound),
        Err(err) => {
            error!("{}:{} db::resolve failed: {err}", line!(), file!());
            Err(Status::InternalServerError)
        }
    }
}

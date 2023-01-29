use crate::entry::{Entries, Entry};
use crate::schema::entry;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, PooledConnection},
    result::Error as DieselError,
};
use rocket::{
    async_trait,
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome},
    Request, State,
    serde::uuid::{Uuid, Error as UuidError},
};
use std::{env, str::FromStr};

type DbResult<T> = Result<T, DieselError>;

pub struct DbConn(PooledConnection<ConnectionManager<SqliteConnection>>);

#[async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = try_outcome!(request.guard::<&State<Pool>>().await);

        match pool.get() {
            Ok(pool) => Outcome::Success(DbConn(pool)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = entry)]
pub struct EntryModel {
    uuid: String,
    title: String,
}

impl EntryModel {
    fn new(entry: Entry) -> Self {
        Self {
            uuid: entry.uuid.to_string(),
            title: entry.title,
        }
    }
}

impl TryFrom<EntryModel> for Entry {
    type Error = UuidError;
    fn try_from(value: EntryModel) -> Result<Self, Self::Error> {
        Ok(Entry {
            uuid: Uuid::from_str(&value.uuid)?,
            title: value.title,
        })
    }
}

pub fn establish_connection() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env is not set!");
    Pool::new(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Failed connect to the DB")
}

pub fn list(mut db: DbConn) -> DbResult<Entries> {
    let models = entry::table::load::<EntryModel>(entry::table, &mut db.0)?;
    let mut entries = Vec::with_capacity(models.len());
    for model in models {
        entries.push(Entry::try_from(model).expect("failed convert from EntryModel to Entry"));
    }

    Ok(Entries(entries))
}

pub fn create(mut db: DbConn, entry: Entry) -> DbResult<()> {
    diesel::insert_into(entry::table)
        .values(&EntryModel::new(entry))
        .execute(&mut db.0)
        .map(|_| ())
}

pub fn delete(mut db: DbConn, uuid: Uuid) -> DbResult<()> {
    let search_patter = uuid.to_string();
    diesel::delete(entry::table.find(search_patter))
        .execute(&mut db.0)
        .map(|_| ())
}

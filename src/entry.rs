use rocket::{
    error,
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
    serde::{json as serde_json, Deserialize, Serialize, uuid::Uuid},
};
use std::io::Cursor;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Entry {
    pub uuid: Uuid,
    pub title: String,
}

impl Entry {
    pub fn new(request: EntryRequest) -> Self {
        Entry {
            uuid: Uuid::new_v4(),
            title: request.title,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Entries(pub Vec<Entry>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EntryRequest {
    title: String,
}

impl<'o, 'r> Responder<'r, 'o> for Entries
where
    'o: 'r,
{
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let data = match serde_json::to_string(&self) {
            Ok(data) => data,
            Err(err) => {
                error!("{}:{} {err}", file!(), line!());
                return Response::build().status(Status::InternalServerError).ok();
            }
        };

        Response::build()
            .header(ContentType::JSON)
            .sized_body(data.len(), Cursor::new(data))
            .status(Status::Ok)
            .ok()
    }
}

#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),

    InternalServerError,

    Custom((u16, String)),
}

macro_rules! sf {
    ($s:expr) => {
        $s.to_owned()
    };
}

impl HttpStatus {
    pub fn convert_status_to_u16_and_string(http_status: &HttpStatus) -> (u16, String) {
        match http_status {
            HttpStatus::Ok => (200, sf!("Ok")),
            HttpStatus::Created => (201, sf!("Created")),
            HttpStatus::Accepted => (202, sf!("Accepted")),
            HttpStatus::NoContent => (204, sf!("No Content")),

            HttpStatus::BadRequest => (400, sf!("Bad Request")),
            HttpStatus::Unauthorized(_) => (401, sf!("Unauthorized")),
            HttpStatus::Forbidden(_) => (403, sf!("Forbidden")),

            HttpStatus::NotFound(_) => (404, sf!("Not Found")),

            HttpStatus::InternalServerError => (405, sf!("Internal Server Error")),

            HttpStatus::Custom(x) => (x.0, x.1.clone()),

            _ => HttpStatus::convert_status_to_u16_and_string(&HttpStatus::InternalServerError),
        }
    }
}
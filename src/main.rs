mod word;
use crate::word::{Language, WordPattern, WordError};
use rocket::request::{Request, FromParam};
use rocket::response::{self, Responder};
use rocket::http::Status;

#[macro_use] extern crate rocket;

impl<'r> FromParam<'r> for Language {
    type Error = &'r str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "uk" => Ok(Language::UK),
            "en" => Ok(Language::EN),
            _ => Err(param)
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for WordError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        eprint!("Failed to handle request: {}", self);
        match self {
            _ => Status::InternalServerError.respond_to(req)
        }
    }
}

#[get("/<lang>/<pattern>")]
fn index(lang: Language, pattern: WordPattern) -> Result<String, WordError> {
    let words = word::similar_words(pattern, lang);
    Ok(words?.join("\n"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

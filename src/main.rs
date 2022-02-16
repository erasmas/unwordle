mod word;
use crate::word::{Language, WordPattern, Words, WordError};
use rocket::request::{Request, FromParam};
use rocket::response::{self, Responder};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
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
fn get_words(lang: Language, pattern: WordPattern) -> Result<String, WordError> {
    let words = word::similar_words(pattern, lang);
    Ok(words?.join("\n"))
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/?<lang>&<pattern>")]
fn search(lang: Language, pattern: WordPattern) -> Template {
    let words_result = word::similar_words(pattern, lang);
    let mut context: HashMap<&str, Words> = HashMap::new();
    let words = match words_result {
        Ok(words) => words,
        _ => Vec::new()
    };
    context.insert("words", words);
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, search, get_words])
}

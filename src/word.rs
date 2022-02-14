use std::fmt;
use rusqlite::Connection;

pub type Word = String;
pub type WordPattern = String;
pub type Words = Vec<Word>;

pub enum Language {
    UK, EN
}

impl fmt::Display for Language {
    // TODO: Consider https://github.com/Peternator7/strum
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Language::UK => write!(f, "uk"),
            Language::EN => write!(f, "en")
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WordError {
    #[error("Failed to query database")]
    DbError(#[from] rusqlite::Error),
    #[error(transparent)]
    UnexpectedError(#[from] Box<dyn std::error::Error>),
}

pub fn similar_words(pattern: WordPattern, lang: Language) -> Result<Words, WordError>{
    let conn = Connection::open("./words.db")?; //TODO: read path from ENV
    let query = format!("SELECT word FROM words_{} WHERE word like (?)", lang);
    let mut stmt = conn.prepare(&query)?;
    let like_str = str::replace(&pattern, "*", "%");
    let rows = stmt.query_map([&like_str], |row| row.get(0))?;
    let mut words = Vec::new();
    for row in rows {
        words.push(row?);
    }
    println!("Found words '{}' matching pattern '{}'", words.join(", "), &like_str);
    Ok(words)
}

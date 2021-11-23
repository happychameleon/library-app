use diesel::prelude::*;

use crate::database;
use crate::models::{Author, Book, Work};
use crate::schema::{authors, books, works};

pub fn books() -> Result<Vec<Book>, diesel::result::Error> {
    let connection = database::connection().get().unwrap();

    let books = books::table.load::<Book>(&connection)?;

    Ok(books)
}

pub fn add_book() {}

pub fn work() {}

pub fn add_work() {}

pub fn author() {}

pub fn add_author() {}

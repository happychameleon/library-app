use diesel::prelude::*;

use openlibrary_client::Entity;

use crate::database;
use crate::models::{Author, Book, Work, NewAuthor, NewBook, NewWork};
use crate::schema::{authors, books, works};

pub fn books() -> Result<Vec<Book>, diesel::result::Error> {
    let connection = database::connection().get().unwrap();

    let books = books::table.load::<Book>(&connection)?;

    Ok(books)
}

pub fn add_book(book: &Entity, uid: String) {
    let connection = database::connection().get().unwrap();

    let book: NewBook = NewBook {
        olid: &book.get_olid(),
        uid: &uid,
        title: &book.get_edition().title,
        author: &book.get_author_name(),
        work: &book.get_work().key,
        covers: &book.get_edition().covers[0].to_string(),
    };

    diesel::insert_into(books::table).values(&book).execute(&connection).expect("Error saving book");
}

pub fn work() {}

pub fn add_work() {}

pub fn author() {}

pub fn add_author() {}

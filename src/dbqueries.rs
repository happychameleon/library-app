use diesel::prelude::*;

use openlibrary_client::Entity;

use crate::database;
use crate::models::{Author, Book, NewAuthor, NewBook, NewWork, Work};
use crate::schema::{authors, books, works};

pub fn books() -> Result<Vec<Book>, diesel::result::Error> {
    let connection = database::connection().get().unwrap();

    let books = books::table.load::<Book>(&connection)?;

    Ok(books)
}

pub fn add_book(book: &Entity, uid: &String) {
    let connection = database::connection().get().unwrap();

    let book: NewBook = NewBook {
        olid: &book.get_olid(),
        uid: &uid,
        title: &book.get_edition().title,
        author: &book.get_author_name(),
        work: &book.get_work().key,
        covers: &book.get_edition().covers[0].to_string(),
    };

    diesel::insert_into(books::table)
        .values(&book)
        .execute(&connection)
        .expect("Error saving book");
}

pub fn work() -> Result<Vec<Work>, diesel::result::Error> {
    let connection = database::connection().get().unwrap();

    let works = works::table.load::<Work>(&connection)?;

    Ok(works)
}

pub fn add_work(entity: &Entity) {
    let connection = database::connection().get().unwrap();

    let work: NewWork = NewWork {
        olid: &entity.get_work().key,
        title: &entity.get_work().title,
        author: &entity.get_author_name(),
    };

    diesel::insert_into(works::table)
        .values(&work)
        .execute(&connection)
        .expect("Error saving book");
}

pub fn author() -> Result<Vec<Author>, diesel::result::Error> {
    let connection = database::connection().get().unwrap();

    let authors = authors::table.load::<Author>(&connection)?;

    Ok(authors)
}

pub fn add_author(entity: &Entity) {
    let connection = database::connection().get().unwrap();

    let author: NewAuthor = NewAuthor {
        olid: &entity.get_author().key,
        name: &entity.get_author_name(),
    };

    diesel::insert_into(authors::table)
        .values(&author)
        .execute(&connection)
        .expect("Error saving book");
}

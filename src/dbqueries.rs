use log::debug;

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

    println!("debug cover len {}", book.get_edition().covers.len());

    let len_comp: usize = 0;

    let cover = if book.get_edition().covers.len() == len_comp {
        String::from("")
    } else {
        book.get_edition().covers[0].to_string()
    };



    let book: NewBook = NewBook {
        uid: &uid,
        olid: &book.get_olid(),
        title: &book.get_edition().title,
        full_title: Some(&book.get_edition().full_title),
        subtitle: Some(&book.get_edition().subtitle),
        type_field: Some(&serde_json::to_string(&book.get_edition().type_field).unwrap()),
        authors: Some(&book.get_author_name()),
        works: Some(&book.get_work().key),
        identifiers: Some(&serde_json::to_string(&book.get_edition().identifiers).unwrap()),
        isbn10: Some(&book.get_edition().isbn10[0]),
        isbn13: Some(&book.get_edition().isbn13[0]),
        lccn: Some(&book.get_edition().lccn[0]),
        ocaid: Some(&book.get_edition().ocaid),
        oclc_numbers: Some(&book.get_edition().oclc_numbers[0]),
        covers: Some(&cover),
        links: Some(&book.get_edition().links),
        languages: Some(&serde_json::to_string(&book.get_edition().languages[0]).unwrap()),
        by_statement: Some(&book.get_edition().by_statement),
        weight: Some(&serde_json::to_string(&book.get_edition().weight).unwrap()),
        edition_name: Some(&book.get_edition().edition_name),
        number_of_pages: Some(&serde_json::to_string(&book.get_edition().number_of_pages).unwrap()),
        pagination: Some(&book.get_edition().pagination),
        physical_dimensions: Some(&serde_json::to_string(&book.get_edition().physical_dimensions).unwrap()),
        physical_format: Some(&book.get_edition().physical_format),
        publish_country: Some(&book.get_edition().publish_country),
        publish_date: Some(&book.get_edition().publish_date),
        publish_places: Some(&book.get_edition().publish_places[0]),
        publishers: Some(&book.get_edition().publishers[0]),
        contributions: Some(&book.get_edition().contributions[0]),
        dewey_decimal_class: Some(&book.get_edition().dewey_decimal_class[0]),
        genres: Some(&book.get_edition().genres[0]),
        lc_classifications: Some(&book.get_edition().lc_classifications[0]),
        other_titles: Some(&book.get_edition().other_titles[0]),
        series: Some(&book.get_edition().series[0]),
        source_records: Some(&book.get_edition().source_records[0]),
        subjects: Some(&book.get_edition().subjects[0]),
        work_titles: Some(&book.get_edition().work_titles),
        table_of_contents: Some(&serde_json::to_string(&book.get_edition().table_of_contents).unwrap()),
        description: Some(&book.get_edition().description),
        first_sentence: Some(&book.get_edition().first_sentence),
        notes: Some(&serde_json::to_string(&book.get_edition().notes).unwrap()),
        revision: Some(&serde_json::to_string(&book.get_edition().revision).unwrap()),
        latest_revision: Some(&serde_json::to_string(&book.get_edition().latest_revision).unwrap()),
        created: Some(&serde_json::to_string(&book.get_edition().created).unwrap()),
        last_modified: Some(&serde_json::to_string(&book.get_edition().last_modified).unwrap()),
        isbn_invalid: Some(&book.get_edition().isbn_invalid[0]),
        ia_box_id: Some(&book.get_edition().ia_box_id[0]),
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
        subtitle: Some(&entity.get_work().subtitle),
        type_field: &serde_json::to_string(&entity.get_work().type_field).unwrap(),
        authors: Some(&serde_json::to_string(&entity.get_work().authors).unwrap()),
        covers: Some(&serde_json::to_string(&entity.get_work().covers).unwrap()),
        links: Some(&serde_json::to_string(&entity.get_work().links).unwrap()),
        id_work: Some(&serde_json::to_string(&entity.get_work().id).unwrap()),
        lc_classifications: Some(&serde_json::to_string(&entity.get_work().lc_classifications).unwrap()),
        subjects: Some(&serde_json::to_string(&entity.get_work().subjects).unwrap()),
        first_publish_date: Some(&entity.get_work().first_publish_date),
        description: Some(&serde_json::to_string(&entity.get_work().description).unwrap()),
        notes: Some(&entity.get_work().notes),
        revision: &serde_json::to_string(&entity.get_work().revision).unwrap(), //usize instead of string
        latest_revision: Some(&serde_json::to_string(&entity.get_work().latest_revision).unwrap()),
        created: Some(&serde_json::to_string(&entity.get_work().created).unwrap()),
        last_modified: &serde_json::to_string(&entity.get_work().last_modified).unwrap(),
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
        type_field: &serde_json::to_string(&entity.get_author().type_field).unwrap(),
        alternate_names: Some(&serde_json::to_string(&entity.get_author().alternate_names).unwrap()),
        bio: Some(&serde_json::to_string(&entity.get_author().bio).unwrap()),
        birth_date: Some(&entity.get_author().birth_date),
        death_date: Some(&entity.get_author().death_date),
        location: Some(&serde_json::to_string(&entity.get_author().location).unwrap()),
        date: Some(&serde_json::to_string(&entity.get_author().date).unwrap()),
        entity_type: Some(&serde_json::to_string(&entity.get_author().entity_type).unwrap()),
        fuller_name: Some(&serde_json::to_string(&entity.get_author().fuller_name).unwrap()),
        personal_name: Some(&entity.get_author().personal_name),
        title: Some(&entity.get_author().title),
        photos: Some(&serde_json::to_string(&entity.get_author().photos).unwrap()),
        links: Some(&serde_json::to_string(&entity.get_author().links).unwrap()),
        remote_ids: Some(&serde_json::to_string(&entity.get_author().remote_ids).unwrap()),
        wikipedia: Some(&serde_json::to_string(&entity.get_author().wikipedia).unwrap()),
        revision: &serde_json::to_string(&entity.get_author().revision).unwrap(),
        latest_revision: Some(&serde_json::to_string(&entity.get_author().latest_revision).unwrap()),
        created: Some(&serde_json::to_string(&entity.get_author().created).unwrap()),
        last_modified: &serde_json::to_string(&entity.get_author().last_modified).unwrap(),
    };

    diesel::insert_into(authors::table)
        .values(&author)
        .execute(&connection)
        .expect("Error saving book");
}


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

pub fn book(uid: &String) -> Result<Book, diesel::result::Error> {
    let connection = database::connection().get().unwrap();

    let book = books::dsl::books.filter(books::dsl::uid.like(uid)).first(&connection)?;

    Ok(book)
}

pub fn add_book(book: &Entity, uid: &String) {
    let connection = database::connection().get().unwrap();

    //println!("debug cover len {}", book.get_edition().covers.len());

    let full_title = book.get_edition().full_title;
    let subtitle = book.get_edition().subtitle;

    // type_field: Some(&serde_json::to_string(&book.get_edition().type_field).unwrap()),
    //     authors: book.get_edition().authors.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     works: Some(""),//book.get_edition().works.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     identifiers: book.get_edition().identifiers.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     isbn10: book.get_edition().isbn10.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     isbn13: book.get_edition().isbn13.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     lccn: book.get_edition().lccn.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     ocaid: book.get_edition().ocaid.as_ref().map(|s| s.as_str()),
    //     oclc_numbers: book.get_edition().oclc_numbers.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     covers: book.get_edition().covers.as_ref().map(|c| serde_json::to_string(c).unwrap().as_str()),
    //     links: book.get_edition().links.as_ref().map(|s| s.as_str()),
    //     languages: book.get_edition().languages.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     by_statement: book.get_edition().by_statement.as_ref().map(|s| s.as_str()),
    //     weight: book.get_edition().weight.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     edition_name: book.get_edition().edition_name.as_ref().map(|s| s.as_str()),
    //     number_of_pages: book.get_edition().number_of_pages.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     pagination: book.get_edition().pagination.as_ref().map(|s| s.as_str()),
    //     physical_dimensions: book.get_edition().physical_dimensions.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     physical_format: book.get_edition().physical_format.as_ref().map(|s| s.as_str()),
    //     publish_country: book.get_edition().publish_country.as_ref().map(|s| s.as_str()),
    //     publish_date: book.get_edition().publish_date.as_ref().map(|s| s.as_str()),
    //     publish_places: book.get_edition().publish_places.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     publishers: book.get_edition().publishers.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     contributions: book.get_edition().contributions.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     dewey_decimal_class: book.get_edition().dewey_decimal_class.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     genres: book.get_edition().genres.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     lc_classifications: book.get_edition().lc_classifications.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     other_titles: book.get_edition().other_titles.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     series: book.get_edition().series.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     source_records: book.get_edition().source_records.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     subjects: book.get_edition().subjects.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     work_titles: book.get_edition().work_titles.as_ref().map(|s| s.as_str()),
    //     table_of_contents: book.get_edition().table_of_contents.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     description: book.get_edition().description.as_ref().map(|s| s.as_str()),
    //     first_sentence: book.get_edition().first_sentence.as_ref().map(|s| s.as_str()),
    //     notes: book.get_edition().notes.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     revision: Some(&serde_json::to_string(&book.get_edition().revision).unwrap()),
    //     latest_revision: book.get_edition().latest_revision.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     created: Some(&serde_json::to_string(&book.get_edition().created).unwrap()),
    //     last_modified: Some(&serde_json::to_string(&book.get_edition().last_modified).unwrap()),
    //     isbn_invalid: book.get_edition().isbn_invalid.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    //     ia_box_id: book.get_edition().ia_box_id.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),

    let book: NewBook = NewBook {
        uid: &uid,
        olid: &book.get_olid(),
        title: &book.get_edition().title,
        full_title: full_title.as_ref().map(|s| s.as_str()),
        subtitle: subtitle.as_ref().map(|s| s.as_str()),
        type_field: Some(&serde_json::to_string(&book.get_edition().type_field).unwrap()),
        authors: book.get_edition().authors.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        works: Some(""),//book.get_edition().works.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        identifiers: book.get_edition().identifiers.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        isbn10: book.get_edition().isbn10.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        isbn13: book.get_edition().isbn13.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        lccn: book.get_edition().lccn.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        ocaid: book.get_edition().ocaid.as_ref().map(|s| s.as_str()),
        oclc_numbers: book.get_edition().oclc_numbers.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        covers: book.get_edition().covers.as_ref().map(|c| serde_json::to_string(c).unwrap().as_str()),
        links: book.get_edition().links.as_ref().map(|s| s.as_str()),
        languages: book.get_edition().languages.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        by_statement: book.get_edition().by_statement.as_ref().map(|s| s.as_str()),
        weight: book.get_edition().weight.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        edition_name: book.get_edition().edition_name.as_ref().map(|s| s.as_str()),
        number_of_pages: book.get_edition().number_of_pages.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        pagination: book.get_edition().pagination.as_ref().map(|s| s.as_str()),
        physical_dimensions: book.get_edition().physical_dimensions.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        physical_format: book.get_edition().physical_format.as_ref().map(|s| s.as_str()),
        publish_country: book.get_edition().publish_country.as_ref().map(|s| s.as_str()),
        publish_date: book.get_edition().publish_date.as_ref().map(|s| s.as_str()),
        publish_places: book.get_edition().publish_places.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        publishers: book.get_edition().publishers.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        contributions: book.get_edition().contributions.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        dewey_decimal_class: book.get_edition().dewey_decimal_class.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        genres: book.get_edition().genres.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        lc_classifications: book.get_edition().lc_classifications.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        other_titles: book.get_edition().other_titles.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        series: book.get_edition().series.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        source_records: book.get_edition().source_records.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        subjects: book.get_edition().subjects.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        work_titles: book.get_edition().work_titles.as_ref().map(|s| s.as_str()),
        table_of_contents: book.get_edition().table_of_contents.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        description: book.get_edition().description.as_ref().map(|s| s.as_str()),
        first_sentence: book.get_edition().first_sentence.as_ref().map(|s| s.as_str()),
        notes: book.get_edition().notes.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        revision: Some(&serde_json::to_string(&book.get_edition().revision).unwrap()),
        latest_revision: book.get_edition().latest_revision.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        created: Some(&serde_json::to_string(&book.get_edition().created).unwrap()),
        last_modified: Some(&serde_json::to_string(&book.get_edition().last_modified).unwrap()),
        isbn_invalid: book.get_edition().isbn_invalid.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        ia_box_id: book.get_edition().ia_box_id.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
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
        subtitle: entity.get_work().subtitle.as_ref().map(|s| s.as_str()),
        type_field: &serde_json::to_string(&entity.get_work().type_field).unwrap(),
        authors: Some(&serde_json::to_string(&entity.get_work().authors).unwrap()),
        covers: Some(&serde_json::to_string(&entity.get_work().covers).unwrap()),
        links: Some(&serde_json::to_string(&entity.get_work().links).unwrap()),
        id_work: Some(&serde_json::to_string(&entity.get_work().id).unwrap()),
        lc_classifications: Some(&serde_json::to_string(&entity.get_work().lc_classifications).unwrap()),
        subjects: Some(&serde_json::to_string(&entity.get_work().subjects).unwrap()),
        first_publish_date: entity.get_work().first_publish_date.as_ref().map(|s| s.as_str()),
        description: Some(&serde_json::to_string(&entity.get_work().description).unwrap()),
        notes: entity.get_work().notes.as_ref().map(|s| s.as_str()),
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
        birth_date: entity.get_author().birth_date.as_ref().map(|s| s.as_str()),
        death_date: entity.get_author().death_date.as_ref().map(|s| s.as_str()),
        location: Some(&serde_json::to_string(&entity.get_author().location).unwrap()),
        date: Some(&serde_json::to_string(&entity.get_author().date).unwrap()),
        entity_type: Some(&serde_json::to_string(&entity.get_author().entity_type).unwrap()),
        fuller_name: Some(&serde_json::to_string(&entity.get_author().fuller_name).unwrap()),
        personal_name: entity.get_author().personal_name.as_ref().map(|s| s.as_str()),
        title: entity.get_author().title.as_ref().map(|s| s.as_str()),
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


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

    println!("debug cover len {}", book.get_edition().covers.len());

    let len_comp: usize = 0;

    let cover = if book.get_edition().covers.len() == len_comp {
        String::from("")
    } else {
        book.get_edition().covers[0].to_string()
    };


    let full_title = &book.get_edition().full_title;
    let subtitle = &book.get_edition().subtitle;
    let type_field = &serde_json::to_string(&book.get_edition().type_field).unwrap();
    let authors = &book.get_author_name();
    let works = &book.get_work().key;
    let identifiers = &serde_json::to_string(&book.get_edition().identifiers).unwrap();
    let isbn10 = &book.get_edition().isbn10[0];
    let isbn13 = &book.get_edition().isbn13[0];
    let lccn = &book.get_edition().lccn[0];
    let ocaid = &book.get_edition().ocaid;
    let oclc_numbers = &book.get_edition().oclc_numbers[0];
    //let covers: Some(&cover),
    let links = &book.get_edition().links;
    let languages = &serde_json::to_string(&book.get_edition().languages[0]).unwrap();
    let by_statement = &book.get_edition().by_statement;
    let weight = &serde_json::to_string(&book.get_edition().weight).unwrap();
    let edition_name = &book.get_edition().edition_name;
    let number_of_pages = &serde_json::to_string(&book.get_edition().number_of_pages).unwrap();
    let pagination = &book.get_edition().pagination;
    let physical_dimensions = &serde_json::to_string(&book.get_edition().physical_dimensions).unwrap();
    let physical_format = &book.get_edition().physical_format;
    let publish_country = &book.get_edition().publish_country;
    let publish_date = &book.get_edition().publish_date;
    let publish_places = &book.get_edition().publish_places[0];
    let publishers = &book.get_edition().publishers[0];
    let contributions = &book.get_edition().contributions[0];
    let dewey_decimal_class = &book.get_edition().dewey_decimal_class[0];
    let genres = &book.get_edition().genres[0];
    let lc_classifications = &book.get_edition().lc_classifications[0];
    let other_titles = &book.get_edition().other_titles[0];
    let series = &book.get_edition().series[0];
    let source_records = &book.get_edition().source_records[0];
    let subjects = &book.get_edition().subjects[0];
    let work_titles = &book.get_edition().work_titles;
    let table_of_contents = &serde_json::to_string(&book.get_edition().table_of_contents).unwrap();
    let description = &book.get_edition().description;
    let first_sentence = &book.get_edition().first_sentence;
    let notes = &serde_json::to_string(&book.get_edition().notes).unwrap();
    let revision = &serde_json::to_string(&book.get_edition().revision).unwrap();
    let latest_revision = &serde_json::to_string(&book.get_edition().latest_revision).unwrap();
    let created = &serde_json::to_string(&book.get_edition().created).unwrap();
    let last_modified = &serde_json::to_string(&book.get_edition().last_modified).unwrap();
    let isbn_invalid = &book.get_edition().isbn_invalid[0];
    let ia_box_id = &book.get_edition().ia_box_id[0];


    let new_book: NewBook = NewBook {
        uid: &uid,
        olid: &book.get_olid(),
        title: &book.get_edition().title.clone(),
        full_title: Some(full_title),
        subtitle: Some(subtitle),
        type_field: Some(&type_field),
        authors: Some(authors),
        works: Some(works),
        identifiers: Some(identifiers),
        isbn10: Some(isbn10),
        isbn13: Some(isbn13),
        lccn: Some(lccn),
        ocaid: Some(ocaid),
        oclc_numbers: Some(oclc_numbers),
        covers: Some(&cover),
        links: Some(links),
        languages: Some(languages),
        by_statement: Some(by_statement),
        weight: Some(weight),
        edition_name: Some(edition_name),
        number_of_pages: Some(number_of_pages),
        pagination: Some(pagination),
        physical_dimensions: Some(physical_dimensions),
        physical_format: Some(physical_format),
        publish_country: Some(publish_country),
        publish_date: Some(publish_date),
        publish_places: Some(publish_places),
        publishers: Some(publishers),
        contributions: Some(contributions),
        dewey_decimal_class: Some(dewey_decimal_class),
        genres: Some(genres),
        lc_classifications: Some(lc_classifications),
        other_titles: Some(other_titles),
        series: Some(series),
        source_records: Some(source_records),
        subjects: Some(subjects),
        work_titles: Some(work_titles),
        table_of_contents: Some(table_of_contents),
        description: Some(description),
        first_sentence: Some(first_sentence),
        notes: Some(notes),
        revision: Some(revision),
        latest_revision: Some(latest_revision),
        created: Some(created),
        last_modified: Some(last_modified),
        isbn_invalid: Some(isbn_invalid),
        ia_box_id: Some(ia_box_id),
    };

    diesel::insert_into(books::table)
        .values(&new_book)
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

    let subtitle = &entity.get_work().subtitle;
    //let type_field: &serde_json::to_string(&entity.get_work().type_field).unwrap(),
    let authors = &serde_json::to_string(&entity.get_work().authors).unwrap();
    let covers = &serde_json::to_string(&entity.get_work().covers).unwrap();
    let links = &serde_json::to_string(&entity.get_work().links).unwrap();
    let id_work = &serde_json::to_string(&entity.get_work().id).unwrap();
    let lc_classifications = &serde_json::to_string(&entity.get_work().lc_classifications).unwrap();
    let subjects = &serde_json::to_string(&entity.get_work().subjects).unwrap();
    let first_publish_date = &entity.get_work().first_publish_date;
    let description = &serde_json::to_string(&entity.get_work().description).unwrap();
    let notes = &entity.get_work().notes;
    //let revision: &serde_json::to_string(&entity.get_work().revision).unwrap(), //usize instead of string
    let latest_revision = &serde_json::to_string(&entity.get_work().latest_revision).unwrap();
    let created = &serde_json::to_string(&entity.get_work().created).unwrap();
    //let last_modified = &serde_json::to_string(&entity.get_work().last_modified).unwrap(),

    let work: NewWork = NewWork {
        olid: &entity.get_work().key,
        title: &entity.get_work().title,
        subtitle: Some(subtitle),
        type_field: &serde_json::to_string(&entity.get_work().type_field).unwrap(),
        authors: Some(authors),
        covers: Some(covers),
        links: Some(links),
        id_work: Some(id_work),
        lc_classifications: Some(lc_classifications),
        subjects: Some(subjects),
        first_publish_date: Some(first_publish_date),
        description: Some(description),
        notes: Some(notes),
        revision: &serde_json::to_string(&entity.get_work().revision).unwrap(), //usize instead of string
        latest_revision: Some(latest_revision),
        created: Some(created),
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

    let alternate_names = &serde_json::to_string(&entity.get_author().alternate_names).unwrap();
    let bio = &serde_json::to_string(&entity.get_author().bio).unwrap();
    let birth_date = &entity.get_author().birth_date;
    let death_date = &entity.get_author().death_date;
    let location = &serde_json::to_string(&entity.get_author().location).unwrap();
    let date = &serde_json::to_string(&entity.get_author().date).unwrap();
    let entity_type = &serde_json::to_string(&entity.get_author().entity_type).unwrap();
    let fuller_name = &serde_json::to_string(&entity.get_author().fuller_name).unwrap();
    let personal_name = &entity.get_author().personal_name;
    let title = &entity.get_author().title;
    let photos = &serde_json::to_string(&entity.get_author().photos).unwrap();
    let links = &serde_json::to_string(&entity.get_author().links).unwrap();
    let remote_ids = &serde_json::to_string(&entity.get_author().remote_ids).unwrap();
    let wikipedia = &serde_json::to_string(&entity.get_author().wikipedia).unwrap();
    // let revision: &serde_json::to_string(&entity.get_author().revision).unwrap(),
    let latest_revision = &serde_json::to_string(&entity.get_author().latest_revision).unwrap();
    let created = &serde_json::to_string(&entity.get_author().created).unwrap();
    // let last_modified: &serde_json::to_string(&entity.get_author().last_modified).unwrap(),

    let author: NewAuthor = NewAuthor {
        olid: &entity.get_author().key,
        name: &entity.get_author_name(),
        type_field: &serde_json::to_string(&entity.get_author().type_field).unwrap(),
        alternate_names: Some(alternate_names),
        bio: Some(bio),
        birth_date: Some(birth_date),
        death_date: Some(death_date),
        location: Some(location),
        date: Some(date),
        entity_type: Some(entity_type),
        fuller_name: Some(fuller_name),
        personal_name: Some(personal_name),
        title: Some(title),
        photos: Some(photos),
        links: Some(links),
        remote_ids: Some(remote_ids),
        wikipedia: Some(wikipedia),
        revision: &serde_json::to_string(&entity.get_author().revision).unwrap(),
        latest_revision: Some(latest_revision),
        created: Some(created),
        last_modified: &serde_json::to_string(&entity.get_author().last_modified).unwrap(),
    };

    diesel::insert_into(authors::table)
        .values(&author)
        .execute(&connection)
        .expect("Error saving book");
}


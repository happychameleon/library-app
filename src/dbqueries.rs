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

    let full_title = book.get_edition().full_title;
    let subtitle = book.get_edition().subtitle;
    let type_field = &serde_json::to_string(&book.get_edition().type_field).unwrap();
    let authors = match book.get_edition().authors {
            Some(authors) => Some(serde_json::to_string(&authors).unwrap()),
            None => None,
        };
    //let works: Some(""),//book.get_edition().works.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let identifiers = match book.get_edition().identifiers {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let isbn10 = match book.get_edition().isbn10 {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let isbn13 = match book.get_edition().isbn13 {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let lccn = match book.get_edition().lccn {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let ocaid = book.get_edition().ocaid;//.as_ref().map(|s| s.as_str()),
    let oclc_numbers = match book.get_edition().oclc_numbers {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let covers = match book.get_edition().covers {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|c| serde_json::to_string(c).unwrap().as_str()),
    let links = book.get_edition().links;//.as_ref().map(|s| s.as_str()),
    let languages = match book.get_edition().languages {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let by_statement = book.get_edition().by_statement;//.as_ref().map(|s| s.as_str()),
    let weight = match book.get_edition().weight {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let edition_name = book.get_edition().edition_name;//.as_ref().map(|s| s.as_str()),
    let number_of_pages = match book.get_edition().number_of_pages {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let pagination = book.get_edition().pagination;//.as_ref().map(|s| s.as_str()),
    let physical_dimensions = match book.get_edition().physical_dimensions {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let physical_format = book.get_edition().physical_format;//.as_ref().map(|s| s.as_str()),
    let publish_country = book.get_edition().publish_country;//.as_ref().map(|s| s.as_str()),
    let publish_date = book.get_edition().publish_date;//.as_ref().map(|s| s.as_str()),
    let publish_places = match book.get_edition().publish_places {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let publishers = match book.get_edition().publishers {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let contributions = match book.get_edition().contributions {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let dewey_decimal_class = match book.get_edition().dewey_decimal_class {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let genres = match book.get_edition().genres {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let lc_classifications = match book.get_edition().lc_classifications {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let other_titles = match book.get_edition().other_titles {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let series = match book.get_edition().series {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let source_records = match book.get_edition().source_records {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let subjects = match book.get_edition().subjects {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let work_titles = book.get_edition().work_titles;//.as_ref().map(|s| s.as_str()),
    let table_of_contents = match book.get_edition().table_of_contents {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };//.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
    let description = book.get_edition().description;//.as_ref().map(|s| s.as_str()),
    let first_sentence = book.get_edition().first_sentence;//.as_ref().map(|s| s.as_str()),
    let notes = match book.get_edition().notes {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let revision = &book.get_edition().revision.to_string();
    let latest_revision = match book.get_edition().latest_revision {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let created = &serde_json::to_string(&book.get_edition().created).unwrap();
    let last_modified = &serde_json::to_string(&book.get_edition().last_modified).unwrap();
    let isbn_invalid = match book.get_edition().isbn_invalid {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let ia_box_id = match book.get_edition().ia_box_id {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };

    let book: NewBook = NewBook {
        uid: &uid,
        olid: &book.get_olid(),
        title: &book.get_edition().title,
        full_title: full_title.as_ref().map(|s| s.as_str()),
        subtitle: subtitle.as_ref().map(|s| s.as_str()),
        type_field: Some(type_field),
        authors: authors.as_ref().map(|s| s.as_str()),//authors.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        works: Some(""),//book.get_edition().works.as_ref().map(|o| serde_json::to_string(o).unwrap().as_str()),
        identifiers: identifiers.as_ref().map(|s| s.as_str()),
        isbn10: isbn10.as_ref().map(|s| s.as_str()),
        isbn13: isbn13.as_ref().map(|s| s.as_str()),
        lccn: lccn.as_ref().map(|s| s.as_str()),
        ocaid: ocaid.as_ref().map(|s| s.as_str()),
        oclc_numbers: oclc_numbers.as_ref().map(|s| s.as_str()),
        covers: covers.as_ref().map(|s| s.as_str()),
        links: links.as_ref().map(|s| s.as_str()),
        languages: languages.as_ref().map(|s| s.as_str()),
        by_statement:by_statement.as_ref().map(|s| s.as_str()),
        weight: weight.as_ref().map(|s| s.as_str()),
        edition_name:edition_name.as_ref().map(|s| s.as_str()),
        number_of_pages: number_of_pages.as_ref().map(|s| s.as_str()),
        pagination: pagination.as_ref().map(|s| s.as_str()),
        physical_dimensions:physical_dimensions.as_ref().map(|s| s.as_str()),
        physical_format: physical_format.as_ref().map(|s| s.as_str()),
        publish_country: publish_country.as_ref().map(|s| s.as_str()),
        publish_date: publish_date.as_ref().map(|s| s.as_str()),
        publish_places: publish_places.as_ref().map(|s| s.as_str()),
        publishers: publishers.as_ref().map(|s| s.as_str()),
        contributions: contributions.as_ref().map(|s| s.as_str()),
        dewey_decimal_class:dewey_decimal_class.as_ref().map(|s| s.as_str()),
        genres: genres.as_ref().map(|s| s.as_str()),
        lc_classifications:lc_classifications.as_ref().map(|s| s.as_str()),
        other_titles: other_titles.as_ref().map(|s| s.as_str()),
        series: series.as_ref().map(|s| s.as_str()),
        source_records:source_records.as_ref().map(|s| s.as_str()),
        subjects: subjects.as_ref().map(|s| s.as_str()),
        work_titles: work_titles.as_ref().map(|s| s.as_str()),
        table_of_contents:table_of_contents.as_ref().map(|s| s.as_str()),
        description: description.as_ref().map(|s| s.as_str()),
        first_sentence:first_sentence.as_ref().map(|s| s.as_str()),
        notes: notes.as_ref().map(|s| s.as_str()),
        revision: Some(revision),//Some(&book.get_edition().revision.to_string()),
        latest_revision: latest_revision.as_ref().map(|s| s.as_str()),
        created: Some(created),
        last_modified: Some(last_modified),
        isbn_invalid: isbn_invalid.as_ref().map(|s| s.as_str()),
        ia_box_id: ia_box_id.as_ref().map(|s| s.as_str()),
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

    let subtitle = entity.get_work().subtitle;//.as_ref().map(|s| s.as_str()),
    //let type_field = &serde_json::to_string(&entity.get_work().type_field).unwrap(),
    let authors = match entity.get_work().authors {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let covers = match entity.get_work().covers {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let links = match entity.get_work().links {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let id_work = match entity.get_work().id {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let lc_classifications = match entity.get_work().lc_classifications {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let subjects = match entity.get_work().subjects {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let first_publish_date = entity.get_work().first_publish_date;
    let description = match entity.get_work().description {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let notes = entity.get_work().notes;//.as_ref().map(|s| s.as_str()),
    //let revision = &serde_json::to_string(&entity.get_work().revision).unwrap(), //usize instead of string
    let latest_revision = match entity.get_work().latest_revision {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    let created = match entity.get_work().created {
        Some(o) => Some(serde_json::to_string(&o).unwrap()),
        None => None,
    };
    //let last_modified = &serde_json::to_string(&entity.get_work().last_modified).unwrap();

    let work: NewWork = NewWork {
        olid: &entity.get_work().key,
        title: &entity.get_work().title,
        subtitle: subtitle.as_ref().map(|s| s.as_str()),
        type_field: &serde_json::to_string(&entity.get_work().type_field).unwrap(),
        authors: authors.as_ref().map(|s| s.as_str()),
        covers: covers.as_ref().map(|s| s.as_str()),
        links: links.as_ref().map(|s| s.as_str()),
        id_work: id_work.as_ref().map(|s| s.as_str()),
        lc_classifications: lc_classifications.as_ref().map(|s| s.as_str()),
        subjects: subjects.as_ref().map(|s| s.as_str()),
        first_publish_date: first_publish_date.as_ref().map(|s| s.as_str()),
        description: description.as_ref().map(|s| s.as_str()),
        notes: notes.as_ref().map(|s| s.as_str()),
        revision: &serde_json::to_string(&entity.get_work().revision).unwrap(), //usize instead of string
        latest_revision: latest_revision.as_ref().map(|s| s.as_str()),
        created: created.as_ref().map(|s| s.as_str()),
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


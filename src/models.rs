use diesel::Queryable;
use serde_json::Value;

use super::schema::{authors, books, devices, editions, libraries, works};

#[derive(Queryable, PartialEq, Debug)]
pub struct Devices {
    pub id: i32,
    pub devid: String,
    pub pubkey: String,
    pub privkey: Option<String>,
    pub name: String,
    pub home_libid: String,
    pub is_current_device: bool,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Libraries {
    pub id: i32,
    pub libid: String,
    pub pubkey: String,
    pub privkey: Option<String>,
    pub name: String,
    pub is_home: bool,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Book {
    pub id: i32,
    pub uid: String,
    pub isbn: String,
    pub edition_olid: String,
    pub authors_olid: String,
    pub works_olid: String,
    pub home_libid: String,
    pub current_libid: String,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Edition {
    pub id: i32,
    pub olid: String,
    pub title: String,
    pub full_title: Option<String>,
    pub subtitle: Option<String>,
    pub type_field: Option<String>,
    pub authors: Option<String>,
    pub works: Option<String>,
    pub identifiers: Option<String>,
    pub isbn10: Option<String>,
    pub isbn13: Option<String>,
    pub lccn: Option<String>,
    pub ocaid: Option<String>,
    pub oclc_numbers: Option<String>,
    pub covers: Option<String>,
    pub links: Option<String>,
    pub languages: Option<String>,
    pub by_statement: Option<String>,
    pub weight: Option<String>,
    pub edition_name: Option<String>,
    pub number_of_pages: Option<String>,
    pub pagination: Option<String>,
    pub physical_dimensions: Option<String>,
    pub physical_format: Option<String>,
    pub publish_country: Option<String>,
    pub publish_date: Option<String>,
    pub publish_places: Option<String>,
    pub publishers: Option<String>,
    pub contributions: Option<String>,
    pub dewey_decimal_class: Option<String>,
    pub genres: Option<String>,
    pub lc_classifications: Option<String>,
    pub other_titles: Option<String>,
    pub series: Option<String>,
    pub source_records: Option<String>,
    pub subjects: Option<String>,
    pub work_titles: Option<String>,
    pub table_of_contents: Option<String>,
    pub description: Option<String>,
    pub first_sentence: Option<String>,
    pub notes: Option<String>,
    pub revision: Option<String>,
    pub latest_revision: Option<String>,
    pub created: Option<String>,
    pub last_modified: Option<String>,
    pub isbn_invalid: Option<String>,
    pub ia_box_id: Option<String>,
}

impl Edition {
    pub fn authors(&self) -> Option<Vec<String>> {
        let authors = match &self.authors.as_ref() {
            Some(authors) => {
                println!("json authors {}", authors);
                let author: Vec<openlibrary_client::AuthorsEdition> =
                    serde_json::from_str(&self.authors.as_ref().unwrap()).unwrap();
                let mut new_author: Vec<String> = Vec::new();
                for i in author {
                    new_author.push(i.key);
                }
                Some(new_author)
            }
            None => None,
        };

        authors
    }

    pub fn works(&self) -> Vec<String> {
        println!("Json string: {}", self.works.as_ref().unwrap());
        let works: Vec<openlibrary_client::Works> =
            serde_json::from_str(&self.works.as_ref().unwrap()).unwrap();

        let mut new_works: Vec<String> = Vec::new();
        for i in works {
            new_works.push(i.key);
        }

        new_works
    }
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Author {
    pub id: i32,
    pub olid: String,
    pub name: String,
    pub type_field: String,
    pub alternate_names: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub location: Option<String>,
    pub date: Option<String>,
    pub entity_type: Option<String>,
    pub fuller_name: Option<String>,
    pub personal_name: Option<String>,
    pub title: Option<String>,
    pub photos: Option<String>,
    pub links: Option<String>,
    pub remote_ids: Option<String>,
    pub wikipedia: Option<String>,
    pub revision: String,
    pub latest_revision: Option<String>,
    pub created: Option<String>,
    pub last_modified: String,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Work {
    pub id: i32,
    pub olid: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub type_field: String,
    pub authors: Option<String>,
    pub covers: Option<String>,
    pub links: Option<String>,
    pub id_work: Option<String>,
    pub lc_classifications: Option<String>,
    pub subjects: Option<String>,
    pub first_publish_date: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub revision: String,
    pub latest_revision: Option<String>,
    pub created: Option<String>,
    pub last_modified: String,
}

impl Work {
    pub fn authors(&self) -> Option<Vec<String>> {
        let authors = match &self.authors.as_ref() {
            Some(authors) => {
                let author: Vec<openlibrary_client::AuthorsWork> =
                    serde_json::from_str(&self.authors.as_ref().unwrap()).unwrap();
                let mut new_author: Vec<String> = Vec::new();
                for i in author {
                    new_author.push(i.author.key);
                }
                Some(new_author)
            }
            None => None,
        };

        authors
    }
}

#[derive(Insertable)]
#[table_name = "devices"]
pub struct NewDevice<'a> {
    pub devid: &'a str,
    pub pubkey: &'a str,
    pub privkey: Option<&'a str>,
    pub name: &'a str,
    pub home_libid: &'a str,
    //pub is_current_device: &'a str,
}

#[derive(Insertable)]
#[table_name = "libraries"]
pub struct NewLibrary<'a> {
    pub libid: &'a str,
    pub pubkey: &'a str,
    pub privkey: Option<&'a str>,
    pub name: &'a str,
    //pub is_home: &'a str,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub uid: &'a str,
    pub isbn: &'a str,
    pub edition_olid: &'a str,
    pub authors_olid: &'a str,
    pub works_olid: &'a str,
    pub home_libid: &'a str,
    pub current_libid: &'a str,
}

#[derive(Insertable)]
#[table_name = "editions"]
pub struct NewEdition<'a> {
    pub olid: &'a str,
    pub title: &'a str,
    pub full_title: Option<&'a str>,
    pub subtitle: Option<&'a str>,
    pub type_field: Option<&'a str>,
    pub authors: Option<&'a str>,
    pub works: Option<&'a str>,
    pub identifiers: Option<&'a str>,
    pub isbn10: Option<&'a str>,
    pub isbn13: Option<&'a str>,
    pub lccn: Option<&'a str>,
    pub ocaid: Option<&'a str>,
    pub oclc_numbers: Option<&'a str>,
    pub covers: Option<&'a str>,
    pub links: Option<&'a str>,
    pub languages: Option<&'a str>,
    pub by_statement: Option<&'a str>,
    pub weight: Option<&'a str>,
    pub edition_name: Option<&'a str>,
    pub number_of_pages: Option<&'a str>,
    pub pagination: Option<&'a str>,
    pub physical_dimensions: Option<&'a str>,
    pub physical_format: Option<&'a str>,
    pub publish_country: Option<&'a str>,
    pub publish_date: Option<&'a str>,
    pub publish_places: Option<&'a str>,
    pub publishers: Option<&'a str>,
    pub contributions: Option<&'a str>,
    pub dewey_decimal_class: Option<&'a str>,
    pub genres: Option<&'a str>,
    pub lc_classifications: Option<&'a str>,
    pub other_titles: Option<&'a str>,
    pub series: Option<&'a str>,
    pub source_records: Option<&'a str>,
    pub subjects: Option<&'a str>,
    pub work_titles: Option<&'a str>,
    pub table_of_contents: Option<&'a str>,
    pub description: Option<&'a str>,
    pub first_sentence: Option<&'a str>,
    pub notes: Option<&'a str>,
    pub revision: Option<&'a str>,
    pub latest_revision: Option<&'a str>,
    pub created: Option<&'a str>,
    pub last_modified: Option<&'a str>,
    pub isbn_invalid: Option<&'a str>,
    pub ia_box_id: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name = "authors"]
pub struct NewAuthor<'a> {
    pub olid: &'a str,
    pub name: &'a str,
    pub type_field: &'a str,
    pub alternate_names: Option<&'a str>,
    pub bio: Option<&'a str>,
    pub birth_date: Option<&'a str>,
    pub death_date: Option<&'a str>,
    pub location: Option<&'a str>,
    pub date: Option<&'a str>,
    pub entity_type: Option<&'a str>,
    pub fuller_name: Option<&'a str>,
    pub personal_name: Option<&'a str>,
    pub title: Option<&'a str>,
    pub photos: Option<&'a str>,
    pub links: Option<&'a str>,
    pub remote_ids: Option<&'a str>,
    pub wikipedia: Option<&'a str>,
    pub revision: &'a str,
    pub latest_revision: Option<&'a str>,
    pub created: Option<&'a str>,
    pub last_modified: &'a str,
}

#[derive(Insertable)]
#[table_name = "works"]
pub struct NewWork<'a> {
    pub olid: &'a str,
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub type_field: &'a str,
    pub authors: Option<&'a str>,
    pub covers: Option<&'a str>,
    pub links: Option<&'a str>,
    pub id_work: Option<&'a str>,
    pub lc_classifications: Option<&'a str>,
    pub subjects: Option<&'a str>,
    pub first_publish_date: Option<&'a str>,
    pub description: Option<&'a str>,
    pub notes: Option<&'a str>,
    pub revision: &'a str,
    pub latest_revision: Option<&'a str>,
    pub created: Option<&'a str>,
    pub last_modified: &'a str,
}

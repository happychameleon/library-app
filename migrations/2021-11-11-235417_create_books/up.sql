-- Your SQL goes here
CREATE TABLE `books` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `uid` TEXT UNIQUE NOT NULL, -- a unique edition specific random id
    `olid` TEXT NOT NULL,
    `title` TEXT NOT NULL,
    `full_title` TEXT,
    `subtitle` TEXT,
    `type_field` TEXT, -- pub type_field: Type,
    `authors` TEXT, -- pub authors: Vec<Authors>,
    `works` TEXT, -- pub works: Vec<Works>,
    `identifiers` TEXT, -- pub identifiers: Identifiers,
    `isbn10` TEXT, -- pub isbn10: Vec<String>,
    `isbn13` TEXT, -- pub isbn13: Vec<String>,
    `lccn` TEXT, -- pub lccn: Vec<String>,
    `ocaid` TEXT,
    `oclc_numbers` TEXT, -- pub oclc_numbers: Vec<String>,
    `covers` TEXT -- pub covers: Vec<isize>,
    `links` TEXT,
    `languages` TEXT, -- pub languages: Vec<Language>,
    `by_statement` TEXT,
    `weight` TEXT, -- pub weight: Weight,
    `edition_name` TEXT,
    `number_of_pages` TEXT,
    `pagination` TEXT,
    `physical_dimensions` TEXT, -- pub physical_dimensions: PhysicalDimensions,
    `physical_format` TEXT,
    `publish_country` TEXT,
    `publish_date` TEXT,
    `publish_places` TEXT,
    `publishers` TEXT, -- pub publishers: Vec<String>,
    `contributions` TEXT, -- pub contributions: Vec<String>,
    `dewey_decimal_class` TEXT, -- pub dewey_decimal_class: Vec<String>,
    `genres` TEXT, -- pub genres: Vec<String>,
    `lc_classifications` TEXT, -- pub lc_classifications: Vec<String>,
    `other_titles` TEXT, -- pub other_titles: Vec<String>,
    `series` TEXT, -- pub series: Vec<String>,
    `source_records` TEXT, -- pub source_records: Vec<String>,
    `subjects` TEXT, -- pub subjects: Vec<String>,
    `work_titles` TEXT,
    `table_of_contents` TEXT, -- pub table_of_contents: TableOfContents,
    `description` TEXT,
    `first_sentence` TEXT,
    `notes` TEXT, -- pub notes: Notes,
    `revision` TEXT, -- pub revision: usize,
    `latest_revision` TEXT, -- pub latest_revision: usize,
    `created` TEXT, -- pub created: Created,
    `last_modified` TEXT, -- pub last_modified: LastModified,
    `isbn_invalid` TEXT, -- pub isbn_invalid: Vec<String>,
    `ia_box_id` TEXT, -- pub ia_box_id: Vec<String>,
);


CREATE TABLE `authors` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `olid` TEXT NOT NULL,
    `name` TEXT NOT NULL
);

CREATE TABLE `works` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `olid` TEXT NOT NULL,
    `title` TEXT NOT NULL,
    `author` TEXT
);

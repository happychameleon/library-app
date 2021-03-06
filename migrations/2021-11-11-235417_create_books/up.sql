
-- devices connected to the same local library
CREATE TABLE `devices` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `devid` TEXT NOT NULL, -- unique device id that identifies a device that is connected to local lib
    `pubkey` TEXT NOT NULL,
    `privkey` TEXT,
    `name` TEXT NOT NULL,
    `home_libid` TEXT NOT NULL, -- libid of the devices home library
    `is_current_device` BOOLEAN NOT NULL DEFAULT 0
);

-- libraries that this library is connected to inculding id info for this lib
CREATE TABLE `libraries` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `libid` TEXT NOT NULL, -- unique id that identifies every library
    `pubkey` TEXT NOT NULL, -- public key, to connect to devices with lib data
    `privkey` TEXT, -- private key only available if the library eq home, otherwise null
    `name` TEXT NOT NULL, -- user facing name for libary
    `is_home` BOOLEAN NOT NULL DEFAULT 0
);

-- unique copy of a book, links to openlibrary data
CREATE TABLE `books` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `uid` TEXT UNIQUE NOT NULL, -- a unique edition specific random id
    `isbn` TEXT NOT NULL, -- isbn that was scaned from physical copy of book
    `edition_olid` TEXT NOT NULL,
    `authors_olid` TEXT NOT NULL,
    `works_olid` TEXT NOT NULL,
    `home_libid` TEXT NOT NULL,
    `current_libid` TEXT NOT NULL
);

-- corresponds to openlibrary editions
CREATE TABLE `editions` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    --`uid` TEXT UNIQUE NOT NULL, -- a unique edition specific random id
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
    `covers` TEXT, -- pub covers: Vec<isize>,
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
    `ia_box_id` TEXT -- pub ia_box_id: Vec<String>,
);

CREATE TABLE `authors` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `olid` TEXT NOT NULL,
    `name` TEXT NOT NULL,
    `type_field` TEXT NOT NULL, -- pub type_field: Type,
    `alternate_names` TEXT, -- pub alternate_names: Vec<String>,
    `bio` TEXT, -- pub bio: Bio,
    `birth_date` TEXT,
    `death_date` TEXT,
    `location` TEXT, -- pub location: Location, //No Idea What this looks like in real
    `date` TEXT, -- pub date: Date, //No Idea What this looks like in real
    `entity_type` TEXT, -- pub entity_type: EntityType, //No Idea What this looks like in real
    `fuller_name` TEXT, -- pub fuller_name: FullerName, //No Idea What this looks like in real
    `personal_name` TEXT, -- pub personal_name: String,
    `title` TEXT,
    `photos` TEXT, -- pub photos: Vec<usize>,
    `links` TEXT, -- pub links: Vec<Link>,
    `remote_ids` TEXT, -- pub remote_ids: RemoteIds,
    `wikipedia` TEXT, -- pub wikipedia: Wikipedia, //No Idea What this looks like in real
    `revision` TEXT NOT NULL, -- pub revision: usize,
    `latest_revision` TEXT, -- pub latest_revision: usize,
    `created` TEXT, -- pub created: Created,
    `last_modified` TEXT NOT NULL -- pub last_modified: LastModified,
);

CREATE TABLE `works` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `olid` TEXT NOT NULL,
    `title` TEXT NOT NULL,
    `subtitle` TEXT,
    `type_field` TEXT NOT NULL, -- pub type_field: Type,
    `authors` TEXT, -- pub authors: Vec<Authors>,
    `covers` TEXT, -- pub covers: Vec<isize>,
    `links` TEXT, -- pub links: Vec<Link>,
    `id_work` TEXT, -- pub id: Id, //No Idea What this looks like in real
    `lc_classifications` TEXT, -- pub lc_classifications: Vec<String>,
    `subjects` TEXT, -- pub subjects: Vec<String>,
    `first_publish_date` TEXT,
    `description` TEXT, -- pub description: Description, //It looks like OL59863W a discription type instead of a string
    `notes` TEXT,
    `revision` TEXT NOT NULL, -- pub revision: usize,
    `latest_revision` TEXT, -- pub latest_revision: usize,
    `created` TEXT, -- pub created: Created,
    `last_modified` TEXT NOT NULL -- pub last_modified: LastModified,
);
    

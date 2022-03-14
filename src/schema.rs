table! {
    authors (id) {
        id -> Integer,
        olid -> Text,
        name -> Text,
        type_field -> Text,
        alternate_names -> Nullable<Text>,
        bio -> Nullable<Text>,
        birth_date -> Nullable<Text>,
        death_date -> Nullable<Text>,
        location -> Nullable<Text>,
        date -> Nullable<Text>,
        entity_type -> Nullable<Text>,
        fuller_name -> Nullable<Text>,
        personal_name -> Nullable<Text>,
        title -> Nullable<Text>,
        photos -> Nullable<Text>,
        links -> Nullable<Text>,
        remote_ids -> Nullable<Text>,
        wikipedia -> Nullable<Text>,
        revision -> Text,
        latest_revision -> Nullable<Text>,
        created -> Nullable<Text>,
        last_modified -> Text,
    }
}

table! {
    books (id) {
        id -> Integer,
        uid -> Text,
        isbn -> Text,
        edition_olid -> Text,
        authors_olid -> Text,
        works_olid -> Text,
        home_libid -> Text,
        current_libid -> Text,
    }
}

table! {
    devices (id) {
        id -> Integer,
        devid -> Text,
        pubkey -> Text,
        privkey -> Nullable<Text>,
        name -> Text,
        home_libid -> Text,
        is_current_device -> Bool,
    }
}

table! {
    editions (id) {
        id -> Integer,
        olid -> Text,
        title -> Text,
        full_title -> Nullable<Text>,
        subtitle -> Nullable<Text>,
        type_field -> Nullable<Text>,
        authors -> Nullable<Text>,
        works -> Nullable<Text>,
        identifiers -> Nullable<Text>,
        isbn10 -> Nullable<Text>,
        isbn13 -> Nullable<Text>,
        lccn -> Nullable<Text>,
        ocaid -> Nullable<Text>,
        oclc_numbers -> Nullable<Text>,
        covers -> Nullable<Text>,
        links -> Nullable<Text>,
        languages -> Nullable<Text>,
        by_statement -> Nullable<Text>,
        weight -> Nullable<Text>,
        edition_name -> Nullable<Text>,
        number_of_pages -> Nullable<Text>,
        pagination -> Nullable<Text>,
        physical_dimensions -> Nullable<Text>,
        physical_format -> Nullable<Text>,
        publish_country -> Nullable<Text>,
        publish_date -> Nullable<Text>,
        publish_places -> Nullable<Text>,
        publishers -> Nullable<Text>,
        contributions -> Nullable<Text>,
        dewey_decimal_class -> Nullable<Text>,
        genres -> Nullable<Text>,
        lc_classifications -> Nullable<Text>,
        other_titles -> Nullable<Text>,
        series -> Nullable<Text>,
        source_records -> Nullable<Text>,
        subjects -> Nullable<Text>,
        work_titles -> Nullable<Text>,
        table_of_contents -> Nullable<Text>,
        description -> Nullable<Text>,
        first_sentence -> Nullable<Text>,
        notes -> Nullable<Text>,
        revision -> Nullable<Text>,
        latest_revision -> Nullable<Text>,
        created -> Nullable<Text>,
        last_modified -> Nullable<Text>,
        isbn_invalid -> Nullable<Text>,
        ia_box_id -> Nullable<Text>,
    }
}

table! {
    libraries (id) {
        id -> Integer,
        libid -> Text,
        pubkey -> Text,
        privkey -> Nullable<Text>,
        name -> Text,
        is_home -> Bool,
    }
}

table! {
    works (id) {
        id -> Integer,
        olid -> Text,
        title -> Text,
        subtitle -> Nullable<Text>,
        type_field -> Text,
        authors -> Nullable<Text>,
        covers -> Nullable<Text>,
        links -> Nullable<Text>,
        id_work -> Nullable<Text>,
        lc_classifications -> Nullable<Text>,
        subjects -> Nullable<Text>,
        first_publish_date -> Nullable<Text>,
        description -> Nullable<Text>,
        notes -> Nullable<Text>,
        revision -> Text,
        latest_revision -> Nullable<Text>,
        created -> Nullable<Text>,
        last_modified -> Text,
    }
}

allow_tables_to_appear_in_same_query!(authors, books, devices, editions, libraries, works,);

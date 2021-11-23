use diesel::Queryable;
use super::schema::authors;
use super::schema::books;
use super::schema::works;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub olid: String,
    pub uid: String,
    pub title: String,
    pub author: String,
    pub work: String,
}

#[derive(Queryable)]
pub struct Author {
    pub id: i32,
    pub olid: String,
    pub name: String,
}

#[derive(Queryable)]
pub struct Work {
    pub id: i32,
    pub oild: String,
    pub title: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook<'a> {
    pub olid: &'a str,
    pub uid: &'a str,
    pub title: &'a str,
    pub author: &'a str,
    pub work: &'a str,
}

#[derive(Insertable)]
#[table_name="authors"]
pub struct NewAuthor<'a> {
    pub olid: &'a str,
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name="works"]
pub struct NewWork<'a> {
    pub olid: &'a str,
    pub title: &'a str,
    pub author: &'a str,
}

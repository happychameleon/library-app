use diesel::Queryable;


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
    pub uid: String,
    pub name: String,
}

#[derive(Queryable)]
pub struct Work {
    pub id: i32,
    pub oild: String,
    pub title: String,
    pub author: String,
}

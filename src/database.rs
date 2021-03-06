// Based on https://gitlab.gnome.org/World/decoder/-/blob/master/src/database.rs

use log::debug;

use std::env;
use std::format;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

extern crate dotenv;

use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, r2d2, r2d2::ConnectionManager};
use dotenv::dotenv;
use once_cell::sync::Lazy;

use crate::config;
use crate::path;
use crate::schema::{authors, books, works};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

embed_migrations!("migrations/");

const DB_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = path::DATA.clone();
    path.push(format!("{}.sqlite", config::NAME));
    path
});

const POOL: Lazy<Pool> = Lazy::new(|| init_pool().expect("Failed to create pool"));

pub fn connection() -> Pool {
    POOL.clone()
}

pub fn clear_db() {
    debug!("Trying to run migrations to clear db");
    let conn = &connection().get().unwrap();

    diesel::delete(books::table).execute(conn).unwrap();
    diesel::delete(works::table).execute(conn).unwrap();
    diesel::delete(authors::table).execute(conn).unwrap();
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn init_pool() -> Result<Pool> {
    init_database();

    let db_path = &DB_PATH;

    debug!("Path to DB: {}", db_path.to_str().unwrap());

    let manager = ConnectionManager::<SqliteConnection>::new(db_path.to_str().unwrap());
    let pool = r2d2::Pool::builder().build(manager)?;

    {
        let db = pool.get()?;
        run_migrations(&db);
    }

    Ok(pool)
}

fn init_database() {
    path::init().expect("could not initalize directories");

    if !DB_PATH.exists() {
        File::create(&DB_PATH.to_str().unwrap()).unwrap();
    }
}

fn run_migrations(conn: &SqliteConnection) {
    embedded_migrations::run_with_output(conn, &mut std::io::stdout()).unwrap();
}

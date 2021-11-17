mod application;
mod config;
mod ui;
mod window;
mod database;
mod models;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use gettextrs::{gettext, LocaleCategory};
use gtk::{gio, glib};

use self::application::BooksApplication;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Books"));

    gtk::init().expect("Unable to start GTK4");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = BooksApplication::new();
    app.run();
}

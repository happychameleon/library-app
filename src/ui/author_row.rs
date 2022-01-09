use log::debug;

use glib::{clone, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;

use crate::models::{Author, Book};
use crate::path;

mod imp {
    use super::*;
    use glib::subclass;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/org/thievingraccoon/Books/ui/author_row.ui")]
    pub struct AuthorRow {
        #[template_child]
        pub author_name: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AuthorRow {
        const NAME: &'static str = "AuthorRow";
        type ParentType = gtk::ListBoxRow;
        type Type = super::AuthorRow;

        fn new() -> Self {
            let author_name = TemplateChild::default();

            Self { author_name }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AuthorRow {}

    impl WidgetImpl for AuthorRow {}

    impl ListBoxRowImpl for AuthorRow {}
}

glib::wrapper! {
    pub struct AuthorRow (ObjectSubclass<imp::AuthorRow>) @extends gtk::Widget, gtk::Box, gtk::ListBoxRow;
}

impl AuthorRow {
    pub fn new(author: Author) -> Self {
        let author_row = glib::Object::new::<Self>(&[]).unwrap();

        let imp = imp::AuthorRow::from_instance(&author_row);

        let author = &author.name;
        debug!("authors: {}", author);
        imp.author_name.set_label(author);

        author_row
    }
}

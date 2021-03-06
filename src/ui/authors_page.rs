use glib::{clone, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;

use crate::application::Action;
use crate::dbqueries;
use crate::models::Author;
use crate::path;
use crate::ui::author_row;

mod imp {
    use super::*;
    use adw::subclass::prelude::BinImpl;
    use glib::subclass;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/org/thievingraccoon/Books/ui/authors_page.ui")]
    pub struct AuthorsPage {
        #[template_child]
        pub authors_listbox: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AuthorsPage {
        const NAME: &'static str = "AuthorsPage";
        type ParentType = adw::Bin;
        type Type = super::AuthorsPage;

        fn new() -> Self {
            let authors_listbox = TemplateChild::default();

            Self { authors_listbox }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AuthorsPage {}

    impl WidgetImpl for AuthorsPage {}

    impl BinImpl for AuthorsPage {}
}

glib::wrapper! {
    pub struct AuthorsPage (ObjectSubclass<imp::AuthorsPage>) @extends gtk::Widget, gtk::Box;
}

impl AuthorsPage {
    pub fn init(&self, sender: Sender<Action>) {
        let imp = imp::AuthorsPage::from_instance(self);

        self.setup_widget();
    }

    fn setup_widget(&self) {
        let imp = imp::AuthorsPage::from_instance(self);

        let listbox: gtk::ListBox = imp.authors_listbox.clone().downcast().unwrap();

        let authors = dbqueries::authors().unwrap();

        for author in authors {
            let author_row = author_row::AuthorRow::new(&author);
            listbox.insert(&author_row, -1)
        }
    }

    pub fn add_author(&self, author: &Author) {
        let imp = imp::AuthorsPage::from_instance(self);

        let authors_listbox: gtk::ListBox = imp.authors_listbox.clone().downcast().unwrap();

        let author_row = author_row::AuthorRow::new(&author);
        authors_listbox.insert(&author_row, -1);
    }
}

use log::debug;

use futures::executor::block_on;
use glib::PRIORITY_DEFAULT;
use glib::{clone, MainContext, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;
use rand::distributions::Alphanumeric;
use rand::prelude::*;

use std::fs;
use std::path::PathBuf;

use openlibrary_client::{Client, CoverKey, CoverSize, Edition};

use crate::application::Action;
use crate::dbqueries;
use crate::models::Book;
use crate::path;
use crate::ui::book_cover;

mod imp {
    use super::*;
    use adw::subclass::prelude::BinImpl;
    use glib::subclass;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/org/thievingraccoon/Books/ui/books_page.ui")]
    pub struct BooksPage {
        #[template_child]
        pub books_flowbox: TemplateChild<gtk::FlowBox>,

        pub sender: OnceCell<Sender<Action>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BooksPage {
        const NAME: &'static str = "BooksPage";
        type ParentType = adw::Bin;
        type Type = super::BooksPage;

        fn new() -> Self {
            let books_flowbox = TemplateChild::default();
            let sender = OnceCell::new();

            Self {
                books_flowbox,
                sender,
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BooksPage {}

    impl WidgetImpl for BooksPage {}

    impl BinImpl for BooksPage {}
}

glib::wrapper! {
    pub struct BooksPage (ObjectSubclass<imp::BooksPage>) @extends gtk::Widget, gtk::Box;
}

impl BooksPage {
    pub fn init(&self, sender: Sender<Action>) {
        let imp = imp::BooksPage::from_instance(self);

        imp.sender.set(sender.clone()).unwrap();

        self.setup_widget(sender);
    }

    // Something does not quite work here, does not remove all children
    pub fn clear_books_page(&self) {
        let imp = imp::BooksPage::from_instance(self);
        let books_flowbox: gtk::FlowBox = imp.books_flowbox.clone().downcast().unwrap();

        let mut count = 0i32;
        count = 0;

        loop {
            let book = books_flowbox.child_at_index(count);

            match book {
                Some(book) => {
                    books_flowbox.remove(&book);
                }
                None => {
                    break;
                }
            }

            count += 1;
        }
    }

    pub fn add_book(&self, isbn: &str) {
        let imp = imp::BooksPage::from_instance(self);

        debug!("Calling add_book function");

        let books_flowbox: gtk::FlowBox = imp.books_flowbox.clone().downcast().unwrap();

        let client = Client::new();
        let image_client = Client::new();

        let entity = block_on(client.entity_by_isbn(isbn));

        let mut image_path = path::DATA.clone();
        image_path.push(format!("covers/"));
        if !image_path.exists() {
            fs::create_dir_all(image_path.clone()).unwrap();
        }
        image_path.push(format!("{}.jpg", isbn));

        let mut rng = rand::thread_rng();
        let uid: String = (&mut rng)
            .sample_iter(Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        match entity {
            Ok(entity) => {
                dbqueries::add_book(&entity, &uid);
                debug!("Adding book with uid: {}", uid);

                // let len_comp: usize = 0;

                // let cover = if entity.get_edition().covers.len() == len_comp {
                //     None
                // } else {
                //     debug!("Image cover path: {}", image_path.to_str().unwrap());
                //     match block_on(image_client.save_cover(
                //         CoverSize::L,
                //         String::from(image_path.to_str().unwrap()),
                //         CoverKey::ISBN(String::from(isbn)),
                //     )) {
                //         Ok(val) => debug!("All well"),
                //         Err(error) => debug!("{}", error),
                //     };
                //     Some(entity.get_edition().covers[0].to_string())
                // };

                let cover = book_cover::BookCover::new(dbqueries::book(&uid).unwrap());
                books_flowbox.insert(&cover, -1);
            }
            Err(error) => debug!("Failed to parse entity {} form ol: {}", isbn, error),
        };
    }

    fn setup_widget(&self, sender: Sender<Action>) {
        let imp = imp::BooksPage::from_instance(self);
        let main_context = MainContext::default();

        let books_flowbox: gtk::FlowBox = imp.books_flowbox.clone().downcast().unwrap();

        main_context.spawn_local(clone!(@weak books_flowbox => async move {

            let books = dbqueries::books();

            match books {
                Ok(books) => {
                    for book in books {
                        let cover = book_cover::BookCover::new(book);
                        books_flowbox.insert(&cover, -1);
                    }
                }
                Err(error) => debug!("Failed to get books from database: {}", error),
            }
        }));
    }
}

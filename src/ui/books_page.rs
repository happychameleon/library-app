use log::debug;

use glib::PRIORITY_DEFAULT;
use glib::{clone, MainContext, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;
use rand::prelude::*;
use rand::distributions::Alphanumeric;

use openlibrary_client::{Client, Edition};

use crate::application::Action;
use crate::ui::book_cover;
use crate::dbqueries;

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

    fn setup_widget(&self, sender: Sender<Action>) {
        let imp = imp::BooksPage::from_instance(self);
        let main_context = MainContext::default();

        let books_flowbox: gtk::FlowBox = imp.books_flowbox.clone().downcast().unwrap();

        main_context.spawn_local(clone!(@weak books_flowbox => async move {
            let client = Client::new();
            let client2 = Client::new();

            let entity = client.entity_by_isbn("9781849352826").await;
            let entity2 = client2.entity_by_isbn("9781849352826").await;

            let mut rng = rand::thread_rng();

            match entity {
                Ok(entity) => {
                    let uid: String = (&mut rng).sample_iter(Alphanumeric).take(32).map(char::from).collect();
                    dbqueries::add_book(&entity, &uid);
                    debug!("Adding book with uid: {}", uid);
                }
                Err(error) => debug!("Failed to parse entity form ol: {}", error),
            };

            match entity2 {
                Ok(entity2) => {
                    let uid: String = (&mut rng).sample_iter(Alphanumeric).take(32).map(char::from).collect();
                    dbqueries::add_book(&entity2, &uid);
                    debug!("Adding book with uid: {}", uid);
                }
                Err(error) => debug!("Failed to parse entity form ol: {}", error),
            };

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

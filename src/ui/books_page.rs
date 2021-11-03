use log::debug;

use glib::{clone, Sender, MainContext};
use glib::PRIORITY_DEFAULT;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;

use openlibrary_client::{Edition, Client};

use crate::application::Action;
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

    fn setup_widget(&self, sender: Sender<Action>) {
        let imp = imp::BooksPage::from_instance(self);
        let main_context = MainContext::default();

        let books_flowbox: gtk::FlowBox = imp.books_flowbox.clone().downcast().unwrap();
        
        main_context.spawn_local(clone!(@weak books_flowbox => async move {
            let client = Client::new();
            let client2 = Client::new();
            
            let entity = client.entity_by_isbn("9781849352826").await;
            let entity2 = client2.entity_by_isbn("9781849352826").await;


            match entity {
                Ok(entity) => {
                    let cover = book_cover::BookCover::new(entity);
                    books_flowbox.insert(&cover, -1);
                }
                Err(error) => debug!("Failed to parse entity form ol: {}", error),
            };
            
            match entity2 {
                Ok(entity2) => {
                    let cover2 = book_cover::BookCover::new(entity2);
                    books_flowbox.insert(&cover2, -1);
                }
                Err(error) => debug!("Failed to parse entity form ol: {}", error),
            };
        }));
    }
}

use glib::{clone, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;

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
        let cover = book_cover::BookCover::new();

        imp.books_flowbox.insert(&cover, -1)
    }
}

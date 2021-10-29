use glib::{clone, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;

mod imp {
    use super::*;
    use adw::subclass::prelude::BinImpl;
    use glib::subclass;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/org/thievingraccoon/Books/ui/book_form_page.ui")]
    pub struct BookFormPage {
        #[template_child]
        pub books_detail_flowbox: TemplateChild<gtk::FlowBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookFormPage {
        const NAME: &'static str = "BookFormPage";
        type ParentType = adw::Bin;
        type Type = super::BookFormPage;

        fn new() -> Self {
            let books_detail_flowbox = TemplateChild::default();

            Self {
                books_detail_flowbox,
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookFormPage {}

    impl WidgetImpl for BookFormPage {}

    impl BinImpl for BookFormPage {}
}

glib::wrapper! {
    pub struct BookFormPage (ObjectSubclass<imp::BookFormPage>) @extends gtk::Widget, gtk::Box;
}

impl BookFormPage {}

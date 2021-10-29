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
    #[template(resource = "/org/thievingraccoon/Books/ui/authors_page.ui")]
    pub struct AuthorsPage {
        #[template_child]
        pub authors_flowbox: TemplateChild<gtk::FlowBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AuthorsPage {
        const NAME: &'static str = "AuthorsPage";
        type ParentType = adw::Bin;
        type Type = super::AuthorsPage;

        fn new() -> Self {
            let authors_flowbox = TemplateChild::default();

            Self { authors_flowbox }
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

impl AuthorsPage {}

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
    #[template(resource = "/org/thievingraccoon/Books/ui/scan_book_page.ui")]
    pub struct ScanBookPage {
        #[template_child]
        pub scan_flowbox: TemplateChild<gtk::FlowBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScanBookPage {
        const NAME: &'static str = "ScanBookPage";
        type ParentType = adw::Bin;
        type Type = super::ScanBookPage;

        fn new() -> Self {
            let scan_flowbox = TemplateChild::default();

            Self { scan_flowbox }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ScanBookPage {}

    impl WidgetImpl for ScanBookPage {}

    impl BinImpl for ScanBookPage {}
}

glib::wrapper! {
    pub struct ScanBookPage (ObjectSubclass<imp::ScanBookPage>) @extends gtk::Widget, gtk::Box;
}

impl ScanBookPage {}

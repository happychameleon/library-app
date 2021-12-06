use glib::{clone, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::unsync::OnceCell;

use crate::models::Book;

mod imp {
    use super::*;
    use glib::subclass;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/org/thievingraccoon/Books/ui/book_cover.ui")]
    pub struct BookCover {
        #[template_child]
        pub cover_image: TemplateChild<gtk::Image>,
        #[template_child]
        pub book_title: TemplateChild<gtk::Label>,
        #[template_child]
        pub author_name: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookCover {
        const NAME: &'static str = "BookCover";
        type ParentType = gtk::FlowBoxChild;
        type Type = super::BookCover;

        fn new() -> Self {
            let cover_image = TemplateChild::default();
            let book_title = TemplateChild::default();
            let author_name = TemplateChild::default();

            Self {
                cover_image,
                book_title,
                author_name,
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookCover {}

    impl WidgetImpl for BookCover {}

    impl FlowBoxChildImpl for BookCover {}
}

glib::wrapper! {
    pub struct BookCover (ObjectSubclass<imp::BookCover>) @extends gtk::Widget, gtk::Box, gtk::FlowBoxChild;
}

impl BookCover {
    pub fn new(book: Book) -> Self {
        let cover = glib::Object::new::<Self>(&[]).unwrap();

        let imp = imp::BookCover::from_instance(&cover);
        imp.book_title.set_label(&book.title);
        imp.author_name.set_label(&book.author.unwrap());

        cover
    }
}

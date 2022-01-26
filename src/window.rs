use log::{debug, error};

use std::fs;
use std::path::PathBuf;

use futures::executor::block_on;
use glib::PRIORITY_DEFAULT;
use glib::MainContext;
use glib::Sender;
use glib::{clone, Value};
use glib::{GEnum, ParamSpec, ToValue};

use rand::distributions::Alphanumeric;
use rand::prelude::*;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};
use gtk_macros::*;

use adw::prelude::*;

use openlibrary_client::{Client, CoverKey, CoverSize};

use crate::application::{Action, BooksApplication, BooksView};
use crate::config::{APP_ID, PROFILE};
use crate::ui::authors_page::AuthorsPage;
use crate::ui::book_form_page::BookFormPage;
use crate::ui::books_page::BooksPage;
use crate::ui::scan_book_page::ScanBookPage;
use crate::dbqueries;
use crate::models::{Book, Edition, Author};
use crate::path;

impl Default for BooksView {
    fn default() -> Self {
        BooksView::Books
    }
}

mod imp {
    use super::*;
    use std::cell::RefCell;

    use gtk::{glib::ParamSpec, CompositeTemplate};
    use once_cell::sync::Lazy;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/org/thievingraccoon/Books/ui/window.ui")]
    pub struct BooksApplicationWindow {
        #[template_child]
        pub books_page: TemplateChild<BooksPage>,
        #[template_child]
        pub scan_book_page: TemplateChild<ScanBookPage>,
        #[template_child]
        pub authors_page: TemplateChild<AuthorsPage>,
        #[template_child]
        pub book_form_page: TemplateChild<BookFormPage>,

        #[template_child]
        pub headerbar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub view_switcher: TemplateChild<adw::ViewSwitcher>,
        #[template_child]
        pub stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub add_book: TemplateChild<gtk::Button>,
        #[template_child]
        pub to_books: TemplateChild<gtk::Button>,

        pub settings: gio::Settings,
        pub view: RefCell<BooksView>,
    }

    impl Default for BooksApplicationWindow {
        fn default() -> Self {
            Self {
                books_page: TemplateChild::default(),
                scan_book_page: TemplateChild::default(),
                authors_page: TemplateChild::default(),
                book_form_page: TemplateChild::default(),

                headerbar: TemplateChild::default(),
                view_switcher: TemplateChild::default(),
                stack: TemplateChild::default(),
                add_book: TemplateChild::default(),
                to_books: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
                view: RefCell::new(BooksView::Books),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BooksApplicationWindow {
        const NAME: &'static str = "BooksApplicationWindow";
        type Type = super::BooksApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);

            klass.install_action("win.back", None, move |obj, _, _| {
                let self_ = imp::BooksApplicationWindow::from_instance(&obj);
                self_.stack.set_visible_child_name("main");

                if self_
                    .stack
                    .visible_child_name()
                    .map(|x| x == "scan")
                    .unwrap_or(false)
                {
                    self_.scan_book_page.start();
                }
            });

            klass.install_action("win.scan-qr", None, move |obj, _, _| {
                let self_ = imp::BooksApplicationWindow::from_instance(&obj);
                self_.stack.set_visible_child_name("scan");
            });
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BooksApplicationWindow {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpec::new_enum(
                    "view",
                    "View",
                    "View",
                    BooksView::static_type(),
                    BooksView::default() as i32,
                    glib::ParamFlags::READWRITE,
                )]
            });

            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "view" => self.view.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn set_property(
            &self,
            obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &ParamSpec,
        ) {
            match pspec.name() {
                "view" => {
                    *self.view.borrow_mut() = value.get().unwrap();
                    obj.update_view();
                }
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
        }
    }

    impl WidgetImpl for BooksApplicationWindow {}
    impl WindowImpl for BooksApplicationWindow {
        // Save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            if let Err(err) = window.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request(window)
        }
    }

    impl ApplicationWindowImpl for BooksApplicationWindow {}
}

glib::wrapper! {
    pub struct BooksApplicationWindow(ObjectSubclass<imp::BooksApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl BooksApplicationWindow {
    pub fn new(sender: Sender<Action>, app: &BooksApplication) -> Self {
        let window: BooksApplicationWindow = glib::Object::new(&[("application", app)])
            .expect("Failed to create BooksApplicationWindow");

        window.setup_widgets(sender.clone());
        window.setup_gactions(sender);

        window.set_view(BooksView::Books);

        window
    }

    fn setup_gactions(&self, sender: Sender<Action>) {
        let imp = imp::BooksApplicationWindow::from_instance(self);
        let app = self.application().unwrap();

        action!(
            self,
            "add-book",
            clone!(@strong sender => move |_,_| {
                send!(sender, Action::Views(BooksView::ScanBook));
            })
        );

        action!(
            self,
            "to-books",
            clone!(@strong sender => move |_,_| {
                send!(sender, Action::Views(BooksView::Books));
            })
        );
        app.set_accels_for_action("win.to-books", &["Escape"]);
    }

    pub fn show_code_detected(&self, isbn: &str) {
        let imp = imp::BooksApplicationWindow::from_instance(self);
        //imp.books_page.add_book(isbn);

        debug!("Show code function compleated");

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

        let isbn_string = String::from(isbn);

        match entity {
            Ok(entity) => {
                dbqueries::add_book(
                    &uid,
                    &isbn_string,
                    &entity.get_olid(),
                    &entity.get_author().key,
                    &entity.get_work().key
                );

                match dbqueries::edition(&entity.get_olid()) {
                    Ok(val) => {}
                    Err(error) => {dbqueries::add_edition(&entity)}
                }
                match dbqueries::author(&entity.get_author().key) {
                    Ok(val) => {}
                    Err(error) => {
                        dbqueries::add_author(&entity);
                        let author = dbqueries::author(&entity.get_author().key).unwrap();
                        imp.authors_page.add_author(&author);
                    }
                }
                match dbqueries::work(&entity.get_work().key) {
                    Ok(val) => {}
                    Err(error) => {dbqueries::add_work(&entity);}
                }



                debug!("Adding book with uid: {}", uid);

                match entity.get_edition().covers {
                    Some(cover) => {
                        debug!("Image cover path: {}", image_path.to_str().unwrap());
                        match block_on(image_client.save_cover(
                            CoverSize::L,
                            String::from(image_path.to_str().unwrap()),
                            CoverKey::ISBN(String::from(isbn)),
                        )) {
                            Ok(val) => debug!("All well"),
                            Err(error) => debug!("{}", error),
                        };
                    }
                    None => {}
                };

                let book = dbqueries::book(&uid).unwrap();
                debug!("{}", book.edition_olid);
                let edition = dbqueries::edition(&book.edition_olid).unwrap();
                let author = dbqueries::author(&book.authors_olid).unwrap();

                imp.books_page.add_book(&book, &edition, &author);
            }
            Err(error) => debug!("Failed to parse entity {} form ol: {}", isbn, error),
        };

    }

    pub fn setup_widgets(&self, sender: Sender<Action>) {
        let imp = imp::BooksApplicationWindow::from_instance(self);

        imp.view_switcher.set_stack(Some(&imp.stack.get()));

        imp.books_page.init(sender.clone());
        imp.authors_page.init(sender.clone());

        imp.scan_book_page
            .connect_local(
                "code-detected",
                false,
                glib::clone!(@weak self as win => @default-return None, move |args| {
                    let code = args.get(1).unwrap().get::<String>().unwrap();
                    win.show_code_detected(&code);

                    None
                }),
            )
            .unwrap();
    }

    pub fn clear_books_page(&self) {
        let imp = imp::BooksApplicationWindow::from_instance(self);

        imp.books_page.clear_books_page();
    }

    pub fn set_view(&self, view: BooksView) {
        self.set_property("view", &view).unwrap()
    }

    fn update_view(&self) {
        let imp = imp::BooksApplicationWindow::from_instance(self);
        let view = *imp.view.borrow();

        match view {
            BooksView::Authors => {
                imp.stack.set_visible_child(&imp.authors_page.get());

                imp.view_switcher.set_visible(true);
                imp.to_books.set_visible(false);
                imp.add_book.set_visible(true);
            }
            BooksView::Books => {
                let scan_book_page = imp.scan_book_page.get();
                scan_book_page.stop();

                imp.stack.set_visible_child(&imp.books_page.get());

                imp.view_switcher.set_visible(true);
                imp.to_books.set_visible(false);
                imp.add_book.set_visible(true);
            }
            BooksView::Categories => {}
            BooksView::EnterBookDetails => {}
            BooksView::ScanBook => {
                let scan_book_page = imp.scan_book_page.get();
                imp.stack.set_visible_child(&scan_book_page);

                scan_book_page.start();

                imp.view_switcher.set_visible(false);
                imp.to_books.set_visible(true);
                imp.add_book.set_visible(false);
            }
        }
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let self_ = imp::BooksApplicationWindow::from_instance(self);

        let (width, height) = self.default_size();

        self_.settings.set_int("window-width", width)?;
        self_.settings.set_int("window-height", height)?;

        self_
            .settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let self_ = imp::BooksApplicationWindow::from_instance(self);

        let width = self_.settings.int("window-width");
        let height = self_.settings.int("window-height");
        let is_maximized = self_.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}

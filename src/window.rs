use glib::Sender;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use adw::prelude::*;

use crate::application::{Action, BooksApplication, BooksView};
use crate::config::{APP_ID, PROFILE};
use crate::ui::books_page::BooksPage;

impl Default for BooksView {
    fn default() -> Self {
        BooksView::Books
    }
}

mod imp {
    use super::*;
    use std::cell::RefCell;

    use gtk::{CompositeTemplate, glib::ParamSpec};
    use once_cell::sync::Lazy;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/org/thievingraccoon/Books/ui/window.ui")]
    pub struct BooksApplicationWindow {
        #[template_child]
        pub books_page: TemplateChild<BooksPage>,

        #[template_child]
        pub headerbar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub view_switcher: TemplateChild<adw::ViewSwitcher>,
        #[template_child]
        pub stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub add_book: TemplateChild<gtk::Button>,

        pub settings: gio::Settings,
        pub view: RefCell<BooksView>,
    }

    impl Default for BooksApplicationWindow {
        fn default() -> Self {
            Self {
                books_page: TemplateChild::default(),
                headerbar: TemplateChild::default(),
                view_switcher: TemplateChild::default(),
                stack: TemplateChild::default(),
                add_book: TemplateChild::default(),
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

        fn set_property(&self, obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
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
        let window: BooksApplicationWindow = glib::Object::new(&[("application", app)]).expect("Failed to create BooksApplicationWindow");

        window.setup_widgets(sender.clone());

        window.set_view(BooksView::Books);

        window
    }

    pub fn setup_widgets(&self, sender: Sender<Action>) {
        let imp = imp::BooksApplicationWindow::from_instance(self);

        imp.view_switcher.set_stack(Some(&imp.stack.get()));
    }

    pub fn set_view(&self, view: BooksView) {
        self.set_property("view", &view).unwrap()
    }

    fn update_view(&self) {
        let imp = imp::BooksApplicationWindow::from_instance(self);
        let view = *imp.view.borrow();

        match view {
            BooksView::Authors => {}
            BooksView::Books => {
                imp.stack.set_visible_child(&imp.books_page.get())
            }
            BooksView::Categories  => {}
            BooksView::EnterBookDetails => {}
            BooksView::ScanBook  => {}
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

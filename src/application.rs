use gettextrs::gettext;
use log::{debug, info};

use glib::clone;
use glib::GEnum;
use glib::{Receiver, Sender};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use gtk_macros::action;

use strum_macros::Display;
use strum_macros::EnumString;

use std::cell::RefCell;
use std::rc::Rc;

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::BooksApplicationWindow;

#[derive(Display, Copy, Debug, Clone, EnumString, PartialEq, GEnum)]
#[repr(u32)]
#[genum(type_name = "BooksBooksView")]
pub enum BooksView {
    ScanBook,
    EnterBookDetails,
    Books,
    Categories,
    Authors,
}

#[derive(Debug, Clone)]
pub enum Action {
    // Books Views
    Views(BooksView),

    // Buttons
    BackToBooks,
}

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug)]
    pub struct BooksApplication {
        pub sender: Sender<Action>,
        pub receiver: RefCell<Option<Receiver<Action>>>,

        pub window: OnceCell<WeakRef<BooksApplicationWindow>>,
    }

    impl Default for BooksApplication {
        fn default() -> Self {
            let (sender, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let receiver = RefCell::new(Some(r));

            let window = OnceCell::new();

            Self {
                sender,
                receiver,
                window,
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BooksApplication {
        const NAME: &'static str = "BooksApplication";
        type Type = super::BooksApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for BooksApplication {}

    impl ApplicationImpl for BooksApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<BooksApplication>::activate");

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.show();
                window.present();
                return;
            }
            
            let window = app.create_window();
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            let receiver = self.receiver.borrow_mut().take().unwrap();
            receiver.attach(
                None,
                clone!(@strong app => move |action| app.process_actions(action)),
            );

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<BooksApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();

            adw::init();
        }
    }

    impl GtkApplicationImpl for BooksApplication {}
}

glib::wrapper! {
    pub struct BooksApplication(ObjectSubclass<imp::BooksApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl BooksApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/org/thievingraccoon/Books/")),
        ])
        .expect("Application initialization failed...")
    }

    fn create_window(&self) -> BooksApplicationWindow {
        let imp = imp::BooksApplication::from_instance(self);
        let window = BooksApplicationWindow::new(imp.sender.clone(), &self.clone());

        window
    }

    fn main_window(&self) -> BooksApplicationWindow {
        let imp = imp::BooksApplication::from_instance(self);
        imp.window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/thievingraccoon/Books/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialogBuilder::new()
            .program_name("Books")
            .logo_icon_name(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/books/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .authors(vec!["Max Hackinger".into()])
            .artists(vec!["Max Hackinger".into()])
            .build();

        dialog.show();
    }

    pub fn run(&self) {
        info!("Books ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }

    fn process_actions(&self, action: Action) -> glib::Continue {
        let imp = imp::BooksApplication::from_instance(self);

        match action {
            Action::Views(view) => {
                imp.window.get().unwrap().upgrade().unwrap().set_view(view);
            }
            Action::BackToBooks => {}
        }

        glib::Continue(true)
    }
}

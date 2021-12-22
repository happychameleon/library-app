// https://gitlab.gnome.org/World/decoder/-/blob/master/src/widgets/camera_page.rs

use anyhow;
use ashpd::{desktop::camera, zbus};
use glib::clone;
use glib::subclass;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::os::unix::prelude::RawFd;

// use crate::screenshot;
use crate::ui::CameraPaintable;

mod imp {
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;

    use super::*;

    #[derive(Debug, CompositeTemplate, Default)]
    #[template(resource = "/org/thievingraccoon/Books/ui/scan_book_page.ui")]
    pub struct ScanBookPage {
        pub paintable: CameraPaintable,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub spinner: TemplateChild<gtk::Spinner>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScanBookPage {
        const NAME: &'static str = "ScanBookPage";
        type Type = super::ScanBookPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for ScanBookPage {
        fn constructed(&self, obj: &Self::Type) {
            self.picture.set_paintable(Some(&self.paintable));

            let callback = glib::clone!(@weak obj as page => @default-return None, move |args: &[glib::Value]| {
                let code = args.get(1).unwrap().get::<String>().unwrap();
                page.emit_by_name("code-detected", &[&code]).unwrap();
                page.stop();

                None
            });
            self.paintable
                .connect_local("code-detected", false, callback)
                .unwrap();
        }

        fn signals() -> &'static [subclass::Signal] {
            static SIGNALS: Lazy<Vec<subclass::Signal>> = Lazy::new(|| {
                vec![subclass::Signal::builder(
                    "code-detected",
                    &[String::static_type().into()],
                    glib::Type::UNIT.into(),
                )
                .flags(glib::SignalFlags::RUN_FIRST)
                .build()]
            });
            SIGNALS.as_ref()
        }
    }
    impl WidgetImpl for ScanBookPage {}
    impl BinImpl for ScanBookPage {}
}

glib::wrapper! {
    pub struct ScanBookPage(ObjectSubclass<imp::ScanBookPage>) @extends gtk::Widget, adw::Bin;
}

impl ScanBookPage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a ScanBookPage")
    }

    pub fn stop(&self) {
        let self_ = imp::ScanBookPage::from_instance(self);

        self_.paintable.close_pipeline();
    }

    pub fn start(&self) {
        let self_ = imp::ScanBookPage::from_instance(self);
        self_.spinner.start();
        self_.stack.set_visible_child_name("loading");

        let ctx = glib::MainContext::default();
        ctx.spawn_local(clone!(@weak self as camera => async move {
            let self_ = imp::ScanBookPage::from_instance(&camera);

            match camera.try_start_stream().await {
                Ok(()) => self_.stack.set_visible_child_name("stream"),
                Err(err) => {
                    self_.stack.set_visible_child_name("not-found");
                    tracing::debug!("Could not find camera: {}", err);
                }
            };
            self_.spinner.stop();
        }))
    }

    async fn try_start_stream(&self) -> anyhow::Result<()> {
        let self_ = imp::ScanBookPage::from_instance(self);
        let stream_fd = stream().await?;
        let node_id = camera::pipewire_node_id(stream_fd).await?;

        self_.paintable.set_pipewire_fd(stream_fd, node_id);

        Ok(())
    }
}

async fn stream() -> Result<RawFd, ashpd::Error> {
    let connection = zbus::Connection::session().await?;
    let proxy = camera::CameraProxy::new(&connection).await?;
    proxy.access_camera().await?;

    proxy.open_pipe_wire_remote().await
}

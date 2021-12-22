// https://gitlab.gnome.org/World/decoder/-/blob/master/src/widgets/camera_paintable.rs

// SPDX-License-Identifier: GPL-3.0-or-later
//
// Fancy Camera with QR code detection using ZBar
//
// Pipeline:
//                            queue -- videoconvert -- zbar -- fakesink
//                         /
//     pipewiresrc -- tee
//                         \
//                            queue -- videoconvert - our fancy sink

use glib::clone;
use glib::{Receiver, Sender};
use gst::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::os::unix::io::AsRawFd;

use gtk::{gdk, graphene};

mod camera_sink {
    use std::convert::AsRef;

    #[derive(Debug)]
    pub struct Frame(pub gst_video::VideoFrame<gst_video::video_frame::Readable>);

    impl AsRef<[u8]> for Frame {
        fn as_ref(&self) -> &[u8] {
            self.0.plane_data(0).unwrap()
        }
    }

    impl From<Frame> for gdk::Paintable {
        fn from(f: Frame) -> gdk::Paintable {
            let format = match f.0.format() {
                gst_video::VideoFormat::Bgra => gdk::MemoryFormat::B8g8r8a8,
                gst_video::VideoFormat::Argb => gdk::MemoryFormat::A8r8g8b8,
                gst_video::VideoFormat::Rgba => gdk::MemoryFormat::R8g8b8a8,
                gst_video::VideoFormat::Abgr => gdk::MemoryFormat::A8b8g8r8,
                gst_video::VideoFormat::Rgb => gdk::MemoryFormat::R8g8b8,
                gst_video::VideoFormat::Bgr => gdk::MemoryFormat::B8g8r8,
                _ => unreachable!(),
            };
            let width = f.0.width() as i32;
            let height = f.0.height() as i32;
            let rowstride = f.0.plane_stride()[0] as usize;

            gdk::MemoryTexture::new(
                width,
                height,
                format,
                &glib::Bytes::from_owned(f),
                rowstride,
            )
            .upcast()
        }
    }

    impl Frame {
        pub fn new(buffer: &gst::Buffer, info: &gst_video::VideoInfo) -> Self {
            let video_frame =
                gst_video::VideoFrame::from_buffer_readable(buffer.clone(), &info).unwrap();
            Self(video_frame)
        }

        pub fn width(&self) -> u32 {
            self.0.width()
        }

        pub fn height(&self) -> u32 {
            self.0.height()
        }
    }

    pub enum Action {
        FrameChanged,
    }

    use super::*;

    mod imp {
        use std::sync::Mutex;

        use gst::subclass::prelude::*;
        use gst_base::subclass::prelude::*;
        use gst_video::subclass::prelude::*;
        use once_cell::sync::Lazy;

        use super::*;

        #[derive(Default)]
        pub struct CameraSink {
            pub info: Mutex<Option<gst_video::VideoInfo>>,
            pub sender: Mutex<Option<Sender<Action>>>,
            pub pending_frame: Mutex<Option<Frame>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for CameraSink {
            const NAME: &'static str = "CameraSink";
            type Type = super::CameraSink;
            type ParentType = gst_video::VideoSink;
        }

        impl ObjectImpl for CameraSink {}
        impl ElementImpl for CameraSink {
            fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
                static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
                    gst::subclass::ElementMetadata::new(
                        "GTK Camera Sink",
                        "Sink/Camera/Video",
                        "A GTK Camera sink",
                        "Bilal Elmoussaoui <bil.elmoussaoui@gmail.com>",
                    )
                });

                Some(&*ELEMENT_METADATA)
            }

            fn pad_templates() -> &'static [gst::PadTemplate] {
                static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
                    let caps = gst_video::video_make_raw_caps(&[
                        gst_video::VideoFormat::Bgra,
                        gst_video::VideoFormat::Argb,
                        gst_video::VideoFormat::Rgba,
                        gst_video::VideoFormat::Abgr,
                        gst_video::VideoFormat::Rgb,
                        gst_video::VideoFormat::Bgr,
                    ])
                    .any_features()
                    .build();

                    vec![gst::PadTemplate::new(
                        "sink",
                        gst::PadDirection::Sink,
                        gst::PadPresence::Always,
                        &caps,
                    )
                    .unwrap()]
                });

                PAD_TEMPLATES.as_ref()
            }
        }
        impl BaseSinkImpl for CameraSink {
            fn set_caps(
                &self,
                _element: &Self::Type,
                caps: &gst::Caps,
            ) -> Result<(), gst::LoggableError> {
                let video_info = gst_video::VideoInfo::from_caps(caps).unwrap();
                let mut info = self.info.lock().unwrap();
                info.replace(video_info);

                Ok(())
            }
        }
        impl VideoSinkImpl for CameraSink {
            fn show_frame(
                &self,
                _element: &Self::Type,
                buffer: &gst::Buffer,
            ) -> Result<gst::FlowSuccess, gst::FlowError> {
                if let Some(info) = &*self.info.lock().unwrap() {
                    let frame = Frame::new(buffer, info);
                    let mut last_frame = self.pending_frame.lock().unwrap();

                    last_frame.replace(frame);
                    let sender = self.sender.lock().unwrap();

                    sender.as_ref().unwrap().send(Action::FrameChanged).unwrap();
                }
                Ok(gst::FlowSuccess::Ok)
            }
        }
    }

    glib::wrapper! {
        pub struct CameraSink(ObjectSubclass<imp::CameraSink>) @extends gst_video::VideoSink, gst_base::BaseSink, gst::Element, gst::Object;
    }
    unsafe impl Send for CameraSink {}
    unsafe impl Sync for CameraSink {}

    impl CameraSink {
        pub fn new(sender: Sender<Action>) -> Self {
            let sink = glib::Object::new(&[]).expect("Failed to create a CameraSink");
            let priv_ = imp::CameraSink::from_instance(&sink);
            priv_.sender.lock().unwrap().replace(sender);
            sink
        }

        pub fn pending_frame(&self) -> Option<Frame> {
            let self_ = imp::CameraSink::from_instance(self);
            self_.pending_frame.lock().unwrap().take()
        }
    }
}

mod imp {
    use glib::subclass;
    use std::cell::RefCell;

    use super::*;

    pub struct CameraPaintable {
        pub sink: camera_sink::CameraSink,
        pub pipeline: RefCell<Option<gst::Pipeline>>,
        pub sender: Sender<camera_sink::Action>,
        pub image: RefCell<Option<gdk::Paintable>>,
        pub size: RefCell<Option<(u32, u32)>>,
        pub receiver: RefCell<Option<Receiver<camera_sink::Action>>>,
    }

    impl Default for CameraPaintable {
        fn default() -> Self {
            let (sender, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let receiver = RefCell::new(Some(r));

            Self {
                pipeline: RefCell::default(),
                sink: camera_sink::CameraSink::new(sender.clone()),
                image: RefCell::new(None),
                sender,
                receiver,
                size: RefCell::new(None),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CameraPaintable {
        const NAME: &'static str = "CameraPaintable";
        type Type = super::CameraPaintable;
        type ParentType = glib::Object;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for CameraPaintable {
        fn constructed(&self, obj: &Self::Type) {
            obj.init_widgets();
            self.parent_constructed(obj);
        }
        fn dispose(&self, paintable: &Self::Type) {
            paintable.close_pipeline();
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

    impl PaintableImpl for CameraPaintable {
        fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
            if let Some((_, height)) = *self.size.borrow() {
                height as i32
            } else {
                0
            }
        }
        fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
            if let Some((width, _)) = *self.size.borrow() {
                width as i32
            } else {
                0
            }
        }

        fn snapshot(
            &self,
            _paintable: &Self::Type,
            snapshot: &gdk::Snapshot,
            width: f64,
            height: f64,
        ) {
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();

            if let Some(ref image) = *self.image.borrow() {
                // Transformation to avoid stretching the camera. We translate and scale the image.
                let aspect = width / height.max(std::f64::EPSILON); // Do not divide by zero.
                let image_aspect = image.intrinsic_aspect_ratio();

                if image_aspect == 0.0 {
                    image.snapshot(snapshot.upcast_ref(), width, height);
                    return;
                };

                let (new_width, new_height) = match aspect <= image_aspect {
                    true => (height * image_aspect, height), // Mobile view
                    false => (width, width / image_aspect),  // Landscape
                };

                let p = graphene::Point::new(
                    ((width - new_width) / 2.0) as f32,
                    ((height - new_height) / 2.0) as f32,
                );
                snapshot.translate(&p);

                image.snapshot(snapshot.upcast_ref(), new_width, new_height);
            } else {
                snapshot.append_color(
                    &gdk::RGBA::black(),
                    &graphene::Rect::new(0f32, 0f32, width as f32, height as f32),
                );
            }
        }
    }
}

glib::wrapper! {
    pub struct CameraPaintable(ObjectSubclass<imp::CameraPaintable>) @implements gdk::Paintable;
}

impl Default for CameraPaintable {
    fn default() -> Self {
        glib::Object::new(&[]).expect("Failed to create a CameraPaintable")
    }
}

impl CameraPaintable {
    pub fn set_pipewire_fd<F: AsRawFd>(&self, fd: F, node_id: u32) {
        let pipewire_element = gst::ElementFactory::make("pipewiresrc", None).unwrap();
        pipewire_element
            .set_property("fd", &fd.as_raw_fd())
            .unwrap();
        pipewire_element
            .set_property("path", &node_id.to_string())
            .unwrap();
        self.init_pipeline(pipewire_element);
    }

    fn init_pipeline(&self, pipewire_src: gst::Element) {
        let self_ = imp::CameraPaintable::from_instance(self);
        let pipeline = gst::Pipeline::new(None);

        let tee = gst::ElementFactory::make("tee", None).unwrap();
        let queue = gst::ElementFactory::make("queue", None).unwrap();
        let videoconvert1 = gst::ElementFactory::make("videoconvert", None).unwrap();
        let videoconvert2 = gst::ElementFactory::make("videoconvert", None).unwrap();

        let zbar = gst::ElementFactory::make("zbar", None).unwrap();
        let fakesink = gst::ElementFactory::make("fakesink", None).unwrap();
        let queue2 = gst::ElementFactory::make("queue", None).unwrap();

        pipeline
            .add_many(&[
                &pipewire_src,
                &tee,
                &queue,
                &videoconvert1,
                &zbar,
                &fakesink,
                &queue2,
                &videoconvert2,
                self_.sink.upcast_ref(),
            ])
            .unwrap();

        gst::Element::link_many(&[
            &pipewire_src,
            &tee,
            &queue,
            &videoconvert1,
            &zbar,
            &fakesink,
        ])
        .unwrap();

        tee.link_pads(None, &queue2, None).unwrap();
        gst::Element::link_many(&[&queue2, &videoconvert2, self_.sink.upcast_ref()]).unwrap();

        let bus = pipeline.bus().unwrap();
        bus.add_watch_local(
            clone!(@weak self as paintable => @default-return glib::Continue(false), move |_, msg| {
                match msg.view() {
                    gst::MessageView::Error(err) => {
                        tracing::error!(
                            "Error from {:?}: {} ({:?})",
                            err.src().map(|s| s.path_string()),
                            err.error(),
                            err.debug()
                        );
                    },
                    gst::MessageView::Element(e) => {
                        if let Some(s) = e.structure() {
                            if let Ok(symbol) = s.get::<String>("symbol") {
                                paintable.emit_by_name("code-detected", &[&symbol]).unwrap();
                            }
                        }
                    },
                    _ => (),
                }
                glib::Continue(true)
            }),
        )
        .expect("Failed to add bus watch");
        pipeline.set_state(gst::State::Playing).ok();
        self_.pipeline.replace(Some(pipeline));
    }

    pub fn close_pipeline(&self) {
        let self_ = imp::CameraPaintable::from_instance(self);
        if let Some(pipeline) = self_.pipeline.borrow_mut().take() {
            pipeline.set_state(gst::State::Null).unwrap();
        }
    }

    pub fn init_widgets(&self) {
        let self_ = imp::CameraPaintable::from_instance(self);

        let receiver = self_.receiver.borrow_mut().take().unwrap();
        receiver.attach(
            None,
            glib::clone!(@weak self as paintable => @default-panic, move |action| paintable.do_action(action)),
        );
    }

    fn do_action(&self, action: camera_sink::Action) -> glib::Continue {
        let self_ = imp::CameraPaintable::from_instance(self);
        match action {
            camera_sink::Action::FrameChanged => {
                if let Some(frame) = self_.sink.pending_frame() {
                    let (width, height) = (frame.width(), frame.height());
                    self_.size.replace(Some((width, height)));
                    self_.image.replace(Some(frame.into()));
                    self.invalidate_contents();
                }
            }
        }
        glib::Continue(true)
    }
}

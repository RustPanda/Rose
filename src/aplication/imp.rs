use adw::glib;
use adw::glib::subclass::prelude::*;
use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::window;

#[derive(Default)]
pub struct RoseApplication {}

#[glib::object_subclass]
impl ObjectSubclass for RoseApplication {
    const NAME: &'static str = "RoseApplication";
    type Type = super::RoseApplication;
    type ParentType = adw::Application;
}

impl ObjectImpl for RoseApplication {}

impl AdwApplicationImpl for RoseApplication {}

impl ApplicationImpl for RoseApplication {
    fn activate(&self) {
        tracing::debug!("AdwApplication<RoseApplication>:activate");
        self.parent_activate();
        let app = self.obj();
        let window = window::RoseApplicatioonWindow::new(app.as_ref());

        {
            let status_page = adw::StatusPage::builder()
                .vexpand(true)
                .title("Sidebar")
                .build();
            window.set_sidebar(status_page);
        }

        window.connect_new_tab(|_app, _path| {
            Some(
                adw::StatusPage::builder()
                    .vexpand(true)
                    .title("Myaw")
                    .build(),
            )
        });

        window.present()
    }
    fn startup(&self) {
        tracing::debug!("GtkApplication<RoseApplication>::startup");
        self.parent_startup();
        let app = self.obj();
        app.setup_accels();
    }
}

impl GtkApplicationImpl for RoseApplication {}

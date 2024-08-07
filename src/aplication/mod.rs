use adw::gio;
use adw::glib;
use adw::gtk;

mod imp;

glib::wrapper! {
    pub struct RoseApplication(ObjectSubclass<imp::RoseApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl RoseApplication {
    pub fn new(app_id: Option<&str>) -> Self {
        glib::Object::builder()
            .property("application-id", app_id)
            .build()
    }
}

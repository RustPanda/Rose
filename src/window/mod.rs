use adw::gio;
use adw::glib;
use adw::gtk;
use adw::prelude::*;

mod imp;

glib::wrapper! {
    pub struct RoseApplicatioonWindow(ObjectSubclass<imp::RoseApplicatioonWindow>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl RoseApplicatioonWindow {
    pub fn new(app: &impl IsA<gtk::Application>) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}

use std::cell::RefCell;

use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;

#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::RoseApplicatioonWindow)]
pub struct RoseApplicatioonWindow {
    #[property(get, set)]
    tab_view: RefCell<Option<adw::TabView>>,
    #[property(get, set)]
    expanded_sidebar: RefCell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for RoseApplicatioonWindow {
    const NAME: &'static str = "RoseApplicatioonWindow";
    type Type = super::RoseApplicatioonWindow;
    type ParentType = adw::ApplicationWindow;

    fn new() -> Self {
        Self {
            expanded_sidebar: RefCell::new(true),
            ..Default::default()
        }
    }
}
#[glib::derived_properties]
impl ObjectImpl for RoseApplicatioonWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        obj.set_default_size(800, 600);
        obj.build_window_ui();
        obj.setup_gactions();
    }
}

impl WidgetImpl for RoseApplicatioonWindow {}

impl WindowImpl for RoseApplicatioonWindow {}

impl ApplicationWindowImpl for RoseApplicatioonWindow {}

impl AdwApplicationWindowImpl for RoseApplicatioonWindow {}

impl RoseApplicatioonWindow {}

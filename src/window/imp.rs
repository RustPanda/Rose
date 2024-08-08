use std::cell::RefCell;
use std::sync::OnceLock;

use adw::glib::subclass::*;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{glib, gtk};

#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::RoseApplicatioonWindow)]
pub struct RoseApplicatioonWindow {
    #[property(get, set)]
    sidebar: RefCell<Option<gtk::Widget>>,
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

    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![Signal::builder("build-new-tab")
                .param_types([String::static_type()])
                .return_type::<gtk::Widget>()
                .build()]
        })
    }
}

impl WidgetImpl for RoseApplicatioonWindow {}

impl WindowImpl for RoseApplicatioonWindow {}

impl ApplicationWindowImpl for RoseApplicatioonWindow {}

impl AdwApplicationWindowImpl for RoseApplicatioonWindow {}

impl RoseApplicatioonWindow {}

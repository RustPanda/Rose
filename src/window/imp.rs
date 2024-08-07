use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;

#[derive(Default)]
pub struct RoseApplicatioonWindow {}

#[glib::object_subclass]
impl ObjectSubclass for RoseApplicatioonWindow {
    const NAME: &'static str = "RoseApplicatioonWindow";
    type Type = super::RoseApplicatioonWindow;
    type ParentType = adw::ApplicationWindow;
}
impl ObjectImpl for RoseApplicatioonWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        obj.set_default_size(400, 300);
        // Создаем контейнер для дочерних виджетов
        let contrnt = adw::StatusPage::builder()
            .title("Hello world")
            .vexpand(true)
            .build();

        // Устанавливаем контейнер в качестве главного виджета окна
        obj.set_content(Some(&contrnt));
    }
}

impl WidgetImpl for RoseApplicatioonWindow {}

impl WindowImpl for RoseApplicatioonWindow {}

impl ApplicationWindowImpl for RoseApplicatioonWindow {}

impl AdwApplicationWindowImpl for RoseApplicatioonWindow {}

use adw::glib::subclass::prelude::*;
use adw::subclass::prelude::*;
use adw::{gdk, prelude::*};
use adw::{gio, glib};
use vte::prelude::*;

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
            let vte = vte::Terminal::new();
            vte.set_vexpand(true);
            vte.set_margin_start(10);

            if let Some(settings) = adw::gtk::Settings::default() {
                let callback = glib::clone!(
                    #[weak]
                    vte,
                    move |settings: &adw::gtk::Settings| {
                        let theme_name = settings
                            .gtk_theme_name()
                            .expect("Could not get theme name.");

                        if theme_name.to_lowercase().contains("dark")
                            || settings.is_gtk_application_prefer_dark_theme()
                        {
                            let background = gdk::RGBA::new(0.0, 0.0, 0.0, 0.0); // черный фон
                            let foreground = gdk::RGBA::new(1.0, 1.0, 1.0, 1.0); // белый текст

                            let palette = [
                                &gdk::RGBA::new(0.0, 0.0, 0.0, 1.0),    // black
                                &gdk::RGBA::new(0.8, 0.19, 0.19, 1.0),  // red
                                &gdk::RGBA::new(0.05, 0.74, 0.47, 1.0), // green
                                &gdk::RGBA::new(0.9, 0.9, 0.06, 1.0),   // yellow
                                &gdk::RGBA::new(0.14, 0.45, 0.78, 1.0), // blue
                                &gdk::RGBA::new(0.74, 0.25, 0.74, 1.0), // magenta
                                &gdk::RGBA::new(0.07, 0.66, 0.8, 1.0),  // cyan
                                &gdk::RGBA::new(0.9, 0.9, 0.9, 1.0),    // white
                                &gdk::RGBA::new(0.4, 0.4, 0.4, 1.0),    // bright black
                                &gdk::RGBA::new(0.94, 0.3, 0.3, 1.0),   // bright red
                                &gdk::RGBA::new(0.14, 0.82, 0.54, 1.0), // bright green
                                &gdk::RGBA::new(0.96, 0.96, 0.26, 1.0), // bright yellow
                                &gdk::RGBA::new(0.23, 0.56, 0.92, 1.0), // bright blue
                                &gdk::RGBA::new(0.84, 0.44, 0.84, 1.0), // bright magenta
                                &gdk::RGBA::new(0.16, 0.72, 0.86, 1.0), // bright cyan
                                &gdk::RGBA::new(0.9, 0.9, 0.9, 1.0),    // bright white
                            ];
                            vte.set_colors(Some(&foreground), Some(&background), &palette);
                        } else {
                            let background = gdk::RGBA::new(1.0, 1.0, 1.0, 0.0); // белый фон
                            let foreground = gdk::RGBA::new(0.0, 0.0, 0.0, 1.0); // черный текст

                            let palette = [
                                &gdk::RGBA::new(0.0, 0.0, 0.0, 1.0), // black
                                &gdk::RGBA::new(0.8, 0.0, 0.0, 1.0), // red
                                &gdk::RGBA::new(0.0, 0.6, 0.0, 1.0), // green
                                &gdk::RGBA::new(0.8, 0.6, 0.0, 1.0), // yellow
                                &gdk::RGBA::new(0.0, 0.0, 0.8, 1.0), // blue
                                &gdk::RGBA::new(0.6, 0.0, 0.6, 1.0), // magenta
                                &gdk::RGBA::new(0.0, 0.6, 0.6, 1.0), // cyan
                                &gdk::RGBA::new(0.8, 0.8, 0.8, 1.0), // white
                                &gdk::RGBA::new(0.4, 0.4, 0.4, 1.0), // bright black
                                &gdk::RGBA::new(1.0, 0.4, 0.4, 1.0), // bright red
                                &gdk::RGBA::new(0.4, 1.0, 0.4, 1.0), // bright green
                                &gdk::RGBA::new(1.0, 1.0, 0.4, 1.0), // bright yellow
                                &gdk::RGBA::new(0.4, 0.4, 1.0, 1.0), // bright blue
                                &gdk::RGBA::new(1.0, 0.4, 1.0, 1.0), // bright magenta
                                &gdk::RGBA::new(0.4, 1.0, 1.0, 1.0), // bright cyan
                                &gdk::RGBA::new(0.9, 0.9, 0.9, 1.0), // bright white
                            ];
                            vte.set_colors(Some(&foreground), Some(&background), &palette);
                        }
                    }
                );

                settings.connect_gtk_application_prefer_dark_theme_notify(callback.clone());
                settings.connect_gtk_theme_name_notify(callback.clone());
                callback(&settings);
            }

            Some(vte)
        });

        window.connect_add_tab(|app, widget, path| {
            let tab_view = app.tab_view().unwrap();
            let page = tab_view.insert(widget, 0);
            page.set_title(path);

            if let Some(terminal) = widget.downcast_ref::<vte::Terminal>() {
                terminal.spawn_async(
                    vte::PtyFlags::empty(),
                    None,
                    &["/bin/zsh"],
                    &[],
                    glib::SpawnFlags::DEFAULT,
                    || {},
                    1000,
                    None::<&gio::Cancellable>,
                    glib::clone!(
                        #[weak]
                        tab_view,
                        #[weak]
                        page,
                        move |result| {
                            match result {
                                Ok(_pid) => {}
                                Err(_exit_code) => tab_view.close_page(&page),
                            };
                        }
                    ),
                );

                terminal.connect_child_exited(move |_, _exit_code| tab_view.close_page(&page));
            }
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

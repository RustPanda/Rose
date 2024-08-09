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

    pub fn connect_new_tab<T, F>(&self, callback: F)
    where
        T: IsA<gtk::Widget>,
        F: Fn(&RoseApplicatioonWindow, &str) -> Option<T> + Send + Sync + 'static,
    {
        self.connect("build-new-tab", true, move |values| {
            let app: &RoseApplicatioonWindow = values[0].get().unwrap();
            let path: &str = values[1].get().unwrap();
            callback(app, path).map(|value| value.to_value())
        });
    }

    pub fn connect_add_tab<F>(&self, callback: F)
    where
        F: Fn(&RoseApplicatioonWindow, &gtk::Widget, &str) + Send + Sync + 'static,
    {
        self.connect("add-new-tab", true, move |values| {
            let app: &RoseApplicatioonWindow = values[0].get().unwrap();
            let widget: &gtk::Widget = values[1].get().unwrap();
            let path: &str = values[2].get().unwrap();

            callback(app, widget, path);
            None
        });
    }
}

impl RoseApplicatioonWindow {
    fn build_window_ui(&self) {
        relm4_macros::view! {
            collaps_button = gtk::Button {
                set_icon_name: "go-previous-symbolic",
                set_action_name: Some("win.toggle-sidebar"),
            },
            expand_button = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                add_css_class: "toolbar",
                append = &gtk::Button {
                    set_icon_name: "go-next-symbolic",
                    set_action_name: Some("win.toggle-sidebar"),

                },
            },
            sidebar_child = adw::Bin {},
            sidebar = gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    add_css_class: "flat",
                    set_show_title: false,
                    set_show_end_title_buttons: false,

                    pack_start: &collaps_button,
                },
                append: &sidebar_child,
            },
            tab_view = adw::TabView {},
            tab_bar = adw::TabBar {
                add_css_class: "inline",
                set_autohide: false,
                set_view: Some(&tab_view)
            },
            content = gtk::Paned {
                set_position: 200,
                set_resize_start_child: false,
                set_start_child: Some(&sidebar),
                #[wrap(Some)]
                set_end_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::WindowHandle {
                        set_hexpand: true,
                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_valign: gtk::Align::Center,
                            append: &expand_button,
                            append: &tab_bar,

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                add_css_class: "toolbar",
                                append = &gtk::Button {
                                    set_icon_name: "tab-new-symbolic",
                                    set_action_name: Some("win.new-tab"),
                                },
                                append = &gtk::WindowControls {
                                    set_side: gtk::PackType::End,
                                }
                            }
                        }
                    },
                    append: &tab_view,
                }
            },
        }
        self.set_content(Some(&content));
        self.set_tab_view(tab_view);

        self.bind_property("sidebar", &sidebar_child, "child")
            .sync_create()
            .build();

        self.bind_property("expanded-sidebar", &sidebar, "visible")
            .sync_create()
            .build();
        self.bind_property("expanded-sidebar", &expand_button, "visible")
            .invert_boolean()
            .sync_create()
            .build();
    }

    fn setup_gactions(&self) {
        let action_new_tab: gio::ActionEntry<RoseApplicatioonWindow> =
            gio::ActionEntry::builder("new-tab")
                .activate(|app: &RoseApplicatioonWindow, _, _parameter| {
                    let wiget = app.emit_by_name::<gtk::Widget>("build-new-tab", &[&"Maria"]);
                    app.emit_by_name::<()>("add-new-tab", &[&wiget, &"Maria"]);
                })
                .build();
        let toggle_sidebar: gio::ActionEntry<RoseApplicatioonWindow> =
            gio::ActionEntry::builder("toggle-sidebar")
                .activate(|app: &RoseApplicatioonWindow, _, _| {
                    app.set_expanded_sidebar(!app.expanded_sidebar());
                })
                .build();

        self.add_action_entries([action_new_tab, toggle_sidebar]);
    }
}

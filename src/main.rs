use adw::glib;
use adw::prelude::*;

mod aplication;
mod window;

const APP_ID: &'static str = "com.mgolubbev.Rose";

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt::init();

    let app = aplication::RoseApplication::new(Some(APP_ID));
    app.run()
}

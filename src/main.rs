mod app;
mod config;
mod pages;

use gtk::gio;
use relm4::{adw, gtk, RelmApp};

use app::AppModel;

fn main() {
    let res = gio::Resource::load(config::PKGDATA_DIR.to_owned() + "/resources.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let application = adw::Application::builder()
        .application_id("indi.lomirus.gtk-qq")
        .build();

    let model = AppModel::new();
    let app = RelmApp::with_app(model, application);
    app.run()
}

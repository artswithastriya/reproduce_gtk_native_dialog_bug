use gio::SimpleAction;
use glib::clone;
use gtk::{gio, glib, prelude::*, Application, ApplicationWindow, Builder};

fn main() {
    let app = Application::new(
        Some("io.fortressia.repro_native_bug"),
        gio::ApplicationFlags::empty(),
    );

    app.connect_activate(|app| {
        let menubar: gio::Menu = Builder::from_string(include_str!("repro.ui"))
            .object("menu")
            .expect("No menu in builder");

        let window: ApplicationWindow = ApplicationWindow::new(app);

        app.set_menubar(Some(&menubar));
        window.set_show_menubar(true);

        {
            let action_open_file: SimpleAction = SimpleAction::new("open-file", None);

            window.add_action(&action_open_file);
            action_open_file.connect_activate(clone!(@weak window =>
                move |_action, _variant| {
                    let dialog = gtk::FileChooserNative::new(
                        Some("Open File"),
                        Some(&window),
                        gtk::FileChooserAction::Open,
                        Some("Open"),
                        Some("Cancel"),
                    );

                    dialog.connect_response(|dialog, response| {
                        println!("{:#?} {:#?}", dialog, response);
                    });

                    dialog.show();
            }));
        }
        window.show();
    });

    std::process::exit(app.run());
}

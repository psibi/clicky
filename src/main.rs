extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;

use gtk::{Application, ApplicationWindow, Button};

fn main() -> Result<(), Box<std::error::Error>> {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())?;

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");

        use std::io::Write;
        let file = std::fs::File::create("mylog.txt").unwrap();
        let file = RefCell::new(file);

        button.connect_clicked(
            move |_| match file.borrow_mut().write_all(b"I was clicked.\n") {
                Ok(()) => (),
                Err(e) => eprintln!("Error writing to file: {}", e),
            },
        );
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
    Ok(())
}

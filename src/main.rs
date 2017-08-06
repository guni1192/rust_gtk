extern crate gtk;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello, World");

    // ウインドウを閉じたときに終了する。
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    //ウィジットを格納するコンテナを用意
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let label = gtk::Label::new("Hoge");
    hbox.pack_start(&label, true, true, 5);

    let entry = gtk::Entry::new();
    hbox.pack_start(&entry, true, true, 5);

    let button = gtk::Button::new_with_label("Say");

    let window_ = window.clone();
    button.connect_clicked(move |_| {
        let message = format!("Text {}", entry.get_text().unwrap());

        let dialog = gtk::MessageDialog::new(
                Some(&window_),
                gtk::DialogFlags::empty(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                &message
            );
        dialog.run();
        dialog.destroy();

        println!("Text: {}", entry.get_text().unwrap());
        label.set_label(&entry.get_text().unwrap());
    });
    hbox.pack_start(&button, false, false, 5);

    window.add(&hbox);
    window.show_all();

    gtk::main();
}

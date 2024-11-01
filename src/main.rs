use gtk::prelude::*;
use gtk::{Button, Label, Orientation, Box};

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create main window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Welcome to the Sailfish OS HADK GUI");
    window.set_default_size(300, 200);

    // Create a box to add widgets vertically
    let vbox = Box::new(Orientation::Vertical, 5);
    window.add(&vbox);

    // Create a label
    let label = Label::new(Some("Choose an alternative:"));
    vbox.pack_start(&label, true, true, 0);

    // Create "Back" button
    let back_button = Button::with_label("Back");
    vbox.pack_start(&back_button, true, true, 0);
    
    // Create "Next" button
    let next_button = Button::with_label("Next");
    vbox.pack_start(&next_button, true, true, 0);

    // Add click handler för the buttons
    back_button.connect_clicked(move |_| {
        label.set_text("Back button was pressed!");
    });

    next_button.connect_clicked(move |_| {
        label.set_text("Next button was pressed!");
    });

    // Handle closing of the window
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all widgets
    window.show_all();

    // Run GTK head loop
    gtk::main();
}

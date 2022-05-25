use std::{cell::Cell, rc::Rc};

use glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Orientation};
use gtk::glib;

#[macro_use]
extern crate tarator;

APPLICATION_DECLARE!(Garlic);
impl Application for Garlic {
    APPLICATION_LAYERIMPL!(Garlic);
    fn new() -> Garlic {
        return Garlic {
            layer_stack: UPtr::new(LayerStack::new())
        }
    }
    fn run(&mut self) {
        // Create a new application
        let app = gtk::Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

        // Connect to "activate" signal of `app`
        app.connect_activate(build_ui);
            
        // Run the application
        app.run();
    }
}

fn build_ui(app: &gtk::Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Reference-counted object with inner mutability
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}

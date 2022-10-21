use gtk::{prelude::*, Align, Box as GtkBox, Button, Entry, Orientation};
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Arifs Cool Display Manager")
            .default_width(350)
            .default_height(70)
            .build();

        window.fullscreen();

        let username_input = GtkBox::builder()
            .visible(true)
            .can_focus(false)
            .height_request(40)
            .width_request(285)
            .build();
        username_input.append(
            &Entry::builder()
                .placeholder_text("Username")
                .height_request(40)
                .width_request(285)
                .build(),
        );
        let password_input = GtkBox::builder()
            .visible(true)
            .can_focus(false)
            .height_request(40)
            .width_request(285)
            .build();
        password_input.append(
            &Entry::builder()
                .placeholder_text("Password")
                .height_request(40)
                .width_request(285)
                .build(),
        );

        let login_button = GtkBox::builder()
            .visible(true)
            .can_focus(false)
            .height_request(40)
            .width_request(285)
            .build();
        login_button.append(
            &Button::builder()
                .label("Login")
                .height_request(40)
                .width_request(285)
                .build(),
        );

        let win = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .visible(true)
            .can_focus(false)
            .spacing(10)
            .build();

        win.append(&username_input);
        win.append(&password_input);
        win.append(&login_button);

        window.set_child(Some(&win));
        window.show();
    });

    application.run();
}

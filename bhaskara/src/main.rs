use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive::event::Key;

fn main() {
    // Creates the cursive root - required for every application
    let mut siv = Cursive::default();

    // Quit when "Esc" is pressed
    siv.add_global_callback(Key::Esc, |s| s.quit());

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Hello, World!"))
        .title("Bhaskara Calculator")
        .button("Quit", |s| s.quit()));

//    siv.add_layer(TextView::new("Press <Esc> to quit."));

    // Starts the event loop
    siv.run();
}

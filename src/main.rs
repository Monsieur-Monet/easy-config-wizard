use webbrowser; // Assuming sidebar.slint is in the same directory

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_button_pressed(move || {
        webbrowser::open("https://www.rust-lang.org").unwrap();
    });

    ui.on_button2_pressed(move || {
        webbrowser::open("https://www.google.com").unwrap();
    });

    ui.on_button3_pressed(move || {
        webbrowser::open("https://www.github.com").unwrap();
    });

    ui.run()
}

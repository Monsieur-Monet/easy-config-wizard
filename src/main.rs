use webbrowser;

slint::include_modules!();

fn open_url(url: &str) {
    if webbrowser::open(url).is_err() {
        println!("Failed to open url: {}", url);
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    open_url("https://www.rust-lang.org/learn/get-started");

    ui.run()
}

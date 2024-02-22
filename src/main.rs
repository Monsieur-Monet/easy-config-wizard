use webbrowser;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let welcome = AppWindow::new()?;

    welcome.run()
}

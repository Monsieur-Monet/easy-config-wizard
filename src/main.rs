use webbrowser;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    use std::fs;
    let ui = AppWindow::new()?;

    // This needs to be refactored big time lol

    let ui_weak1 = ui.as_weak();
    let ui_weak2 = ui.as_weak();
    let ui_weak3 = ui.as_weak();
    let ui_weak4 = ui.as_weak();

    ui.on_button1_pressed(move || {
        let ui = ui_weak1.upgrade().unwrap();
        let mut value = ui.get_currentView();
        value = 1;
        ui.set_currentView(value);
    });

    ui.on_button2_pressed(move || {
        let ui = ui_weak2.upgrade().unwrap();
        let mut value = ui.get_currentView();
        value = 2;
        ui.set_currentView(value);
    });

    ui.on_button3_pressed(move || {
        let ui = ui_weak3.upgrade().unwrap();
        let mut value = ui.get_currentView();
        value = 3;
        ui.set_currentView(value);
    });

    ui.on_button4_pressed(move || {
        let ui = ui_weak4.upgrade().unwrap();
        let mut value = ui.get_currentView();
        value = 4;
        ui.set_currentView(value);
    });

    ui.on_button5_pressed(move || {
        webbrowser::open("https://swaywm.org/").unwrap();
    });

    ui.on_button6_pressed(move || {
        webbrowser::open("https://github.com/Monsieur-Monet/easy-config-wizard").unwrap();
    });

    ui.on_button7_pressed(move || {
        let data = fs::read_to_string("/home/timo/.config/sway/config").expect("Unable to read file");
        println!("{}", data);
    });

    ui.run()
}

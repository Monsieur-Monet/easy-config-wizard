use webbrowser;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    use std::fs;
    use std::fs::read_to_string;
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
        let data =
            fs::read_to_string("/home/timo/.config/sway/config").expect("Unable to read file");
        println!("{}", data);
    });

    ui.on_button8_pressed(move || {
        fn read_lines(filename: &str) -> Vec<String> {
            read_to_string(filename)
                .unwrap()
                .lines()
                .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
                .map(|line| line.trim_start().to_string())
                .collect()
        }

        println!("Printing lines from sway config");
        println!("");
        println!("###############################################");
        println!("Keybindings:");
        println!("###############################################");
        println!("");
        for line in read_lines("/home/timo/.config/sway/config") {
            if line.starts_with("bindsym") {
                println!("{}", line);
            }
        }
        println!("");
        println!("###############################################");
        println!("Variables:");
        println!("###############################################");
        println!("");
        for line in read_lines("/home/timo/.config/sway/config") {
            if line.starts_with("set") {
                println!("{}", line);
            }
        }
        println!("");
        println!("###############################################");
        println!("Startup Programs:");
        println!("###############################################");
        println!("");
        for line in read_lines("/home/timo/.config/sway/config") {
            if line.starts_with("exec") || !line.starts_with("set") && line.ends_with("\\") {
                // This is a janky way from preventing a line from the Variables section to be printed
                println!("{}", line);
            }
        }
        println!("");
        println!("###############################################");
        println!("Window Borders:");
        println!("###############################################");
        println!("");
        for line in read_lines("/home/timo/.config/sway/config") {
            if line.contains("default") && line.contains("border") || line.contains("focused") {
                println!("{}", line);
            }
        }
        println!("");
        println!("###############################################");
        println!("Window Gaps:");
        println!("###############################################");
        println!("");
        for line in read_lines("/home/timo/.config/sway/config") {
            if line.contains("gaps") {
                println!("{}", line);
            }
        }
        println!("");
        println!("###############################################");
        println!("Keyboard Layout:");
        println!("###############################################");
        println!("");
        for line in read_lines("/home/timo/.config/sway/config") {
            if line.contains("xkb_layout") {
                println!("{}", line);
            }
        }
    });

    ui.run()
}

// ToDo:
// - Add a way to change the config file path
// - Find a way to get determine what kind of value is being set
// - Find a way to set different value

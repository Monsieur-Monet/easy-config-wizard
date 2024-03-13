use std::array;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    use std::fs::read_to_string;
    let ui = AppWindow::new()?;

    let mut keybindings: Vec<String> = Vec::new();
    let mut variables: Vec<String> = Vec::new();
    let mut startup_programs: Vec<String> = Vec::new();
    let mut window_borders: Vec<String> = Vec::new();
    let mut window_gaps: Vec<String> = Vec::new();
    let mut keyboard_layout: Vec<String> = Vec::new();

    fn read_lines(filename: &str) -> Vec<String> {
        read_to_string(filename)
            .unwrap()
            .lines()
            .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
            .map(|line| line.trim_start().to_string())
            .collect()
    }

    for line in read_lines("/home/timo/.config/sway/config") {
        if line.starts_with("bindsym") {
            keybindings.push(line.to_string());
        }
    }

    for line in read_lines("/home/timo/.config/sway/config") {
        if line.starts_with("set") {
            variables.push(line.to_string());
        }
    }

    for line in read_lines("/home/timo/.config/sway/config") {
        if line.starts_with("exec") || !line.starts_with("set") && line.ends_with("\\") {
            // This is a janky way from preventing a line from the Variables section to be printed
            startup_programs.push(line.to_string());
        }
    }

    for line in read_lines("/home/timo/.config/sway/config") {
        if line.contains("default") && line.contains("border") || line.contains("focused") {
            window_borders.push(line.to_string());
        }
    }

    for line in read_lines("/home/timo/.config/sway/config") {
        if line.contains("gaps") {
            window_gaps.push(line.to_string());
        }
    }

    for line in read_lines("/home/timo/.config/sway/config") {
        if line.contains("xkb_layout") {
            keyboard_layout.push(line.to_string());
        }
    }

    ui.on_button1_pressed(move || {
        println!("Keybindings: {:?}", keybindings);
        println!("Variables: {:?}", variables);
        println!("Startup Programs: {:?}", startup_programs);
        println!("Window Borders: {:?}", window_borders);
        println!("Window Gaps: {:?}", window_gaps);
        println!("Keyboard Layout: {:?}", keyboard_layout);
    });

    ui.run()
}

// ToDo:
// - Add a way to change the config file path
// - Find a way to get determine what kind of value is being set
// - Find a way to set different value

//#![windows_subsystem = "windows"]

extern crate libui;
use libui::controls::*;
use libui::prelude::*;

use std::process::{Command, Stdio};
use std::{
    env, error::Error, fs::File, io::{self, prelude::*}, path::PathBuf, process, str::from_utf8
};



struct HotkeyHandler {
    filepath: Option<PathBuf>,
    textfield: MultilineEntry,
}

impl AreaHandler for HotkeyHandler{
    fn key_event(&mut self, area: &Area, event: &AreaKeyEvent) -> bool {
        if event.up {
            return false;
        }
        match (event.key, event.modifier) {
            (b's', Modifiers::MODIFIER_CTRL) => {
                save(&self.filepath, self.textfield.value());
                true
            }
            _ => false
        }
    }
}




fn main() -> Result<(), Box<dyn Error>> {

    let input_file = get_filepath();

    create_window(input_file);

    Ok(())
}

fn create_window(filepath: Option<PathBuf>) {
    let ui = UI::init()
        .expect("Couldn't initialize UI library");
    

    libui::menu! { &ui,
        let menu_file = Menu("File") {
            let menu_file_reload = MenuItem("Reload")
            let menu_file_save = MenuItem("Save")
            let menu_file_save_as = MenuItem("Save As")
        }
    }
    
    let mut win = Window::new(&ui, "gpgsef", 900, 400, WindowType::HasMenubar);
    let mut layout = VerticalBox::new();

    let mut textfield: MultilineEntry = MultilineEntry::new();
    // textfield.set_readonly(true);
    // textfield.set_value(&decrypt(&filepath));
    textfield.set_value("File -> Reload");

    menu_file_reload.on_clicked({
        let filepath = filepath.clone();
        let mut textfield = textfield.clone();
        move |_, _| {
            textfield.set_value(&decrypt(&filepath));
        }
    });

    menu_file_save.on_clicked({
        let filepath = filepath.clone();
        let textfield = textfield.clone();
        move |_, _| {
            save(&filepath, textfield.value());
        }
    });

    menu_file_save_as.on_clicked({
        let textfield = textfield.clone();
        let win = win.clone();
        move |_, _| {
            save(&win.save_file(),textfield.value());
        }
    });

    layout.append(textfield, LayoutStrategy::Stretchy);
    win.set_child(layout);
    win.show();
    ui.main();
}

fn decrypt(filepath: &Option<PathBuf>) -> String {
    match filepath {
        Some(filepath) => {
            match process::Command::new("gpg")
            .arg("-d").arg("-q").arg("--no-symkey-cache").arg(filepath)
            .output() {
                Ok(output) => {
                    let stdout = match String::from_utf8(output.stdout) {
                        Ok(string) => string,
                        Err(err) => format!("ERROR: {}", err.to_string())
                    };
                    let stderr = match String::from_utf8(output.stderr) {
                        Ok(string) => string,
                        Err(err) => format!("ERROR: {}", err.to_string())
                    };
                    format!("{}{}{}", stdout, if stdout.len()==0 || stderr.len()==0 {""} else {"\n\n"}, stderr)
                },
                Err(err) => format!("An error occured while decrypting the file: {}", err.to_string())
            }
        },
        None => "No file path supplied".to_owned()
    }
}

fn save(file: &Option<PathBuf>, data: String) {
    match file {
        Some(path) => {
            let file = path.to_str().unwrap();
            let mut gpg = Command::new("gpg")
            .args(["-c", "--no-symkey-cache", "--yes", "-o", file])
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn().expect("Failed to start gpg to encrypt the file");
            gpg.stdin.as_mut().unwrap().write_all(data.as_bytes()).expect("msg");
        },
        None => println!("No filename")
    }
}

fn get_filepath() -> Option<PathBuf> {
    match env::args().nth(1) {
        Some(fname) => Some(PathBuf::from(fname)),
        None => None
    }
}
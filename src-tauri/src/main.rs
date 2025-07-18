// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, process::{exit, Command}};

use tauri::Manager;

#[tauri::command]
fn decrypt() -> String {
  if cfg!(debug_assertions) {
    return String::from("I KNOW YOUR SECRETS");
  }

  let file = match env::args().nth(1) {
    Some(something) => something,
    None => return "No filename supplied".to_owned()
  };
  let output = Command::new("gpg")
    .arg("--yes").arg("--batch").arg("--passphrase-fd").arg("0").arg("--no-symkey-cache").arg("-d").arg(&file)
    .output().expect("msg");
  let stdout = match String::from_utf8(output.stdout) {
    Ok(ok) => ok,
    Err(err) => {
      println!("An error occured while parsing gpg stdout: {}", err);
      "STDOUT ERROR".to_owned()
    }
  };
  let stderr = match String::from_utf8(output.stderr) {
    Ok(ok) => ok,
    Err(err) => {
      println!("An error occured while parsing gpg stderr: {}", err);
      "STDERR ERROR".to_owned()
    }
  };
  format!("{}{}{}", stdout, if stdout.len()==0 || stderr.len()==0 {""} else {"\n\n"}, stderr)
}

#[tauri::command]
fn printit(string: &str) {
  println!("{}", string);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    tauri::Builder::default()
      .plugin(tauri_plugin_opener::init())
      .invoke_handler(tauri::generate_handler![decrypt, printit])
      .setup(|app| {
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
          let window = app.get_webview_window("main").unwrap();
          window.open_devtools();
        }
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

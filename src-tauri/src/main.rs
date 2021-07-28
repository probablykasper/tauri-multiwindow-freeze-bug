#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

mod cmd;

fn main() {
  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    .create_window("main".to_string(), WindowUrl::default(), |win, webview| {
      let win = win
        .title("Main")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(800.0, 600.0)
        .min_inner_size(300.0, 150.0)
        .fullscreen(false);
      return (win, webview);
    })
    .invoke_handler(tauri::generate_handler![cmd::error_popup, cmd::new_window])
    .run(ctx)
    .expect("error while running tauri app");
}

use tauri::api::dialog;
use tauri::{command, AppHandle, WindowBuilder, WindowUrl};

#[command]
pub fn error_popup(msg: String) {
  println!("Error popup: {}", msg);
  dialog::message("Error", msg);
}

#[command]
pub fn new_window(app_handle: AppHandle) -> Result<(), String> {
  println!("new w");
  app_handle
    .create_window("main2".to_string(), WindowUrl::default(), |win, webview| {
      let win = win
        .title("Main 2")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(800.0, 600.0)
        .min_inner_size(300.0, 150.0)
        .fullscreen(false);
      println!("- struct created");
      return (win, webview);
    })
    .expect("Error creating window");
  println!("- done");
  Ok(())
}

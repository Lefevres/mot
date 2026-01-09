// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use librairie::main as main_mot;



#[tauri::command]
fn say_hello(name: String) -> String {
  format!("Hello, {} 👋 depuis Rust", name)
}


#[tauri::command]
fn commence(){
  main_mot();
}


fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![say_hello])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}


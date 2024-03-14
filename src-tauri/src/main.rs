// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{self, Receiver, Sender};

pub(crate) mod audio;
pub(crate) mod track;

pub struct TestState {
    hello_tx: Sender<()>,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let _ = std::thread::spawn(move || {
        say_hello(rx);
    });
    tauri::Builder::default()
        .manage(TestState { hello_tx: tx })
        .invoke_handler(tauri::generate_handler![audio::play_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn say_hello(rx: Receiver<()>) {
    for _ in rx.recv() {
        println!("hello!");
    }
}

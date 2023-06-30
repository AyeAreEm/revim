// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use std::fs;
use std::env;
use enigo::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save_file(path: String, contents: String) {
    fs::write(path, contents).unwrap();
}

#[tauri::command]
fn move_direction(direction: String) {
    let mut enigo = Enigo::new();

    let dir = direction.as_str();

    match dir {
        "left" => enigo.key_down(Key::LeftArrow),
        "right" => enigo.key_down(Key::RightArrow),
        "up" => enigo.key_down(Key::UpArrow),
        "down" => enigo.key_down(Key::DownArrow),
        "top" => {
            enigo.key_down(Key::Meta);
            enigo.key_click(Key::UpArrow);
            enigo.key_up(Key::Meta);
        },
        "bottom" => {
            enigo.key_down(Key::Meta);
            enigo.key_click(Key::DownArrow);
            enigo.key_up(Key::Meta);
        },
        _ => println!("not a direction")
    }
}

#[tauri::command]
fn new_line() {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Return);

    // match direction {
    //     "down" => {
    //         enigo.key_down(Key::Meta);
    //         enigo.key_click(Key::Return);
    //         enigo.key_up(Key::Meta);
    //     },
    //     &_ => println!("didn't really work out")
    // }
}

#[tauri::command]
fn backspace() {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Backspace)
}

#[tauri::command]
fn paste() {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Meta);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Meta);
}

fn main() {
    tauri::Builder::default()
        // .menu(menu)
        .invoke_handler(tauri::generate_handler![save_file, move_direction, new_line, backspace, paste])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

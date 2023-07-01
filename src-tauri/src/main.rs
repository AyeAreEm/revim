// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use std::fs;
use std::env;
// use std::process::Command;
use enigo::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save_file(path: String, contents: String) {
    fs::write(path, contents).unwrap();
}

#[tauri::command]
fn move_direction(direction: &str) {
    let mut enigo = Enigo::new();

    match direction {
        "left" => enigo.key_down(Key::LeftArrow),
        "right" => enigo.key_down(Key::RightArrow),
        "up" => enigo.key_down(Key::UpArrow),
        "down" => enigo.key_down(Key::DownArrow),
        "top" => {
            if env::consts::OS == "macos" {
                enigo.key_down(Key::Meta);
                enigo.key_click(Key::UpArrow);
                enigo.key_up(Key::Meta);
                return;
            }

            enigo.key_down(Key::Control);
            enigo.key_click(Key::UpArrow);
            enigo.key_up(Key::Control);
        },
        "bottom" => {
            if env::consts::OS == "macos" {
                enigo.key_down(Key::Meta);
                enigo.key_click(Key::DownArrow);
                enigo.key_up(Key::Meta);
                return;
            }

            enigo.key_down(Key::Control);
            enigo.key_click(Key::DownArrow);
            enigo.key_up(Key::Control);
        },
        _ => println!("not a direction")
    }
}

#[tauri::command]
fn new_line(direction: &str) {
    let mut enigo = Enigo::new();

    if direction == "down" && env::consts::OS == "macos" {
        enigo.key_down(Key::Meta);
        enigo.key_click(Key::RightArrow);
        enigo.key_up(Key::Meta);
        
        enigo.key_click(Key::Return);
    } else if direction == "up" && env::consts::OS == "macos" {
        enigo.key_down(Key::Meta);
        enigo.key_click(Key::LeftArrow);
        enigo.key_up(Key::Meta);

        enigo.key_click(Key::Return);
        enigo.key_click(Key::UpArrow);
    } else if direction == "down" && env::consts::OS == "windows" {
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Return);
        enigo.key_up(Key::Control);
    } else if direction == "down" && env::consts::OS == "windows" {
        enigo.key_down(Key::Shift);
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Return);
        enigo.key_up(Key::Control);
        enigo.key_down(Key::Shift);
    }
}

#[tauri::command]
fn backspace() {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Backspace)
}

#[tauri::command]
fn copy_paste(edit: char) {
    let mut enigo = Enigo::new();
    
    if env::consts::OS == "macos" {
        enigo.key_down(Key::Meta);
        enigo.key_click(Key::Layout(edit));
        enigo.key_up(Key::Meta);
        return;
    } 

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout(edit));
    enigo.key_up(Key::Control);

}

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
  tauri::WindowBuilder::new(
    &handle,
    "external", /* the unique window label */
    tauri::WindowUrl::External("https://127.0.0.1:1420".parse().unwrap())
  ).build().unwrap();
}

#[tauri::command]
fn open_term() {
    println!("ran open term command");
}

fn main() {
    tauri::Builder::default()
        // .menu(menu)
        .invoke_handler(tauri::generate_handler![save_file, move_direction, new_line, backspace, copy_paste, open_docs, open_term])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

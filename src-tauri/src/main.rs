// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use std::fs;
use std::env;
use std::process::Command;
use enigo::*;
use open;

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
        // can't figure out how to get this working on windows
        enigo.key_click(Key::Return);
    } else if direction == "up" && env::consts::OS == "windows" {
        // can't figure out how to get this working on windows
        enigo.key_click(Key::Return);
        enigo.key_click(Key::UpArrow);
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
async fn phind_window(handle: tauri::AppHandle) {
    tauri::WindowBuilder::new(
        &handle,
        "external", /* the unique window label */
        tauri::WindowUrl::External("https://www.phind.com".parse().unwrap())
    ).build().unwrap();
}

#[tauri::command]
fn help_page() {
    // "https://github.com/AyeAreEm/revim"
    let _ = open::that("https://github.com/AyeAreEm/revim");
}

fn terminal() {
    if env::consts::OS == "windows" {
        Command::new("cmd")
                .spawn()
                .expect("failed");
        return;
    }

    Command::new("sh")
            .spawn()
            .expect("failed");
}

#[tauri::command]
fn open_term() {
    terminal();
}

fn main() {
    tauri::Builder::default()
        // .menu(menu)
        .invoke_handler(tauri::generate_handler![save_file, move_direction, new_line, backspace, copy_paste, phind_window, open_term, help_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

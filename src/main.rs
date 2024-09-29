// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod raw_info;
pub mod remote_media_info;

use {
    tauri::webview_version,
    windows::{
        w,
        Win32::{
            Foundation::HWND,
            System::Console::GetConsoleWindow,
            UI::{
                Input::KeyboardAndMouse::GetActiveWindow,
                WindowsAndMessaging::{MessageBoxW, MESSAGEBOX_STYLE},
            },
        },
    },
    winver::WindowsVersion,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn extract_media_url(url: &str) -> std::result::Result<String, String> {
    use std::str::FromStr;

    let url = match String::from_str(url) {
        Ok(s) => s,
        Err(e) => unreachable!("{e:?}, &str to String conversion is Infallible"),
    };
    let raw = match remote_media_info::fetch_info_from_url(url) {
        Ok(raw) => raw,
        Err(e) => Err(format!("{e:?}"))?,
    };
    let info = match remote_media_info::process_raw_info(raw) {
        Ok(info) => info,
        Err(e) => Err(format!("{e:?}"))?,
    };
    Ok(info.format[0].url.clone())
}

fn main() {
    let wnd = {
        let console_window = unsafe { GetConsoleWindow() };
        // DANGER: this currently checks against a built-in
        // sentinal value that essentially evaluates to NULL.
        // as long as this stays true, then this check is always
        // safe and correct.
        if console_window != HWND(-1) {
            console_window
        } else {
            let active_window = unsafe { GetActiveWindow() };
            if active_window != HWND(-1) {
                active_window
            } else {
                return;
            }
        }
    };

    let version = WindowsVersion::detect().unwrap();
    if version <= WindowsVersion::new(10, 0, 0) {
        unsafe {
            MessageBoxW(
                wnd,
                w!("Windows 10 or higher is required to run this application."),
                w!("Cinema"),
                MESSAGEBOX_STYLE(0x00000000 | 0x00000010 | 0x00002000),
            );
        };
        return;
    }

    match webview_version() {
        Err(e) => {
            unsafe {
                MessageBoxW(
                    wnd,
                    w!("The WebView2 runtime could not be found or is not installed. Please install the Microsoft WebView2 Runtime."),
                    w!("Cinema"),
                    MESSAGEBOX_STYLE(0x00000000 | 0x00000010 | 0x00002000),
                );
            };
            return;
        }
        _ => {}
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![extract_media_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

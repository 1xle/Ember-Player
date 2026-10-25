// Ember — native side.
//
// Only three jobs live here. Everything else is the HTML file in ../src.
//   1. scan_folder  — walk a music folder and hand the file list to the UI
//   2. oauth_listen — open the system browser, catch the redirect, return the code
//   3. open_url     — open a link in the user's real browser, not in our window
//
// No web framework, no HTTP crate. The OAuth listener is a plain TCP socket
// that reads one request line. That is all an OAuth redirect ever is.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

<<<<<<< HEAD
use std::sync::Mutex;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde::Serialize;

/// The live connection to the Discord app, if one is open.
/// None means "not connected yet" or "Discord was closed" — both are fine:
/// the next update simply tries again.
static DISCORD: Mutex<Option<DiscordIpcClient>> = Mutex::new(None);

=======
use serde::Serialize;

>>>>>>> 965359227f3ec497b0184f97b95072d6fe2c6efb
/// Extensions we treat as playable. The webview can decode all of these.
const AUDIO: &[&str] = &[
    "mp3", "m4a", "aac", "ogg", "oga", "opus", "wav", "flac", "webm",
];

#[derive(Serialize)]
struct Track {
    path: String,
    name: String,
    size: u64,
}

/// Walk `dir` and every folder under it, collecting audio files.
///
/// `depth` stops a symlink loop from running forever — without it, a folder
/// that links back to its own parent would recurse until the stack blows up.
fn walk(dir: &Path, out: &mut Vec<Track>, depth: u8) {
    if depth > 12 || out.len() > 20_000 {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return, // unreadable folder (permissions, unplugged drive) — skip it
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            // Skip the dotfolders that every music library is littered with.
            let hidden = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with('.'))
                .unwrap_or(false);
            if !hidden {
                walk(&path, out, depth + 1);
            }
            continue;
        }

        let is_audio = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| AUDIO.contains(&e.to_lowercase().as_str()))
            .unwrap_or(false);

        if is_audio {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            out.push(Track {
                name: path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                path: path.to_string_lossy().to_string(),
                size,
            });
        }
    }
}

#[tauri::command]
fn scan_folder(path: String) -> Result<Vec<Track>, String> {
    let root = PathBuf::from(&path);
    if !root.is_dir() {
        return Err(format!("{} is not a folder", path));
    }
    let mut out = Vec::new();
    walk(&root, &mut out, 0);
    out.sort_by_key(|t| t.path.to_lowercase());
    Ok(out)
}

/// Open a URL in the user's default browser.
///
/// A login page must never open inside our own window: the user cannot see the
/// address bar there, so they cannot tell a real Spotify page from a fake one.
#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    // Not `cmd /C start`: cmd reads the & in a Spotify link as "run the next
    // command", which cuts the URL short. rundll32 receives it as one argument.
    let result = if cfg!(target_os = "windows") {
        Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", &url])
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(&url).spawn()
    } else {
        Command::new("xdg-open").arg(&url).spawn()
    };
    result.map(|_| ()).map_err(|e| e.to_string())
}

/// Open `url`, then block on `port` until the browser redirects back.
///
/// This is the whole trick behind "Log in with…" in a desktop app. The app
/// becomes a web server for about ten seconds, purely so the browser has
/// somewhere to deliver the authorization code.
#[tauri::command]
async fn oauth_listen(url: String, port: u16) -> Result<String, String> {
    // bind BEFORE opening the browser, or a fast redirect arrives at a closed port
    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|e| format!("port {} is busy: {}", port, e))?;

    open_url(url)?;

    // tauri::async_runtime keeps the blocking accept() off the main thread
    tauri::async_runtime::spawn_blocking(move || {
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };

            // The request line looks like:  GET /callback?code=AQB...&state=x HTTP/1.1
            let mut first = String::new();
            if BufReader::new(&stream).read_line(&mut first).is_err() {
                continue;
            }

            let target = first.split_whitespace().nth(1).unwrap_or("");
            let query = target.split('?').nth(1).unwrap_or("");

            let mut code: Option<String> = None;
            let mut error: Option<String> = None;
            for pair in query.split('&') {
                let mut kv = pair.splitn(2, '=');
                match (kv.next(), kv.next()) {
                    (Some("code"), Some(v)) => code = Some(percent_decode(v)),
                    (Some("error"), Some(v)) => error = Some(percent_decode(v)),
                    _ => {}
                }
            }

            // Reply to the browser so the user sees something human.
            let msg = if code.is_some() {
                "Signed in. You can close this tab and go back to Ember."
            } else {
                "Sign-in did not complete. You can close this tab."
            };
            let body = format!(
                "<!doctype html><meta charset=utf-8><title>Ember</title>\
                 <body style=\"font:16px system-ui;display:grid;place-items:center;\
                 height:100vh;margin:0;background:#101114;color:#f0eeea\"><p>{}</p>",
                msg
            );
            let _ = stream.write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\
                     Content-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                )
                .as_bytes(),
            );
            let _ = stream.flush();

            if let Some(c) = code {
                return Ok(c);
            }
            if let Some(e) = error {
                return Err(format!("authorization denied: {}", e));
            }
            // Anything else (a favicon request, say) — keep waiting.
        }
        Err("listener closed".to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// URL-decode a query value: %20 back to a space, + back to a space.
fn percent_decode(s: &str) -> String {
    let bytes = s.replace('+', " ").into_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
            if let Ok(b) = u8::from_str_radix(hex, 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

<<<<<<< HEAD
/// Show what is playing on the user's Discord profile.
#[tauri::command]
fn discord_set(app_id: String, details: String, state: String) -> Result<(), String> {
    let mut guard = DISCORD.lock().map_err(|e| e.to_string())?;
    if guard.is_none() {
        let mut client = DiscordIpcClient::new(&app_id).map_err(|e| e.to_string())?;
        client.connect().map_err(|e| e.to_string())?;
        *guard = Some(client);
    }
    let result = match guard.as_mut() {
        Some(client) => {
            client.set_activity(activity::Activity::new().details(&details).state(&state))
        }
        None => return Err("not connected".to_string()),
    };
    if let Err(e) = result {
        // Discord was probably closed. Forget the connection so we reconnect next time.
        *guard = None;
        return Err(e.to_string());
    }
    Ok(())
}

/// Remove the activity from Discord and drop the connection.
#[tauri::command]
fn discord_clear() {
    if let Ok(mut guard) = DISCORD.lock() {
        if let Some(client) = guard.as_mut() {
            let _ = client.clear_activity();
        }
        *guard = None;
    }
}

=======
>>>>>>> 965359227f3ec497b0184f97b95072d6fe2c6efb
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            scan_folder,
            open_url,
<<<<<<< HEAD
            oauth_listen,
            discord_set,
            discord_clear
=======
            oauth_listen
>>>>>>> 965359227f3ec497b0184f97b95072d6fe2c6efb
        ])
        .run(tauri::generate_context!())
        .expect("Ember failed to start");
}

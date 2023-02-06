#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::Write;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn create_whazzup_txt(call :String, name :String, lat :String, lon :String, alt :String, spd :String, actype :String, squawk :String, rule :String, route :String, hdg :String) -> String {
    let mut whazzup_path;
	let mut file;
	let aircraft;

	whazzup_path = dirs::home_dir().unwrap();
	whazzup_path.push("AppData/Local/JoinFS/whazzup.txt");
	file = File::create(&whazzup_path).expect("whazzup.txtを作成できませんでした");
	file.write_all(b"!GENERAL\nVERSION = 1\nRELOAD = 1\nUPDATE = 99999999999999\nCONNECTED CLIENTS = 1\nCONNECTED SERVERS = 0\n").unwrap();
	file.write_all(b"!CLIENTS\n").unwrap();
	aircraft = format!("{call}:{name}:{name}:PILOT:111.1.:{lat}:{lon}:{alt}:{spd}:/{actype}:::::JoinFS:::{squawk}::::{rule}:::::::::{route}::::::::{hdg}:::\n");
	file.write_all(aircraft.as_bytes()).unwrap();
	file.write_all(b"!SERVERS\n").unwrap();
    aircraft
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_whazzup_txt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

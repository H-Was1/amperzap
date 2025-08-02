use std::{fs::File, io::BufReader};

use sysinfo::System;
use tauri::{path::BaseDirectory, Manager};

use crate::utils::{BatteryInfo, StaticBatteryInfo, SystemInfo};

use super::battery_info::{get_battery_info, get_static_battery_info};

#[tauri::command]
pub fn battery_info() -> BatteryInfo {
    get_battery_info().unwrap_or(BatteryInfo {
        charge: 0,
        health: None,
        status: String::from(""),
        energy: String::from(""),
        full_energy: String::from(""),
        energy_rate: String::from(""),
        time_to_empty: Option::None,
        time_to_full: Option::None,
        voltage: String::from(""),
        temperature: Option::None,
    })
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        host: System::host_name().unwrap(),
        os_version: System::long_os_version().unwrap(),
        battery: get_static_battery_info().unwrap_or(StaticBatteryInfo {
            cycle_count: Option::None,
            design_energy: Option::None,
            serial_number: Option::None,
            technology: "".to_string(),
            vendor: Option::None,
            model: Option::None,
        }),
    }
}

#[tauri::command(async)]
pub fn beep(handle: tauri::AppHandle) {
    let my_battery_info = battery_info();

    let condition = (my_battery_info.charge <= 35
        && my_battery_info.status.as_str() == "Discharging")
        || (my_battery_info.charge >= 80 && my_battery_info.status.as_str() == "Charging");

    if condition {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let file = BufReader::new(
            File::open(
                handle
                    .path()
                    .resolve("assets/alert_sound.mp3", BaseDirectory::Resource)
                    .unwrap(),
            )
            .unwrap(),
        );
        let source = rodio::Decoder::new(file).unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    } else {
        return;
    }
}

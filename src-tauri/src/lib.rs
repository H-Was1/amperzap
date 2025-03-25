use battery::State;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use sysinfo::System;
use tauri::Manager;

#[derive(Serialize, Deserialize)]
struct BatteryInfo {
    charge: u32,
    health: Option<u32>,
    status: String,
    energy: String,
    full_energy: String,
    energy_rate: String,
    time_to_empty: Option<String>,
    time_to_full: Option<String>,
    voltage: String,
    temperature: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct StaticBatteryInfo {
    vendor: Option<String>,
    model: Option<String>,
    cycle_count: Option<String>,
    design_energy: Option<String>,
    serial_number: Option<String>,
    technology: String,
}

#[derive(Serialize, Deserialize)]
struct SystemInfo {
    host: String,
    os_version: String,
    battery: StaticBatteryInfo,
}

#[tauri::command]
fn battery_info() -> BatteryInfo {
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

#[tauri::command(async)]
fn beep() {
    let my_battery_info = battery_info();

    let condition = (my_battery_info.charge <= 35
        && my_battery_info.status.as_str() == "Discharging")
        || (my_battery_info.charge >= 75 && my_battery_info.status.as_str() == "Charging");

    if condition {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open("/usr/share/sounds/alert_sound.mp3").unwrap());
        let source = rodio::Decoder::new(file).unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    } else {
        return;
    }
}

#[tauri::command]
fn get_system_info() -> SystemInfo {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle()
                    .plugin(tauri_plugin_autostart::init(
                        MacosLauncher::LaunchAgent,
                        Some(vec!["--flag1", "--flag2"]),
                    ))
                    .unwrap();

                // Get the autostart manager
                let autostart_manager = app.autolaunch();
                // Enable autostart
                let _ = autostart_manager.enable();
                // Check enable state
                println!(
                    "registered for autostart? {}",
                    autostart_manager.is_enabled().unwrap()
                );
                // Disable autostart
                // let _ = autostart_manager.disable();
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            use tauri_plugin_notification::NotificationExt;
            app.notification()
                .builder()
                .title("Tauri")
                .body("Tauri is awesome")
                .show()
                .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            battery_info,
            beep,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_battery_info() -> Result<BatteryInfo, battery::Error> {
    // Create a battery manager
    let manager = battery::Manager::new()?;

    // Get the list of batteries
    let batteries = manager.batteries()?;

    let mut battery_info = BatteryInfo {
        charge: 0,
        health: None,
        status: String::from(""),
        voltage: String::from(""),
        temperature: Option::None,
        energy: String::from(""),
        full_energy: String::from(""),
        energy_rate: String::from(""),
        time_to_empty: Option::None,
        time_to_full: Option::None,
    };

    // Iterate over the batteries (most systems have only one)
    for battery in batteries {
        let battery = battery?;

        // Access battery health (state of health)
        let health = battery.state_of_health().value * 100.0;
        battery_info.health = Some(health as u32);

        // Access battery charge level (state of charge)
        let charge = battery.state_of_charge().value * 100.0;
        battery_info.charge = charge as u32;

        // Access charging status
        let charging_status = match battery.state() {
            State::Charging => "Charging",
            State::Discharging => "Discharging",
            State::Full => "Full",
            State::Unknown => "Unknown",
            _ => "Other",
        };
        battery_info.status = String::from(charging_status);
        battery_info.voltage = format!("{:?}", battery.voltage());
        battery_info.temperature = Some(format!("{:?}", battery.temperature()));
        battery_info.energy = format!("{:?}", battery.energy());
        battery_info.full_energy = format!("{:?}", battery.energy_full());
        battery_info.energy_rate = format!("{:?}", battery.energy_rate());
        battery_info.time_to_empty = match battery.time_to_empty() {
            Some(time) => Some(format!("{:?}", time)),
            None => Some(String::from("N/A")),
        };
        battery_info.time_to_full = match battery.time_to_full() {
            Some(time) => Some(format!("{:?}", time)),
            None => Some(String::from("N/A")),
        };
    }

    Ok(battery_info)
}

fn get_static_battery_info() -> Result<StaticBatteryInfo, battery::Error> {
    // Create a battery manager
    let manager = battery::Manager::new()?;

    // Get the list of batteries
    let batteries = manager.batteries()?;

    let mut static_battery_info = StaticBatteryInfo {
        cycle_count: Option::None,
        design_energy: Option::None,
        serial_number: Option::None,
        technology: "".to_string(),
        vendor: Option::None,
        model: Option::None,
    };

    // Iterate over the batteries (most systems have only one)
    for battery in batteries {
        let battery = battery?;

        static_battery_info.model = battery.model().map(|s| s.to_string());
        static_battery_info.vendor = battery.vendor().map(|s| s.to_string());
        static_battery_info.serial_number = battery.serial_number().map(|s| s.to_string());
        static_battery_info.cycle_count = battery.cycle_count().map(|s| s.to_string());
        static_battery_info.technology = battery.technology().to_string();

        let _energy = battery.energy_full_design();

        static_battery_info.design_energy = Some(format!("{:?}", _energy));
    }

    Ok(static_battery_info)
}

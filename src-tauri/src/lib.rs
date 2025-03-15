use battery::{Manager, State};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
struct BatteryInfo {
    charge: u32,
    health: Option<u32>,
    status: String,
}

#[tauri::command]
fn battery_info() -> BatteryInfo {
    get_battery_info().unwrap_or(BatteryInfo {
        charge: 0,
        health: None,
        status: String::from(""),
    })
}

#[tauri::command(async)]
fn beep() {
    let my_battery_info = battery_info();

    let condition = (my_battery_info.charge <= 35
        && my_battery_info.status.as_str() == "Discharging")
        || (my_battery_info.charge >= 75 && my_battery_info.status.as_str() == "Charging");

    println!(
        "Status: {}, Charge: {}, condition: {condition}",
        my_battery_info.status, my_battery_info.charge,
    );

    if condition {
        println!("Beeping starts");

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open("/usr/share/sounds/alert_sound.mp3").unwrap());
        let source = rodio::Decoder::new(file).unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();

        println!("Beep --- Beep");
    } else {
        println!("Skipped Beeping");
        return;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        // .plugin(tauri_plugin_autostart::init())
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
        .invoke_handler(tauri::generate_handler![battery_info, beep])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_battery_info() -> Result<BatteryInfo, battery::Error> {
    // Create a battery manager
    let manager = Manager::new()?;

    // Get the list of batteries
    let batteries = manager.batteries()?;

    let mut battery_info = BatteryInfo {
        charge: 0,
        health: None,
        status: String::from(""),
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
    }

    Ok(battery_info)
}

use battery::{Manager, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BatteryInfo {
    charge: u32,
    health: Option<f32>,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![battery_info])
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
        battery_info.health = Some(health);

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

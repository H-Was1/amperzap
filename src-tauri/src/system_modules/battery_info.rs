use battery::State;

use crate::utils::{BatteryInfo, StaticBatteryInfo};

pub fn get_battery_info() -> Result<BatteryInfo, battery::Error> {
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

pub fn get_static_battery_info() -> Result<StaticBatteryInfo, battery::Error> {
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

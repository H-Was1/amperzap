use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BatteryInfo {
    pub charge: u32,
    pub health: Option<u32>,
    pub status: String,
    pub energy: String,
    pub full_energy: String,
    pub energy_rate: String,
    pub time_to_empty: Option<String>,
    pub time_to_full: Option<String>,
    pub voltage: String,
    pub temperature: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StaticBatteryInfo {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub cycle_count: Option<String>,
    pub design_energy: Option<String>,
    pub serial_number: Option<String>,
    pub technology: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub host: String,
    pub os_version: String,
    pub battery: StaticBatteryInfo,
}

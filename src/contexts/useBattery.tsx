import { invoke } from "@tauri-apps/api/core";
import { createContext, useContext, useState, useEffect } from "react";
import { handleBeep } from "../lib/utils";

interface BatteryInfoProps {
  charge: number;
  health: number;
  status: "Unknown" | "Charging" | "Discharging" | "Full" | "Other";
  voltage: string;
  temperature?: string;
  energy: string;
  full_energy: string;
  energy_rate: string;
  time_to_empty: string;
  time_to_full: string;
}

const initialState: BatteryInfoProps = {
  status: "Unknown",
  charge: 0,
  health: 100,
  energy: "0",
  full_energy: "0",
  energy_rate: "0",
  time_to_empty: "0",
  time_to_full: "0",
  voltage: "0",
  temperature: "0",
};

// Create the context
const BatteryContext = createContext<BatteryInfoProps>(initialState);

// Create a provider
const BatteryProvider = ({ children }: { children: React.ReactNode }) => {
  const [batteryInfo, setBatteryInfo] =
    useState<BatteryInfoProps>(initialState);

  useEffect(() => {
    const updateBatteryInfo = async () => {
      try {
        const battery_info = await invoke("battery_info");
        setBatteryInfo(battery_info as BatteryInfoProps);
      } catch (error) {
        console.error("Failed to check Battery info:", error);
      }
    };

    updateBatteryInfo();
    const interval = setInterval(updateBatteryInfo, 1000);

    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        await handleBeep();
      } catch (error) {
        console.error("Failed to beep:", error);
      }
    }, 15000);

    return () => clearInterval(interval);
  }, []);

  return (
    <BatteryContext.Provider value={batteryInfo}>
      {children}
    </BatteryContext.Provider>
  );
};

const useBattery = () => {
  const batteryInfo = useContext(BatteryContext);
  return { batteryInfo };
};

export { BatteryProvider, useBattery };

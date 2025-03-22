import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
// import { enqueueNotification } from "./lib/utils";

interface BatteryInfoProps {
  charge: number;
  health: number;
  status: "Unknown" | "Charging" | "Discharging" | "Full" | "Other";
  system_info?: string;
}

function App() {
  const [batteryInfo, setBatteryInfo] = useState<BatteryInfoProps>({
    charge: 0,
    health: 100,
    status: "Unknown",
  });

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

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        invoke("battery_info").then((battery_info) => {
          setBatteryInfo(battery_info as BatteryInfoProps);
        });
        // console.log(batteryInfo.system_info);
      } catch (error) {
        console.error("Failed to check Battery info:", error);
      }
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  async function handleBeep() {
    try {
      await invoke("beep");
    } catch (error) {
      console.error("Failed to beep:", error);
    }
  }

  return (
    <main className="h-screen bg-lime-500">
      <h2 className="font-semibold text-3xl">Battery</h2>
      <div>
        <p>Charge: {batteryInfo.charge}</p>
        <p>Health: {batteryInfo.health}</p>
        <p>Status: {batteryInfo.status}</p>
      </div>
    </main>
  );
}

export default App;

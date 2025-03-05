import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/plugin-notification";
import "./App.css";
import { enqueueNotification } from "./lib/utils";

interface BatteryInfoProps {
  charge: number;
  health: number;
  status: "Unknown" | "Charging" | "Discharging" | "Full" | "Other";
}

function App() {
  const [batteryInfo, setBatteryInfo] = useState<BatteryInfoProps>({
    charge: 0,
    health: 100,
    status: "Unknown",
  });
  const [permission, setPermission] = useState(false);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        await handleBeep();
      } catch (error) {
        console.error("Failed to beep:", error);
      }
    }, 10000);

    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    invoke("battery_info").then((battery_info) => {
      setBatteryInfo(battery_info as BatteryInfoProps);
    });
    // Update every minute
  }, []);

  useEffect(() => {
    isPermissionGranted().then((val) => setPermission(val));
  }, []);

  async function handleBeep() {
    const shouldBeep =
      (batteryInfo.status === "Charging" && batteryInfo.charge >= 75) ||
      (batteryInfo.status === "Discharging" && batteryInfo.charge <= 35);

    if (!shouldBeep) return;
    try {
      await invoke("beep");
    } catch (error) {
      console.error("Failed to beep:", error);
    }
  }

  const handleNotification = async () => {
    try {
      // if (batteryInfo.charge < 20) {
      if (await isPermissionGranted()) {
        await enqueueNotification(
          "Battery Low!",
          "Your battery is below 20%. Plug in your device.",
        );
      } else {
        const permit = await requestPermission();
        if (permit) {
          await enqueueNotification(
            "Battery Low!",
            "Your battery is below 20%. Plug in your device.",
          );
        } else {
          console.log("Notification permission denied.");
        }
      }
      // }
    } catch (error) {
      console.error("Failed to send notification:", error);
    }
  };

  return (
    <main className={`${permission ? "bg-lime-400" : "bg-red-400"}`}>
      <h2 className="font-semibold text-3xl">Battery</h2>
      <div>
        <p>Charge: {batteryInfo.charge}</p>
        <p>Health: {batteryInfo.health}</p>
        <p>Status: {batteryInfo.status}</p>
      </div>
      <button
        onClick={handleNotification}
        type="button"
        className="bg-white hover:bg-gray-200 shadow-md"
      >
        Send Notification
      </button>
    </main>
  );
}

export default App;

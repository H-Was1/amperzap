import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface BatteryInfoProps {
  charge: string;
  health: number;
  status: string;
}

function App() {
  const [batteryInfo, setBatteryInfo] = useState<BatteryInfoProps>({
    charge: "0",
    health: 100,
    status: "Unknown",
  });

  // const handleBattery = () => {
  //    invoke("battery_info").then((battery_info) =>console.log(battery_info)
  // }

  useEffect(() => {
    invoke("battery_info").then((battery_info) => {
      setBatteryInfo(battery_info as BatteryInfoProps);
    });
  }, [batteryInfo]);

  return (
    <main className="w-full h-screen px-3 flex flex-col items-center justify-center">
      <h2 className="font-semibold text-3xl">Battery</h2>
      <div>
        <p>Charge: {batteryInfo.charge}</p>
        <p>Health: {Math.trunc(batteryInfo.health)}</p>
        <p>Status: {batteryInfo.status}</p>
      </div>
    </main>
  );
}

export default App;

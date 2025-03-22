import "./App.css";
import { useBattery } from "./contexts/useBattery";
import { useSystemInfo } from "./contexts/useSystem";

function App() {
  const { batteryInfo } = useBattery();
  const { systemInfo } = useSystemInfo();

  return (
    <main className="h-screen bg-cyan-600 select-none cursor-default px-6 py-3">
      <h2 className="font-semibold text-3xl">Battery</h2>
      <div>
        <p>Charge: {batteryInfo.charge}</p>
        <p>Health: {batteryInfo.health}</p>
        <p>Status: {batteryInfo.status}</p>
        <p>OS Version: {systemInfo.os_version}</p>
        <p>Host Name: {systemInfo.host}</p>
      </div>
    </main>
  );
}

export default App;

import "./App.css";
import { useBattery } from "./contexts/useBattery";
import { useSystemInfo } from "./contexts/useSystem";

function App() {
  const { systemInfo } = useSystemInfo();
  const { batteryInfo } = useBattery();

  return (
    <main className="h-screen bg-cyan-600 cursor-default px-6 py-3">
      <h2 className="font-semibold text-3xl">Battery</h2>
      <div>
        <p>Charge: {batteryInfo.charge}</p>
        <p>Health: {batteryInfo.health}</p>
        <p>Status: {batteryInfo.status}</p>
        <p>Voltage: {batteryInfo.voltage}</p>
        <p>Temperature: {batteryInfo.temperature || "N/A"}</p>
        <p>Energy: {batteryInfo.energy}</p>
        <p>Full Energy: {batteryInfo.full_energy}</p>
        <p>Energy Rate: {batteryInfo.energy_rate}</p>
        <p>Time to Empty: {batteryInfo.time_to_empty}</p>
        <p>Time to Full: {batteryInfo.time_to_full}</p>
      </div>

      <h2 className="font-semibold text-3xl">System</h2>
      <div>
        <p>OS Version: {systemInfo?.os_version}</p>
        <p>Host Name: {systemInfo?.host}</p>
        <p>cycle_count: {systemInfo?.battery?.cycle_count || "N/A"}</p>
        <p>model: {systemInfo?.battery?.model || "N/A"}</p>
        <p>design_energy: {systemInfo?.battery?.design_energy || "N/A"}</p>
        <p>serial_number: {systemInfo?.battery?.serial_number || "N/A"}</p>
        <p>technology: {systemInfo?.battery?.technology || "N/A"}</p>
        <p>vendor: {systemInfo?.battery?.vendor || "N/A"}</p>
      </div>
    </main>
  );
}

export default App;

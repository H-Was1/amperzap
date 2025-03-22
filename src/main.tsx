import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BatteryProvider } from "./contexts/useBattery";
import { SystemProvider } from "./contexts/useSystem";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {/* used for battery info */}
    <BatteryProvider>
      {/* used for system info */}
      <SystemProvider>
        <App />
      </SystemProvider>
    </BatteryProvider>
  </React.StrictMode>,
);

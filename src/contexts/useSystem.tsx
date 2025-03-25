import { invoke } from "@tauri-apps/api/core";
import { createContext, useContext, useEffect, useState } from "react";

interface BatteryInfoProps {
  vendor?: string;
  model?: string;
  cycle_count?: string;
  design_energy?: string;
  serial_number?: string;
  technology?: string;
}

export interface SystemInfoProps {
  host: string;
  os_version: string;
  battery?: BatteryInfoProps;
}

const initialState: SystemInfoProps = {
  host: "John Doe",
  os_version: "MacOS 13.0.1",
};

// Create the context
const SystemContext = createContext<SystemInfoProps>(initialState);

// Create a provider
const SystemProvider = ({ children }: { children: React.ReactNode }) => {
  const [systemInfo, setSystemInfo] = useState<SystemInfoProps>(initialState);

  useEffect(() => {
    invoke("get_system_info").then((systemInformation) => {
      setSystemInfo(systemInformation as SystemInfoProps);
    });
  }, []);

  return (
    <SystemContext.Provider value={systemInfo}>
      {children}
    </SystemContext.Provider>
  );
};

const useSystemInfo = () => {
  const systemInfo = useContext(SystemContext);
  return { systemInfo };
};

export { SystemProvider, useSystemInfo };

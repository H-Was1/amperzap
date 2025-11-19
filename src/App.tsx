import "./App.css";
import { useBattery } from "./contexts/useBattery";
import { useSystemInfo } from "./contexts/useSystem";

// Helper component for displaying data rows consistently.
// The `last:border-b-0` class will remove the border from the last row in a list.
const InfoRow = ({
	label,
	value,
}: {
	label: string;
	value: React.ReactNode;
}) => (
	<div className="flex justify-between items-center text-sm py-2 border-b border-gray-700/50 last:border-b-0">
		<span className="text-gray-400">{label}</span>
		<span
			className="font-medium text-gray-200 truncate"
			title={String(value || "")}
		>
			{value || "N/A"}
		</span>
	</div>
);

function App() {
	const { systemInfo } = useSystemInfo();
	const { batteryInfo } = useBattery();

	return (
		<main className="h-screen w-full bg-gray-900 text-white font-sans overflow-y-auto selection:bg-green-400/50 select-none">
			<div className="p-4">
				{/* Battery Information Card */}
				<div className="bg-gray-800/70 backdrop-blur-sm rounded-lg shadow-md mb-6 border border-gray-700/60">
					<h2 className="text-base font-semibold p-3 border-b border-gray-700/60 text-green-400">
						Battery Information
					</h2>
					<div className="px-4 py-2">
						<InfoRow label="Charge" value={batteryInfo.charge} />
						<InfoRow label="Health" value={batteryInfo.health} />
						<InfoRow label="Status" value={batteryInfo.status} />
						<InfoRow
							label="Voltage"
							value={
								batteryInfo.voltage != null
									? (() => {
											const valueStr = String(batteryInfo.voltage);
											const match = valueStr.match(/^(-?\d+(\.\d+)?)/); // Extract potential number at the beginning
											if (match && match[1]) {
												const numValue = parseFloat(match[1]);
												if (!isNaN(numValue)) {
													return `${numValue.toFixed(2)} V`; // Format to 2 decimal places and append ' V'
												}
											}
											return undefined; // If parsing fails, let InfoRow display "N/A"
										})()
									: undefined // If batteryInfo.voltage is null/undefined, let InfoRow display "N/A"
							}
						/>
						<InfoRow label="Temperature" value={batteryInfo.temperature} />
						<InfoRow label="Energy" value={batteryInfo.energy} />
						<InfoRow label="Time to Empty" value={batteryInfo.time_to_empty} />
						<InfoRow label="Time to Full" value={batteryInfo.time_to_full} />
					</div>
				</div>

				{/* System Information Card */}
				<div className="bg-gray-800/70 backdrop-blur-sm rounded-lg shadow-md border border-gray-700/60">
					<h2 className="text-base font-semibold p-3 border-b border-gray-700/60 text-blue-400">
						System Information
					</h2>
					<div className="px-4 py-2">
						<InfoRow label="OS Version" value={systemInfo?.os_version} />
						<InfoRow label="Host Name" value={systemInfo?.host} />
						<InfoRow
							label="Cycle Count"
							value={systemInfo?.battery?.cycle_count}
						/>
						<InfoRow label="Model" value={systemInfo?.battery?.model} />
						<InfoRow
							label="Design Energy"
							value={systemInfo?.battery?.design_energy}
						/>
						<InfoRow
							label="Technology"
							value={systemInfo?.battery?.technology}
						/>
						<InfoRow label="Vendor" value={systemInfo?.battery?.vendor} />
						<InfoRow
							label="Serial Number"
							value={systemInfo?.battery?.serial_number}
						/>
					</div>
				</div>

				<footer className="text-center my-2">
					<a
						href="https://github.com/H-Was1"
						target="_blank"
						rel="noopener noreferrer"
						className="text-sm text-gray-600 hover:text-gray-300 transition-colors"
					>
						Made with ❤️ by H-Was1
					</a>
				</footer>
			</div>
		</main>
	);
}

export default App;

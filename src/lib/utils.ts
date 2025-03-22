import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import { invoke } from "@tauri-apps/api/core";

async function checkPermission() {
  if (!(await isPermissionGranted())) {
    return (await requestPermission()) === "granted";
  }
  return true;
}

export async function enqueueNotification(title: string, body: string) {
  if (!(await checkPermission())) {
    await requestPermission();
  }
  sendNotification({ title, body });
}

export async function handleBeep() {
  try {
    await invoke("beep");
  } catch (error) {
    console.error("Failed to beep:", error);
  }
}

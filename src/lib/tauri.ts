import { invoke } from "@tauri-apps/api/core";

export interface DisplayDevice {
  vendorId: number;
  productId: number;
  interfaceNumber: number | null;
  path: string;
  manufacturer: string | null;
  product: string | null;
}

export interface CommandError {
  code: string;
  message: string;
}

export interface SetBrightnessResponse {
  brightness: number;
}

export async function listDisplays(): Promise<DisplayDevice[]> {
  return invoke<DisplayDevice[]>("list_displays");
}

export async function setBrightness(value: number): Promise<SetBrightnessResponse> {
  return invoke<SetBrightnessResponse>("set_brightness", { value });
}

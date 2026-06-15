import { useCallback, useEffect, useMemo, useState } from "react";
import "./App.css";
import { BrightnessSlider } from "./components/BrightnessSlider";
import { DeviceStatus } from "./components/DeviceStatus";
import { PresetButtons } from "./components/PresetButtons";
import { brightnessFromPercent } from "./lib/brightness";
import { listDisplays, setBrightness, type CommandError, type DisplayDevice } from "./lib/tauri";

const DEFAULT_BRIGHTNESS = brightnessFromPercent(50);

function getCommandMessage(error: unknown): string {
  if (typeof error === "object" && error && "message" in error) {
    return String((error as CommandError).message);
  }

  if (typeof error === "string" && error.length > 0) {
    return `命令失败：${error}`;
  }

  return "命令执行失败";
}

function App() {
  const [devices, setDevices] = useState<DisplayDevice[]>([]);
  const [loadingDevices, setLoadingDevices] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [brightness, setBrightnessValue] = useState(DEFAULT_BRIGHTNESS);
  const [lastConfirmedBrightness, setLastConfirmedBrightness] = useState<number | null>(null);
  const [commandMessage, setCommandMessage] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const refreshDisplays = useCallback(async () => {
    setLoadingDevices(true);
    setLoadError(null);

    try {
      const items = await listDisplays();
      setDevices(items);
    } catch (error) {
      setDevices([]);
      setLoadError(getCommandMessage(error));
    } finally {
      setLoadingDevices(false);
    }
  }, []);

  useEffect(() => {
    void refreshDisplays();
  }, [refreshDisplays]);

  const disabled = useMemo(() => busy || loadingDevices || devices.length === 0, [busy, devices.length, loadingDevices]);

  async function commitBrightness(value: number) {
    setBusy(true);
    setCommandMessage(null);

    try {
      const response = await setBrightness(value);
      setLastConfirmedBrightness(response.brightness);
      setBrightnessValue(response.brightness);
      setCommandMessage("亮度已更新");
    } catch (error) {
      setCommandMessage(getCommandMessage(error));
    } finally {
      setBusy(false);
    }
  }

  function selectPreset(percent: number) {
    const value = brightnessFromPercent(percent);
    setBrightnessValue(value);
    void commitBrightness(value);
  }

  return (
    <main className="app-shell">
      <section className="header">
        <div>
          <h1>Studio Display</h1>
          <DeviceStatus devices={devices} loading={loadingDevices} error={loadError} />
        </div>
        <button type="button" onClick={() => void refreshDisplays()} disabled={busy || loadingDevices}>
          刷新
        </button>
      </section>

      <BrightnessSlider
        value={brightness}
        disabled={disabled}
        onChange={setBrightnessValue}
        onCommit={commitBrightness}
      />

      <PresetButtons disabled={disabled} onSelect={selectPreset} />

      <footer className="footer">
        <span
          aria-atomic="true"
          aria-live="polite"
          className={commandMessage && commandMessage !== "亮度已更新" ? "footer-message error" : "footer-message"}
        >
          {commandMessage ?? "准备就绪"}
        </span>
        <span>{lastConfirmedBrightness === null ? "尚未设置" : `最近成功：${lastConfirmedBrightness}`}</span>
      </footer>
    </main>
  );
}

export default App;

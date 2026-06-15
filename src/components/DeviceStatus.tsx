import type { DisplayDevice } from "../lib/tauri";

interface DeviceStatusProps {
  devices: DisplayDevice[];
  loading: boolean;
  error: string | null;
}

export function DeviceStatus({ devices, loading, error }: DeviceStatusProps) {
  if (loading) {
    return <p className="status muted">正在检测 Studio Display...</p>;
  }

  if (error) {
    return <p className="status error">{error}</p>;
  }

  if (devices.length === 0) {
    return <p className="status error">未找到 Apple Studio Display 控制接口</p>;
  }

  const first = devices[0];

  return (
    <p className="status ok">
      已连接 {first.manufacturer ?? "Apple"} {first.product ?? "Studio Display"}
    </p>
  );
}

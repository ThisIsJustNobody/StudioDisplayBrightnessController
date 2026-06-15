const PRESETS = [25, 50, 75, 100] as const;

interface PresetButtonsProps {
  disabled: boolean;
  onSelect: (percent: number) => void;
}

export function PresetButtons({ disabled, onSelect }: PresetButtonsProps) {
  return (
    <div className="preset-row" aria-label="亮度预设">
      {PRESETS.map((percent) => (
        <button key={percent} type="button" disabled={disabled} onClick={() => onSelect(percent)}>
          {percent}%
        </button>
      ))}
    </div>
  );
}

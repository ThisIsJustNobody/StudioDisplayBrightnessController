import { MAX_BRIGHTNESS, MIN_BRIGHTNESS } from "../lib/brightness";

interface BrightnessSliderProps {
  value: number;
  disabled: boolean;
  onChange: (value: number) => void;
  onCommit: (value: number) => void;
}

const SLIDER_COMMIT_KEYS = new Set([
  "Enter",
  " ",
  "ArrowLeft",
  "ArrowRight",
  "ArrowUp",
  "ArrowDown",
  "Home",
  "End",
  "PageUp",
  "PageDown",
]);

export function BrightnessSlider({ value, disabled, onChange, onCommit }: BrightnessSliderProps) {
  return (
    <label className="control-group">
      <span>亮度</span>
      <input
        type="range"
        min={MIN_BRIGHTNESS}
        max={MAX_BRIGHTNESS}
        value={value}
        disabled={disabled}
        onChange={(event) => onChange(Number(event.currentTarget.value))}
        onPointerUp={(event) => onCommit(Number(event.currentTarget.value))}
        onKeyUp={(event) => {
          if (SLIDER_COMMIT_KEYS.has(event.key)) {
            onCommit(Number(event.currentTarget.value));
          }
        }}
      />
      <output>{value}</output>
    </label>
  );
}

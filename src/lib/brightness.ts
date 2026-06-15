export const MIN_BRIGHTNESS = 400;
export const MAX_BRIGHTNESS = 60000;

export function brightnessFromPercent(percent: number): number {
  const clamped = Math.min(100, Math.max(0, percent));
  return MIN_BRIGHTNESS + Math.round((clamped / 100) * (MAX_BRIGHTNESS - MIN_BRIGHTNESS));
}

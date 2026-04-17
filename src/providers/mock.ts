import type { OverlaySnapshot } from "../overlay/types";

const hhmm = (minutes: number): string => {
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  return `${String(hours).padStart(2, "0")}:${String(mins).padStart(2, "0")}`;
};

export const getMockSnapshot = (): OverlaySnapshot => {
  const now = Date.now();
  const tick = Math.floor(now / 1000);

  const claudeRemaining = 72 - (tick % 11);
  const codexRemaining = 58 - (tick % 7);

  const toStatus = (remaining: number) => {
    if (remaining <= 20) {
      return "critical" as const;
    }
    if (remaining <= 45) {
      return "warn" as const;
    }
    return "ok" as const;
  };

  return {
    title: "quota hud",
    mode: "mock overlay",
    updatedAt: new Date(now).toLocaleTimeString("zh-CN", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
      hour12: false,
    }),
    providers: [
      {
        id: "claude",
        label: "claude",
        remainingLabel: `${claudeRemaining}%`,
        remainingPercent: claudeRemaining,
        windowLabel: "5h 窗口",
        resetInLabel: hhmm(102),
        detail: "剩余会话额度",
        status: toStatus(claudeRemaining),
      },
      {
        id: "codex",
        label: "codex",
        remainingLabel: `${codexRemaining}%`,
        remainingPercent: codexRemaining,
        windowLabel: "周额度",
        resetInLabel: "2d 04h",
        detail: "剩余本周额度",
        status: toStatus(codexRemaining),
      },
    ],
  };
};

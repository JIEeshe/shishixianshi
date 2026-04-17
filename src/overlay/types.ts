export type ProviderStatus = "ok" | "warn" | "critical" | "error";

export interface ProviderSnapshot {
  id: string;
  label: string;
  remainingLabel: string;
  remainingPercent: number;
  windowLabel: string;
  resetInLabel: string;
  detail: string;
  status: ProviderStatus;
}

export interface OverlaySnapshot {
  title: string;
  mode: string;
  updatedAt: string;
  providers: ProviderSnapshot[];
}

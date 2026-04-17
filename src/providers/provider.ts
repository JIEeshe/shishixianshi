import type { OverlaySnapshot } from "../overlay/types";

export interface OverlayProvider {
  loadSnapshot(): Promise<OverlaySnapshot>;
}

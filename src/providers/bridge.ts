import { invoke, isTauri } from "@tauri-apps/api/core";

import type { OverlaySnapshot } from "../overlay/types";
import type { OverlayProvider } from "./provider";
import { getMockSnapshot } from "./mock";

class TauriOverlayProvider implements OverlayProvider {
  async loadSnapshot(): Promise<OverlaySnapshot> {
    if (!isTauri()) {
      return getMockSnapshot();
    }

    return invoke<OverlaySnapshot>("get_overlay_snapshot");
  }
}

export const overlayProvider = new TauriOverlayProvider();

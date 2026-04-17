import { isTauri } from "@tauri-apps/api/core";
import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu } from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import {
  PhysicalPosition,
  currentMonitor,
  getCurrentWindow,
} from "@tauri-apps/api/window";

import { OverlayApp } from "./overlay/OverlayApp";
import { overlayProvider } from "./providers/bridge";
import "./overlay/overlay.css";

const REFRESH_MS = 30_000;
const WINDOW_MARGIN_X = 24;
const WINDOW_MARGIN_Y = 24;

const root = document.querySelector<HTMLDivElement>("#app");

if (!root) {
  throw new Error("Missing #app root");
}

const overlayApp = new OverlayApp(root);

const renderSnapshot = async (): Promise<void> => {
  const snapshot = await overlayProvider.loadSnapshot();
  overlayApp.render(snapshot);
};

const pinOverlayWindow = async (): Promise<void> => {
  if (!isTauri()) {
    return;
  }

  const monitor = await currentMonitor();
  if (!monitor) {
    return;
  }

  const window = getCurrentWindow();
  const windowSize = await window.outerSize();

  const x =
    monitor.position.x + monitor.size.width - windowSize.width - WINDOW_MARGIN_X;
  const y = monitor.position.y + WINDOW_MARGIN_Y;

  await window.setAlwaysOnTop(true);
  await window.setResizable(false);
  await window.setPosition(new PhysicalPosition(x, y));
};

const setupCloseToHide = async (): Promise<void> => {
  if (!isTauri()) {
    return;
  }

  const window = getCurrentWindow();
  await window.onCloseRequested(async (event) => {
    event.preventDefault();
    await window.hide();
  });
};

const setupTray = async (): Promise<void> => {
  if (!isTauri()) {
    return;
  }

  const window = getCurrentWindow();
  const icon = await defaultWindowIcon();
  const trayMenu = await Menu.new({
    items: [
      {
        id: "toggle-overlay",
        text: "显示 / 隐藏",
        action: async () => {
          if (await window.isVisible()) {
            await window.hide();
          } else {
            await window.show();
            await window.setFocus();
            await pinOverlayWindow();
          }
        },
      },
      {
        id: "refresh-overlay",
        text: "立即刷新",
        action: async () => {
          await renderSnapshot();
        },
      },
      {
        id: "quit-app",
        text: "退出",
        action: async () => {
          await window.destroy();
        },
      },
    ],
  });

  const trayOptions: Parameters<typeof TrayIcon.new>[0] = {
    tooltip: "实时显示",
    menu: trayMenu,
    showMenuOnLeftClick: true,
  };

  if (icon) {
    trayOptions.icon = icon;
  }

  await TrayIcon.new(trayOptions);
};

const bootstrap = async (): Promise<void> => {
  await renderSnapshot();
  await pinOverlayWindow();
  await setupCloseToHide();
  await setupTray();

  window.setInterval(() => {
    void renderSnapshot();
  }, REFRESH_MS);
};

void bootstrap();

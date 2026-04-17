import type { OverlaySnapshot, ProviderSnapshot } from "./types";

const escapeHtml = (value: string): string =>
  value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");

const providerRow = (provider: ProviderSnapshot): string => {
  const width = Math.max(0, Math.min(100, provider.remainingPercent));

  return `
    <section class="provider provider--${provider.status}">
      <div class="provider__head">
        <span class="provider__label">${escapeHtml(provider.label)}</span>
        <span class="provider__value">${escapeHtml(provider.remainingLabel)}</span>
      </div>
      <div class="provider__meta">
        <span>${escapeHtml(provider.windowLabel)}</span>
        <span>重置 ${escapeHtml(provider.resetInLabel)}</span>
      </div>
      <div class="provider__bar">
        <span class="provider__fill" style="width:${width}%"></span>
      </div>
      <div class="provider__detail">${escapeHtml(provider.detail)}</div>
    </section>
  `;
};

export class OverlayApp {
  private readonly root: HTMLElement;

  constructor(root: HTMLElement) {
    this.root = root;
  }

  render(snapshot: OverlaySnapshot): void {
    this.root.innerHTML = `
      <main class="hud">
        <header class="hud__header">
          <div>
            <div class="hud__title">${escapeHtml(snapshot.title)}</div>
            <div class="hud__mode">${escapeHtml(snapshot.mode)}</div>
          </div>
          <div class="hud__updated">${escapeHtml(snapshot.updatedAt)}</div>
        </header>
        <section class="hud__body">
          ${snapshot.providers.map(providerRow).join("")}
        </section>
      </main>
    `;
  }
}

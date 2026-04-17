import { defineConfig } from "vite";

const devHost = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: devHost || false,
  },
});

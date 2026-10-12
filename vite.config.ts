import { defineConfig } from 'vite';

// 모바일 기기/에뮬레이터가 개발 서버에 접근하려면 tauri CLI가 주는 TAURI_DEV_HOST가 필요하다.
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
    watch: { ignored: ['**/src-tauri/**'] },
  },
});

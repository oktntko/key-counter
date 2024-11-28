import { fileURLToPath, URL } from 'node:url';
import { defineConfig } from 'vite';

import Vue from '@vitejs/plugin-vue';
import VueJsx from '@vitejs/plugin-vue-jsx';
import AutoImport from 'unplugin-auto-import/vite';
import Unfonts from 'unplugin-fonts/vite';
import { VueUseComponentsResolver } from 'unplugin-vue-components/resolvers';
import VueComponents from 'unplugin-vue-components/vite';
import { VueRouterAutoImports } from 'unplugin-vue-router';
import VueRouter from 'unplugin-vue-router/vite';
import VueLayouts from 'vite-plugin-vue-layouts';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    VueRouter({
      extensions: ['.vue'],
      routesFolder: 'src/page',
      exclude: ['**/component', '**/modal'],
      dts: 'src/vue-router.d.ts',
    }),
    Vue(),
    VueJsx(),
    VueLayouts({
      layoutsDirs: 'src/layout',
      pagesDirs: 'src/page',
      extensions: ['vue'],
      exclude: ['**/component', '**/modal'],
      defaultLayout: 'default',
    }),
    Unfonts({
      // https://fonts.google.com/
      google: {
        families: ['Noto Sans JP', 'M PLUS 1', 'M PLUS 2', 'Murecho', 'M PLUS 1 Code'],
      },
    }),
    AutoImport({
      imports: ['vue', VueRouterAutoImports, 'pinia', '@vueuse/core'],
      vueTemplate: true,
      dts: 'src/auto-import.d.ts',
    }),
    VueComponents({
      dirs: ['src/component'],
      resolvers: [VueUseComponentsResolver()],
      dts: 'src/vue-components.d.ts',
    }),
  ],
  resolve: {
    alias: {
      '~': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
}));

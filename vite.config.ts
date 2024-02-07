import path from 'path';
import { defineConfig, loadEnv } from 'vite';
import vue from '@vitejs/plugin-vue';
import Pages from 'vite-plugin-pages';
import legacy from '@vitejs/plugin-legacy';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { ViteWebfontDownload } from 'vite-plugin-webfont-dl';

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  const envMode = command === 'serve' ? mode : process.env.NODE_ENV || mode || 'production';
  process.env = { ...process.env, ...loadEnv(envMode.trim().toLowerCase(), process.cwd()) };

  return {
    plugins: [
      vue(),
      Pages(),

      AutoImport({
        imports: [
          'vue',
          'vue-router',
          '@vueuse/head',
          '@vueuse/core',
          { 'vue3-toastify': ['toast'] },
        ],
        dirs: ['./src/lib/**'],
        dts: 'src/auto-imports.d.ts',
      }),

      Components({
        dts: 'src/components.d.ts',
      }),

      legacy({
        targets: ['defaults', 'not IE 11'],
      }),

      ViteWebfontDownload([
        'https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@700&family=IBM+Plex+Sans:wght@400;700&display=swap',
      ]),
    ],
    resolve: {
      alias: {
        '~': `${path.resolve(__dirname, 'src')}/`,
        '@': `${path.resolve(__dirname, 'src')}/`,
      },
    },
  };
});

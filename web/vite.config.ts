import { defineConfig } from 'vite';
import path from 'path';
import tailwindcss from '@tailwindcss/vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  base: '/', // set to '/subpath/' if deploying to a subfolder
  build: {
    outDir: 'dist',
    sourcemap: true, // set to false for smaller builds
  },
  define: {
    'process.env': {},
  },
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
});

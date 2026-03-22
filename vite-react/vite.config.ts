import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  base: './',
  build: {
    chunkSizeWarningLimit: 6000,      
    outDir: 'dist',
    // minify: 'esbuild',
    sourcemap: false, // Set to true if you need to debug production code
    rollupOptions: {
      output: {
      manualChunks(id) {
          if (id.includes('node_modules')) {
            return 'vendor';
          }
        },        
      },
    },
  },  
  plugins: [
    react(),
  ],
  ssr: {
    noExternal: ['@apollo/client'],
  },  
  server: {
    origin: 'http://localhost:5173',
    port: 5173,
  },  
})

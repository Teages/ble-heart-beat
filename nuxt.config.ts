// eslint-disable-next-line node/prefer-global/process
const host = process.env.TAURI_DEV_HOST

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: 'latest',
  css: ['~/assets/css/main.css'],

  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
      hmr: host ? { protocol: 'ws', host, port: 14231 } : undefined,
    },
  },
  devServer: {
    port: 14230,
    host: host || 'localhost',
  },
  ignore: ['**/src-tauri/**'],

  modules: [
    '@nuxt/eslint',
    '@nuxt/ui-pro',
    '@vueuse/nuxt',
    '@nuxtjs/device',
  ],

  devtools: { enabled: true },

  eslint: {
    config: { standalone: false },
  },

  icon: {
    provider: 'iconify',
    clientBundle: {
      scan: { globInclude: ['apps/**/*'] },
    },
  },
})

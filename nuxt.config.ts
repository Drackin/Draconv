import tailwindcss from '@tailwindcss/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    ssr: false,
    compatibilityDate: '2025-07-15',
    devtools: { enabled: true },

    vite: {
        clearScreen: false,
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            strictPort: true,
        },
        plugins: [tailwindcss()]
    },
    nitro: {
        preset: 'static'
    },

    ignore: ['/src-tauri/'],
    modules: ['@nuxt/icon', '@pinia/nuxt']
})
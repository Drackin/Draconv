import tailwindcss from '@tailwindcss/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    ssr: false,
    compatibilityDate: '2025-07-15',
    devtools: { enabled: true },

    css: ['~/assets/css/main.css'],

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

    icon: {
        serverBundle: "local",
    },
    ignore: ['/src-tauri/'],
    modules: ['@nuxtjs/color-mode', '@nuxt/icon', '@pinia/nuxt', 'shadcn-nuxt']
})
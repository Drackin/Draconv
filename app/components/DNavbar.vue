<template>
    <div data-tauri-drag-region class="w-full bg-[#111] border-b border-zinc-700 py-3 px-5 flex justify-between space-x-2 items-center select-none h-14 sticky top-0 z-10">
        <header data-tauri-drag-region class="flex items-center h-full space-x-2">
            <img data-tauri-drag-region src="/icon.ico" alt="" class="h-full" draggable="false" />
            <p data-tauri-drag-region class="font-bold">{{ title }}</p>
        </header>

        <section class="flex space-x-1">
            <button id="titlebar-minimize" class="w-8 h-8 bg-zinc-800 hover:bg-zinc-900 rounded-md transition duration-200">
                <Icon name="ic:round-minus" class="text-xl mt-2 text-gray-300"/>
            </button>
            <button id="titlebar-close" class="w-8 h-8 flex justify-center items-center bg-red-600 hover:bg-red-700 rounded-md transition duration-200">
                <Icon name="ic:round-close" class="text-xl text-gray-300"/>
            </button>
        </section>
    </div>
</template>

<script setup lang="ts">
const { title = "DRACONV" } = defineProps<{
    title?: string,
}>()
</script>
<template>
    <div data-tauri-drag-region class="w-full border border-zinc-700 rounded-xl py-3 px-5 flex justify-between space-x-2 items-center select-none h-14">
        <p data-tauri-drag-region class="font-bold">{{ title }}</p>

        <section class="flex space-x-1">
            <button id="titlebar-minimize" class="w-6 h-6 border border-zinc-700 hover:bg-yellow-500 rounded-sm hover:brightness-90 transition duration-200">
                <Icon name="ic:round-minus" class="w-3 h-3 mt-2 text-gray-300"/>
            </button>
            <button id="titlebar-close" class="w-6 h-6 border border-zinc-700 flex justify-center items-center hover:bg-red-500 rounded-sm hover:brightness-90 transition duration-200">
                <Icon name="ic:round-close" class="w-3 h-3 text-gray-300"/>
            </button>
        </section>
    </div>
</template>

<script setup lang="ts">
const { title = "DRACONV" } = defineProps<{
    title?: string,
}>()
</script>
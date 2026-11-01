<template>
    <div
        data-tauri-drag-region
        class="w-full backdrop-blur-xl py-3 flex justify-center items-center select-none h-14 sticky top-0 z-99999"
        :class="showWindowControls ? 'px-24' : 'px-5'"
    >
        <!-- macOS uses native window controls configured in Rust. -->

        <header data-tauri-drag-region class="flex min-w-0 items-center h-full gap-2">
            <img data-tauri-drag-region src="/icon_transparent.png" alt="" class="size-4 shrink-0" draggable="false" />
            <p data-tauri-drag-region class="truncate font-bold">{{ title }}</p>
        </header>

        <div
            v-if="showWindowControls"
            role="group"
            aria-label="Window controls"
            class="window-controls"
            @mousedown.stop
            @dblclick.stop
        >
            <button
                id="titlebar-minimize"
                type="button"
                class="window-control"
                aria-label="Minimize window"
                title="Minimize"
                @click="minimizeWindow"
            >
                <Minus :size="16" :stroke-width="1.75" aria-hidden="true" />
            </button>

            <button
                id="titlebar-close"
                type="button"
                class="window-control window-control-close"
                aria-label="Close window"
                title="Close"
                @click="closeWindow"
            >
                <X :size="16" :stroke-width="1.75" aria-hidden="true" />
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { Minus, X } from "@lucide/vue"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { platform } from "@tauri-apps/plugin-os"

const currentPlatform = platform()
const showWindowControls = currentPlatform === "windows" || currentPlatform === "linux"

const { title = "DRACONV" } = defineProps<{
    title?: string,
}>()

function minimizeWindow() {
    return getCurrentWindow().minimize()
}

function closeWindow() {
    return getCurrentWindow().close()
}
</script>

<style scoped>
.window-controls {
    position: absolute;
    top: 50%;
    right: 0.75rem;
    display: flex;
    gap: 2px;
    padding: 3px;
    border: 1px solid rgb(255 255 255 / 8%);
    border-radius: 11px;
    background: rgb(255 255 255 / 3%);
    box-shadow: 0 1px 2px rgb(0 0 0 / 15%);
    transform: translateY(-50%);
}

.window-control {
    display: grid;
    width: 36px;
    height: 30px;
    place-items: center;
    border-radius: 7px;
    color: #a1a1aa;
    cursor: default;
    transition: background-color 120ms ease, color 120ms ease;
}

.window-control :deep(svg) {
    pointer-events: none;
}

.window-control:hover {
    background: rgb(255 255 255 / 8%);
    color: #fafafa;
}

.window-control:active {
    background: rgb(255 255 255 / 12%);
}

.window-control:focus-visible {
    outline: 2px solid rgb(255 255 255 / 65%);
    outline-offset: 1px;
}

.window-control-close:hover {
    background: #e5484d;
    color: #fff;
}

.window-control-close:active {
    background: #c9363c;
}

@media (prefers-reduced-motion: reduce) {
    .window-control {
        transition: none;
    }
}
</style>

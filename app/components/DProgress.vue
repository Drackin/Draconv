<template>
    <div v-bind="$attrs" class="absolute h-1 left-0 bottom-0 w-full rounded-full bg-neutral-900 progress-track overflow-hidden">
        <div v-if="mode === 'immediate'" class="w-1/3 h-full bg-green-500 rounded-full progress-immediate-thumb"></div>
        <div v-else ref="trackRef" class="h-full bg-green-500 rounded-full"></div>
    </div>
</template>

<script setup lang="ts">
import gsap from 'gsap';

const trackRef = ref<HTMLElement | null>(null);

const { mode = "percent", percent } = defineProps<{
    mode: "percent" | "immediate";
    percent?: number;
}>();

watch(() => percent, (newPercent) => {
    if (mode === "percent" && newPercent !== undefined) {
        const clamped = Math.min(100, Math.max(0, newPercent));
        const el = document.querySelector('.progress-track > div') as HTMLElement;
        if (el) {
            gsap.to(el, { width: `${clamped}%`, duration: 0.5, ease: "power1.out" });
        }
    }
}, { immediate: true });
</script>

<style>
.progress-immediate-thumb {
    animation: progress 1s linear infinite;
}

@keyframes progress {
    0% { transform: translateX(-200%); }
    50% { transform: translateX(0); }
    100% { transform: translateX(400%); }
}
</style>
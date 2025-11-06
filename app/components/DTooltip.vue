<template>

    <div class="relative" @mouseenter="tooltipTrigger" @mouseleave="tooltipTrigger">
        <teleport to="body">
            <section
                ref="contentRef"
                class="fixed hidden opacity-0 z-10 p-2 bg-[#222] border border-zinc-700 rounded-md text-sm text-gray-300 shadow-lg tooltip-content"
                :style="{ top: pos.top + 'px', left: pos.left + 'px' }"
            >
                <slot name="content" />
            </section>
        </teleport>
    
        <div ref="target">
            <slot></slot>
        </div>
    </div>

</template>


<script setup lang="ts">
import { gsap } from "gsap";

const contentRef = ref<HTMLElement | null>(null);
const target = ref<HTMLElement | null>(null);
const isOpen = ref(false);
const tl = ref<gsap.core.Timeline | null>(null);
const pos = ref({ top: 0, left: 0 });

const updatePosition = () => {
    if (!target.value || !contentRef.value) return;

    const rect = target.value.getBoundingClientRect();
    const ttRect = contentRef.value.getBoundingClientRect();

    pos.value = {
        top: rect.top - window.scrollY - rect.height - 16, // 8px yukarı boşluk
        left: rect.left + window.scrollX - (ttRect.width / 2),
    };
}

onMounted(() => {
    tl.value = gsap.timeline({ paused: true, reversed: true, defaults: { delay: 0.3, duration: 0.1 } })
    .fromTo(contentRef.value, { display: "none", opacity: 0 }, { display: "flex", opacity: 1 })
});

const tooltipTrigger = async () => {
    if(!isOpen.value) {
        updatePosition()
        await nextTick()
        tl.value?.play()
    }
    else
        tl.value?.reverse()

    isOpen.value = !isOpen.value
}
</script>
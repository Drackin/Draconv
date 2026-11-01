<template>
    <Teleport to="body">
        <Transition :css="false" @enter="onEnter" @leave="onLeave">
            <div
                v-if="isOpen"
                ref="tooltipRef"
                role="tooltip"
                class="fixed max-w-[90vw] opacity-0 z-10 p-2 bg-[#111] border border-zinc-800 rounded-md text-sm text-gray-300 shadow-lg break-all pointer-events-none"
                :style="{ top: pos.top + 'px', left: pos.left + 'px' }"
            >
                <span v-if="props.title">{{ title }}</span>
                <slot name="content" />
            </div>
        </Transition>
    </Teleport>

    <div class="inline-block w-fit h-fit" ref="triggerRef" @focusin="show" @focusout="hide" @mouseenter="show" @mouseleave="hide">
        <slot />
    </div>

</template>


<script setup lang="ts">
import { gsap } from "gsap";

const triggerRef = ref<HTMLElement | null>(null);
const tooltipRef = ref<HTMLElement | null>(null);
const isOpen = ref(false);
const pos = ref({ top: 0, left: 0 });

const props = defineProps<{
    title?: string
}>()

const calculatePosition = () => {
    if (!triggerRef.value || !tooltipRef.value) return;

    const triggerRect = triggerRef.value.getBoundingClientRect()
    const tooltipRect = tooltipRef.value.getBoundingClientRect()
    const windowWidth = window.innerWidth
    const padding = 12; // Gap from window edges
    const gap = 8;      // Gap from trigger element

    let left = triggerRect.left + (triggerRect.width / 2) - (tooltipRect.width / 2)
    left = Math.max(padding, Math.min(left, windowWidth - tooltipRect.width - padding));

    let top = triggerRect.top - tooltipRect.height - gap

    if (top < padding) {
        top = triggerRect.bottom + gap
    }


    if (left < padding) {
        left = padding
    } else if (left + tooltipRect.width > windowWidth - padding) {
        left = windowWidth - tooltipRect.width - padding
    }

    pos.value = {
        top,
        left,
    }
}

const show = async () => { isOpen.value = true }
const hide = () => { isOpen.value = false }

const onEnter = (el: Element, done: () => void) => {
    calculatePosition()

    gsap.fromTo(el,
        {
            opacity: 0,
            y: 5, 
            scale: 0.95, 
        },
        { 
            opacity: 1,
            y: 0,
            scale: 1,
            duration: 0.2,
            ease: "power2.in",
            onComplete: () => {
                done();
                window.addEventListener('scroll', calculatePosition, { passive: true });
            }
        }
    )
}

const onLeave = (el: Element, done: () => void) => {

    gsap.to(el, {
        opacity: 0,
        scale: 0.95,
        y: 5,
        duration: 0.2,
        ease: "power2.in",
        onComplete: done
    })

    window.removeEventListener("scroll", calculatePosition)

}
</script>
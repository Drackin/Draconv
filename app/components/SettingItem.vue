<template>
    <li
        class="w-full gap-2"
        :class="position === 'row' ? 'flex justify-between items-center' : 'flex flex-col'"
    >
        <p class="flex flex-col shrink-0">
            <span>
                {{ title }}
            </span>

            <span
                v-if="subtitle"
                class="text-xs"
                :class="subtitleVariant === 'neutral' ? 'text-gray-400' : 'text-yellow-600'"
            >
                {{ subtitle }}
            </span>
        </p>

        <section
            class="flex items-center w-full space-x-2"
            :class="[
                position === 'column' ? 'w-full' : 'justify-end' 
            ]"
        >
            <slot />
        </section>
    </li>
</template>

<script setup lang="ts">

const { title, subtitle, subtitleVariant = 'neutral', position = "column" } = defineProps<{
    title: string
    subtitle?: string
    subtitleVariant?: "neutral" | "warning"
    position?: "row" | "column"
}>()

</script>
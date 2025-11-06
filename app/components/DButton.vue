<template>
    <button
        v-bind="$attrs"
        :class="[
            'p-3 transition duration-200 rounded-lg flex items-center justify-center text-sm',
            variant === 'primary' ? 'bg-green-500 hover:bg-green-600 disabled:bg-green-900 active:bg-green-700' : '',
            variant === 'secondary' ? 'bg-gray-500 hover:bg-gray-600 disabled:bg-gray-900 active:bg-gray-700' : '',
            variant === 'error' ? 'bg-red-500 hover:bg-red-600 disabled:bg-red-900 active:bg-red-700' : '',
            variant === 'warning' ? 'bg-yellow-600 hover:bg-yellow-700 disabled:bg-yellow-900 active:bg-yellow-700' : '',
            variant === 'neutral' ? 'bg-neutral-800 hover:bg-zinc-900 disabled:bg-zinc-900' : '',
        ]"
    >
        <slot />
    </button>
</template>

<script setup lang="ts">
type Variants = 'primary' | 'secondary' | 'error' | 'warning' | 'neutral'

const { variant = "primary" } = defineProps<{
    variant?: Variants,
}>()
</script>
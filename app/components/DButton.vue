<template>
    <button
        v-bind="$attrs"
        :class="[
            'py-3 px-5 transition duration-200 rounded-xl flex items-center justify-center text-sm active:scale-95 cursor-pointer disabled:transform-none',
            variant === 'primary' ? 'bg-green-600 hover:bg-green-700 disabled:bg-green-900 active:bg-green-800' : '',
            variant === 'secondary' ? 'bg-gray-500 hover:bg-gray-600 disabled:bg-gray-900 active:bg-gray-700' : '',
            variant === 'error' ? 'bg-red-500 hover:bg-red-600 disabled:bg-red-900 active:bg-red-700' : '',
            variant === 'warning' ? 'bg-yellow-600 hover:bg-yellow-700 disabled:bg-yellow-900 active:bg-yellow-700' : '',
            variant === 'neutral' ? 'bg-neutral-800 hover:bg-zinc-900 disabled:bg-zinc-900' : '',
        ]"
    >
        <Icon v-if="icon" :name="icon" class="text-xl mr-2" />
        <slot />
    </button>
</template>

<script setup lang="ts">
type Variants = 'primary' | 'secondary' | 'error' | 'warning' | 'neutral'

const { variant = "primary", icon } = defineProps<{
    variant?: Variants,
    icon?: string,
}>()
</script>
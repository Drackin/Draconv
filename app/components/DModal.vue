<template>
    <Teleport to="body">
        <Transition name="modal">
            <div v-bind="$attrs" ref="backdropRef" v-if="isOpen" class="flex items-center justify-center fixed w-screen h-screen bg-black/50 z-10 top-0 left-0">
                
                <section ref="dialogRef" class="flex flex-col rounded-xl bg-[#111] p-2 w-[50vw] max-w-1/2 max-h-[90vh]">
                    
                    <header v-if="title" class="flex items-center justify-center text-lg p-3">
                        <p class="font-semibold">{{ title }}</p>
                    </header>
    
                    <main class="flex-1 p-3 space-y-3 h-full overflow-y-auto text-sm">
                        <slot />
                    </main>
    
                    <footer class="flex items-center justify-end pt-2 space-x-2">
                        <slot name="footer" />
                    </footer>
                </section>
            </div>
        </Transition>
    </Teleport>
</template>

<script setup lang="ts">
import { useDialogs } from '~/lib/useDialogs'

const dialogRef = ref<HTMLElement | null>(null)
const dialog = useDialogs()

defineOptions({ inheritAttrs: false })

const props = defineProps({
    title: String,
    closeClickOutside: {
        type: Boolean,
        default: true,
    },
})

const isOpen = defineModel({
    default: true
})

const emit = defineEmits(['update:modelValue'])

function close() {
    emit('update:modelValue', false)
}

const onKey = (e: KeyboardEvent) => {
    if (e.key === 'Escape') close()
}

const onClickOutside = (e: MouseEvent) => {
    if(dialog.disableClickOutsideSettings) return

    if (props.closeClickOutside && dialogRef.value && !dialogRef.value.contains(e.target as Node)) {
        close()
    }
}

onMounted(() => {
    document.addEventListener('keydown', onKey)
    document.addEventListener('mousedown', onClickOutside)
})

onBeforeUnmount(() => {
    document.removeEventListener('keydown', onKey)
    document.removeEventListener('mousedown', onClickOutside)
})
</script>

<style>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
<template>
    <div class="relative bg-[#111] rounded-xl overflow-hidden border border-neutral-800" ref="selectContainer">
        <button @click="toggleDropdown" ref="triggerButton" class="relative w-full p-3 text-left flex justify-between items-center">
            <span class="block truncate">
                {{ selectedOption || placeholder || "Select an option" }}
            </span>

            <Icon name="solar:alt-arrow-down-outline" class="text-xl" />
        </button>

        <Teleport to="body">
            <Transition @before-enter="onBeforeEnter" name="dropdown">
                <ul v-if="isOpen" ref="dropdownList" :style="dropdownStyle" class="fixed z-10 mt-1 w-full max-h-60 rounded-xl p-1 overflow-auto bg-[#111] border border-neutral-800 shadow-lg">
                    <li v-for="option in normalizedOptions" @click="selectOption(option)" class="select-none relative hover:bg-neutral-800 rounded-md p-2 cursor-pointer transition duration-150 flex items-center justify-between">
                        <span class="block truncate">
                            {{ option.name }}
                        </span>
    
                        <span v-if="modelValue === option.value">
                            <Icon name="ic:round-check" class="text-lg" />
                        </span>
                    </li>
                </ul>
            </Transition>
        </Teleport>
    </div>
</template>

<script setup lang="ts">
import { useDialogs } from '~/lib/useDialogs';

export interface SelectOption {
    name: string;
    value: string | number;
}

type OptionType = string | number | SelectOption

const props = defineProps<{
    modelValue: string | number | null;
    options: OptionType[];
    placeholder?: string
}>()

const emit = defineEmits<{
    (e: 'update:modelValue', value: string | number | null): void;
}>()

const isOpen = ref(false)
const selectContainer = ref<HTMLElement | null>(null)
const triggerButton = ref<HTMLElement | null>(null)
const dropdownList = ref<HTMLElement | null>(null)

const dialog = useDialogs()

const dropdownStyle = ref({
    top: "0px",
    left: "0px",
    width: "auto",
    bottom: "0px",
    transformOrigin: "top",
})

const normalizedOptions = computed<SelectOption[]>(() => {
    return props.options.map(opt => {
        if (typeof opt === 'string' || typeof opt === 'number') {
            return { name: String(opt), value: opt }
        }

        return opt as SelectOption
    })
})

const selectedOption = computed(() => {
    const selected = normalizedOptions.value.find(opt => opt.value === props.modelValue)
    return selected ? selected.name : null
})

const updatePosition = (element: HTMLElement) => {
    if (!triggerButton.value) return

    const rect = triggerButton.value.getBoundingClientRect()
    const windowHeight = window.innerHeight

    element.style.width = `${rect.width}px`

    const dropdownHeight = element.offsetHeight

    const spaceBelow = windowHeight - rect.bottom
    const spaceAbove = rect.top

    const safetyMargin = 10

    const shouldFlipUp = (spaceBelow < (dropdownHeight + safetyMargin)) && (spaceAbove > spaceBelow)

    if (shouldFlipUp) {
        return {
            bottom: `${windowHeight - rect.top + 5}px`,
            left: `${rect.left}px`,
            width: `${rect.width}px`,
            top: 'auto',
            transformOrigin: 'bottom',
        }
    } else {
        return {
            top: `${rect.bottom + 5}px`,
            left: `${rect.left}px`,
            width: `${rect.width}px`,
            bottom: 'auto',
            transformOrigin: 'top',
        }
    }
}

const onBeforeEnter = (el: Element) => {
    const htmlEl = el as HTMLElement
    
    const styles = updatePosition(htmlEl)

    Object.assign(htmlEl.style, styles)

    dropdownStyle.value = styles as typeof dropdownStyle.value
};

const updatePositionOnly = () => {
    if (!dropdownList.value) return;

    dropdownStyle.value = updatePosition(dropdownList.value) as typeof dropdownStyle.value
};

const toggleDropdown = async () => {
    
    if(isOpen.value) {
        isOpen.value = false
        dialog.disableClickOutsideSettings = false
        window.removeEventListener('scroll', updatePositionOnly)
    } else {
        isOpen.value = true

        await nextTick()

        updatePositionOnly()
        
        dialog.disableClickOutsideSettings = true
        window.addEventListener('scroll', updatePositionOnly, true)
    }
}

const selectOption = (option: SelectOption) => {
    emit('update:modelValue', option.value)
    dialog.disableClickOutsideSettings = false
    isOpen.value = false
}

const handleClickOutside = (event: MouseEvent) => {
    if (selectContainer.value && !selectContainer.value.contains(event.target as Node)) {
        isOpen.value = false
        dialog.disableClickOutsideSettings = false
    }
}

onMounted(() => {
    document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
    document.removeEventListener('scroll', updatePositionOnly)
})
</script>

<style>
.dropdown-enter-active, .dropdown-leave-active {
    transition: all 0.2s ease;
}

.dropdown-enter-from, .dropdown-leave-to {
    opacity: 0;
    transform: scaleY(0.95);
}

.dropdown-enter-to, .dropdown-leave-from {
    opacity: 1;
    transform: scaleY(1);
}
</style>
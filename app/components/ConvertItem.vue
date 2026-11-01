<template>
    <section class="relative isolate w-full overflow-hidden rounded-2xl border border-white/10 p-3 shadow-lg shadow-black/10 backdrop-blur-xl transition duration-300 hover:border-white/20 hover:bg-zinc-950/75 will-change-transform sm:p-4">
        <div class="pointer-events-none absolute -right-12 -top-20 -z-10 h-44 w-44 rounded-full bg-white/10 blur-3xl"></div>

        <div class="relative flex flex-col gap-4 md:flex-row md:items-center">
            <div class="flex min-w-0 flex-1 items-center gap-3">
                <div class="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-white text-black shadow-lg shadow-white/10">
                    <Icon :name="getIcon()" class="text-2xl" />
                </div>

                <div class="min-w-0 flex-1">
                    <div class="flex min-w-0 items-center gap-2">
                        <Tooltip>
                            <template v-slot:content>
                                {{ fileData?.full_file_name }}
                            </template>
                            <p class="min-w-0 truncate text-sm font-semibold tracking-tight sm:text-base">{{ displayFileName }}</p>
                        </Tooltip>
                        <span class="shrink-0 rounded-full border px-2 py-0.5 text-xs" :class="statusClass">
                            {{ statusLabel }}
                        </span>
                    </div>
                    <p class="mt-1 truncate text-xs text-zinc-400">{{ fileData?.dir_path }}</p>
                </div>
            </div>

            <div class="flex w-full flex-col gap-2 sm:flex-row sm:items-center md:w-auto md:min-w-100 md:justify-end">
                <Tooltip title="Select a format to convert">
                    <UiSelect v-model="selectedExtension" class="bg-zinc-900/60 h-11" placeholder="Convert to" :options="fileData?.convertibles as string[]">
                        <UiSelectTrigger class="w-36 h-11!">
                            <UiSelectValue placeholder="Convert to" />
                        </UiSelectTrigger>

                        <UiSelectContent>
                            <UiSelectGroup>
                                <UiSelectLabel>Valid Formats</UiSelectLabel>

                                <UiSelectItem v-for="ext in fileData?.convertibles" :key="ext" :value="ext">
                                    {{ ext }}
                                </UiSelectItem>
                            </UiSelectGroup>
                        </UiSelectContent>
                    </UiSelect>
                </Tooltip>

                <div class="flex w-full overflow-hidden rounded-xl bg-white/5 sm:w-auto">
                    <UiButton
                        @click="startConversion"
                        :disabled="
                                fileData?.conversionStatus === 'processing' ||
                                fileData?.conversionStatus === 'success' ||
                                fileData?.selected_extension === 'unselected'
                        "
                        class="h-11 flex-1 rounded-none text-2xl"
                        aria-label="Convert file"
                    >
                        <Icon v-if="fileData?.conversionStatus === 'processing'" name="lucide:loader-circle" class="text-xl animate-spin" />
                        <Icon v-else :name="fileData?.conversionStatus === 'success' ? 'lucide:check' : 'lucide:play'" class="text-xl" />
                    </UiButton>

                    <UiButton @click="() => toggleConversionInfo(id)" v-if="fileData?.conversionStatus === 'success'"
                        variant="secondary" class="h-11 flex-1 rounded-none text-2xl" aria-label="View conversion details">
                        <Icon name="lucide:info" class="text-xl" />
                    </UiButton>

                    <UiButton
                        v-if="fileData?.conversionStatus === 'processing'"
                        :disabled="fileData.file_type === 'image'"
                        @click="$emit('cancelRequest')"
                        class="h-11 flex-1 rounded-none"
                        variant="destructive"
                        aria-label="Cancel conversion"
                    >
                        <Icon name="lucide:circle-stop" class="text-xl" />
                    </UiButton>

                    <UiButton v-else @click="() => fileStore.deleteFile(id)" class="h-11 flex-1 rounded-none" variant="destructive">
                        <Icon name="lucide:trash" class="text-xl" />
                    </UiButton>
                </div>

                <UiButton
                    @click="toggleDetails"
                    class="size-11 overflow-hidden p-0 rounded-xl"
                    variant="secondary"
                    :aria-label="isDetailsOpen ? 'Hide details' : 'Show details'"
                    :aria-expanded="isDetailsOpen"
                    :aria-controls="`file-details-${id}`"
                >
                    <Icon name="lucide:chevron-down" class="text-xl transition-transform duration-300 motion-reduce:transition-none" :class="{ 'rotate-180': isDetailsOpen }" />
                </UiButton>
            </div>

        </div>

        <ProgressBar :key="fileData?.id" :mode="fileData?.file_type === 'image' ? 'immediate' : 'percent'"
            :percent="fileData.progress" v-if="fileData?.conversionStatus === 'processing'" class="absolute left-0 bottom-0" />

        <div
            :id="`file-details-${id}`"
            class="details-panel"
            :class="{ 'details-panel-open': isDetailsOpen }"
            :inert="!isDetailsOpen"
            :aria-hidden="!isDetailsOpen"
        >
            <div class="min-h-0 overflow-">
                <div class="border-t border-muted-800/50 mt-3 pt-3 text-xs text-zinc-400 sm:mt-4 sm:pt-4">
                    <section v-if="fileData" class="space-y-1">
                        <p>Edit Export Name</p>
                        <UiInput
                            v-model="fileData.output_file_name"
                            @blur="() => fileData?.output_file_name === '' ? fileData.output_file_name : fileData?.file_name"
                            class="w-full bg-zinc-900/60"
                            placeholder="Export Name"
                        />
                    </section>
                </div>
            </div>
        </div>
    </section>
</template>

<script setup lang="ts">
import { useDialogs } from '~/lib/useDialogs';
import { useFileStore } from '~/lib/useFileStore';

const {
    id,
    startConversion,
} = defineProps<{
    id: string,
    startConversion: () => void,
}>()

const isDetailsOpen = ref(false)

const fileStore = useFileStore()
const { toggleConversionInfo } = useDialogs()
const fileData = computed(() => fileStore.files.find(file => file.id === id))
const displayFileName = computed(() => {
    const name = fileData.value?.full_file_name || ''
    const characters = Array.from(name)
    const maxLength = 30

    return characters.length > maxLength
        ? `${characters.slice(0, maxLength - 1).join('')}…`
        : name
})
const statusLabel = computed(() => {
    switch (fileData.value?.conversionStatus) {
        case 'processing': return 'Converting'
        case 'success': return 'Completed'
        case 'failed': return 'Failed'
        case 'cancelled': return 'Cancelled'
        default: return 'Ready'
    }
})
const statusClass = computed(() => {
    switch (fileData.value?.conversionStatus) {
        case 'processing': return 'border-white/20 bg-white/10 text-white'
        case 'success': return 'border-emerald-400/20 bg-emerald-400/10 text-emerald-300'
        case 'failed':
        case 'cancelled': return 'border-red-400/20 bg-red-400/10 text-red-300'
        default: return 'border-white/10 bg-white/5 text-zinc-400'
    }
})

const selectedExtension = ref(fileData.value?.selected_extension || "unselected")

watch(selectedExtension, (newVal) => {
    if (fileData.value) {
        fileData.value!.selected_extension = newVal
    }
})

const toggleDetails = () => {
    isDetailsOpen.value = !isDetailsOpen.value
}

const getIcon = () => {
    switch(fileData.value?.file_type) {
        case 'video':
            return 'lucide:film'
        case 'audio':
            return 'lucide:eighth-note'
        case 'image':
            return 'lucide:image'
        default:
            return 'lucide:file'
    }
}
</script>

<style scoped>
.details-panel {
    display: grid;
    grid-template-rows: 0fr;
    opacity: 0;
    visibility: hidden;
    transition:
        grid-template-rows 300ms cubic-bezier(0.22, 1, 0.36, 1),
        opacity 200ms ease,
        visibility 0s 300ms;
}

.details-panel-open {
    grid-template-rows: 1fr;
    opacity: 1;
    visibility: visible;
    transition-delay: 0s;
}

@media (prefers-reduced-motion: reduce) {
    .details-panel {
        transition: none;
    }
}
</style>

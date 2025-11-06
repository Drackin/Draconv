<template>
    <section class="w-full h-24 p-5 rounded-xl border border-zinc-700 flex items-center relative z-0 overflow-hidden">

            <div class="flex items-center pr-4">
                <Icon
                    :name="icon"
                    class="text-4xl"
                />
            </div>

            <div class="flex flex-col w-1/2">
                <DTooltip>
                    <template v-slot:content>
                        {{ fileData?.full_file_name }}
                    </template>
                    <p class="text-lg font-semibold line-clamp-1">{{ fileData?.full_file_name }}</p>
                </DTooltip>
                <p class="text-xs text-gray-300">{{ fileData?.dir_path }}</p>
            </div>

            <div class="h-full border-l border-neutral-800 mx-3"></div>

            <div class="flex items-center space-x-3 justify-between w-1/2">
                <section class="flex flex-col">

                    <DTooltip :text="isSupported ? 'Supported' : 'Not Supported'">
                        <template v-slot:content>
                            <p v-if="isSupported">Supported</p>
                            <p v-else>Not Supported</p>
                        </template>
                        <p :class="['font-bold text-lg flex items-center', isSupported ? 'text-green-500' : 'text-red-500']">
                            <span>Type</span>
                        </p>
                    </DTooltip>
                    <p class="text-sm text-gray-300">{{ fileData?.file_type }}</p>
                </section>

                <section>
                    <select v-model="fileStore.selected_extension" :disabled="!isSupported" name="type" id="type-selector" class="w-28 px-1 py-2 bg-[#111] h-full rounded-lg backdrop-blur-sm border border-neutral-700">
                        <option value="unselected" selected disabled>Convert to</option>
                        <option :value="type" v-for="type in convertibles">{{ type }}</option>
                    </select>
                </section>

                <div class="flex rounded-xl overflow-hidden">
                    <DButton
                        @click="startConversion"
                        :disabled="fileStore.conversionStatus === 'processing' || fileStore.selected_extension === 'unselected'"
                        class="text-2xl h-full rounded-none"
                    >
                        <Icon
                            name="solar:refresh-outline"
                            :class="[
                                'text-xl',
                                fileStore.conversionStatus === 'processing' ? 'animate-spin' : ''
                            ]"
                        />
                    </DButton>

                    <DButton
                        v-if="fileStore.conversionStatus === 'processing'"
                        @click="$emit('cancelRequest')"
                        class="text-2xl h-full rounded-none"
                        variant="error"
                    >
                        <Icon name="solar:trash-bin-trash-outline" class="text-xl" />
                    </DButton>

                    <DButton v-else @click="fileStore.deleteFile" class="h-full rounded-none" variant="error">
                        <Icon name="solar:trash-bin-trash-outline" class="text-xl" />
                    </DButton>
                </div>

            </div>

            <DProgress :mode="fileData?.file_type === 'image' ? 'immediate' : 'percent'" :percent="progressPercent" v-if="fileStore.conversionStatus === 'processing'" />

        </section>
</template>

<script setup lang="ts">
import { useFileStore } from '~/lib/useFileStore';

const {
    convertibles,
    isSupported,
    progressPercent,
    startConversion,
} = defineProps<{
    convertibles: string[],
    isSupported: boolean,
    progressPercent: number,
    startConversion: () => void,
}>()

const icon = ref('solar:file-outline')
const fileStore = useFileStore()
const fileData = fileStore.file_data

onMounted(() => {
    icon.value = getIcon(fileData?.file_type || '')
})



const getIcon = (type: string) => {
    switch(type) {
        case 'video':
            return 'solar:clapperboard-play-outline'
        case 'audio':
            return 'solar:music-note-outline'
        case 'image':
            return 'solar:gallery-outline'
        default:
            return 'solar:file-outline'
    }
}
</script>
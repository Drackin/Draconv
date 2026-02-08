<template>
    <section class="w-full h-24 p-5 rounded-xl border border-zinc-700 flex items-center relative backdrop-blur-lg bg-[#111]/50 overflow-hidden hover:bg-gradient-to-r from-green-600/10 via-transparent to-green-600/10 will-change-transform">

        <div class="flex items-center pr-4">
            <Icon :name="getIcon()" class="text-4xl" />
        </div>

        <div class="flex flex-col w-1/2">
            <DTooltip>
                <template v-slot:content>
                    {{ fileData?.full_file_name }}
                </template>
                <p class="text-lg font-semibold w-full line-clamp-1 break-all">{{ fileData?.full_file_name }}</p>
            </DTooltip>
            <p class="text-xs text-gray-300">{{ fileData?.dir_path }}</p>
        </div>

        <div class="h-full border-l border-neutral-800 mx-3"></div>

        <div class="flex items-center space-x-3 justify-between w-1/2">

            <section>
                <DTooltip title="Select a format to convert">
                    <DSelect v-model="selectedExtension" class="w-32 bg-neutral-900/50" placeholder="Convert to" :options="fileData?.convertibles as string[]" />
                </DTooltip>
            </section>

            <div class="flex rounded-xl overflow-hidden">
                <DButton @click="startConversion" :disabled="
                            fileData?.conversionStatus === 'processing' ||
                            fileData?.conversionStatus === 'success' ||
                            fileData?.selected_extension === 'unselected'" class="text-2xl h-full !rounded-none">
                    <Icon :name="fileData?.conversionStatus === 'success' ? 'ic:round-check' : 'solar:refresh-outline'"
                        :class="[
                                'text-xl',
                                fileData?.conversionStatus === 'processing' ? 'animate-spin' : '',
                            ]" />
                </DButton>

                <DButton @click="() => toggleConversionInfo(id)" v-if="fileData?.conversionStatus === 'success'"
                    class="text-2xl h-full !rounded-none !bg-blue-500 hover:brightness-90 transition duration-200">
                    <Icon name="solar:info-circle-outline" class="text-xl" />
                </DButton>

                <DButton v-if="fileData?.conversionStatus === 'processing'" :disabled="fileData.file_type === 'image'" @click="$emit('cancelRequest')"
                    class="text-2xl h-full !rounded-none" variant="error">
                    <Icon name="solar:trash-bin-trash-outline" class="text-xl" />
                </DButton>

                <DButton v-else @click="() => fileStore.deleteFile(id)" class="h-full !rounded-none" variant="error">
                    <Icon name="solar:trash-bin-trash-outline" class="text-xl" />
                </DButton>
            </div>

        </div>

        <DProgress :key="fileData?.id" :mode="fileData?.file_type === 'image' ? 'immediate' : 'percent'"
            :percent="fileData.progress" v-if="fileData?.conversionStatus === 'processing'" class="absolute left-0 bottom-0" />

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


const fileStore = useFileStore()
const { toggleConversionInfo } = useDialogs()
const fileData = computed(() => fileStore.files.find(file => file.id === id))

const selectedExtension = ref(fileData.value?.selected_extension || "unselected")

watch(selectedExtension, (newVal) => {
    if (fileData.value) {
        fileData.value!.selected_extension = newVal
    }
})

const getIcon = () => {
    switch(fileData.value?.file_type) {
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
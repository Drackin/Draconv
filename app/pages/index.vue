<template>

    <DNavbar />

    <section class="w-full flex space-x-2 my-2">
        <div v-if="isDragEnter" class="w-1/2 h-64 rounded-xl border border-dashed border-zinc-700 flex flex-col items-center justify-center space-y-2">
            <p>Drop Here</p>
            <Icon name="solar:arrow-down-outline" class="text-4xl" />
        </div>
        
        <div v-else class="w-1/2 h-64 rounded-xl border border-dashed border-zinc-700 flex flex-col items-center justify-center space-y-2">
        <p>Drag & Drop Here</p>
            <Icon name="solar:copy-outline" class="text-4xl" />
        </div>

        <div class="w-1/2 h-64 rounded-xl border border-zinc-700 flex flex-col items-center justify-center space-y-2">
            <p>or Select File</p>
            <DButton @click="selectFile" size="xl">Open</DButton>
        </div>
    </section>

    <Transition name="file-info">
        <DConvertItem v-if="fileStore.file_data" @cancel-request="() => isCancelling = true" :startConversion="convert" :progressPercent="progressPercent" conversion-status="idle" :isSupported="isSupported" :convertibles="convertibles" />
    </Transition>

    <DModal v-model="isCancelling">
        <p>Are you sure you want to cancel the process?</p>

        <template v-slot:footer>
            <div class="w-full flex space-x-2 justify-end">
                <DButton @click="isCancelling = false" class="font-semibold" variant="error">Close</DButton>
                <DButton @click="cancelConversion" class="font-semibold">Yes</DButton>
            </div>
        </template>
    </DModal>

    <DModal v-model="isDialogOpen" :title="(errorTitle as string) || 'Error'">
        <p class="overflow-auto">{{ errorDescription }}</p>

        <template v-slot:footer>
            <DButton @click="isDialogOpen = false" class="font-semibold self-end" variant="error">Close</DButton>
        </template>
    </DModal>

    <DModal :modelValue="fileStore.conversionStatus === 'success'" title="Conversion Successful">
        <p>The file has been successfully converted.</p>
        <p class="text-sm flex flex-col">
            <span class="text-gray-300">Output File:</span>
            <span>{{ conversionOutput?.new_file_path }}</span>
        </p>
        <p class="text-sm flex flex-col">
            <span class="text-gray-300">Total Time:</span>
            <span>{{ conversionOutput?.total_time }} seconds</span>
        </p>

        <template v-slot:footer>
            <DButton @click="fileStore.conversionStatus = 'idle'" class="font-semibold self-end" variant="error">Close</DButton>
        </template>
    </DModal>

    <DModal v-if="!isOnline" title="No Internet">
        Draconv needs internet connection to install required files on first install.
        If it's back but you still have this error, wait 5-10 seconds and try again.
        If it doesn't work, <button @click="() => openUrl('https://github.com/Drackin/Draconv/issues/new')" class="underline cursor-pointer">click to report.</button>

        <template #footer>
            <DButton @click="checkConnection" :disabled="checkingConnection">Try Again</DButton>
        </template>
    </DModal>

    <SettingsModal v-model="isSettingsOpen" />

    <DButton class="!p-3 fixed bottom-5 right-5" @click="toggleSettings">
        <Icon name="solar:settings-outline" class="text-2xl" />
    </DButton>

    <FFmpegInstall v-model="isFfmpegInstalling" :progressPercent="ffmpegInstallProgress" :installState="ffmpegInstallState" />

</template>

<script setup lang="ts">

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event"
import { getCurrentWindow } from '@tauri-apps/api/window';
import { openUrl } from "@tauri-apps/plugin-opener";
import { open } from '@tauri-apps/plugin-dialog';
import { useFileStore } from "~/lib/useFileStore";
import { useDialogs } from "~/lib/useDialogs";
import type { ConversionOutput } from "~/lib/types";
import supported_types from "@/assets/supported.json"

const appWindow = getCurrentWindow();

const isDragEnter = ref(false);
const isSupported = ref(false);
const isOnline = ref(true)
const checkingConnection = ref(false)

const file_path = ref<string | null>(null);
const convertibles = ref<string[]>([])
const errorTitle = ref<string | null>(null);
const errorDescription = ref<string | null>(null);
const progressPercent = ref(0);
const ffmpegInstallProgress = ref(0);
const ffmpegInstallState = ref("idle");
const conversionOutput = ref<ConversionOutput | null>(null);
const fileStore = useFileStore()
const {
    isSettingsOpen,
    isDialogOpen,
    isCancelling,
    isFfmpegInstalling,
    toggleSettings
} = useDialogs()
const fileData = fileStore.file_data

onMounted(async () => {
    document
    .getElementById('titlebar-minimize')
    ?.addEventListener('click', () => appWindow.minimize());
    
    document
    .getElementById('titlebar-close')
    ?.addEventListener('click', () => appWindow.close());
    
    listen("tauri://drag-enter", () => isDragEnter.value = true)
    listen("tauri://drag-leave", () => isDragEnter.value = false)
    listen("tauri://drag-drop", (event) => {
        isDragEnter.value = false
        file_path.value = (event as any).payload.paths[0]
        getFileInfo(file_path.value as string)
    })

    invoke("get_args")
    .then(args => {
        if((args as string[]).length > 1) {
            file_path.value = (args as string[])[1] as string
            getFileInfo(file_path.value)
        }
    })


    invoke("is_ffmpeg_installed")
    .then((installed) => {
        if(!installed) {
            if (checkConnection()) {
                isFfmpegInstalling.value = true
                ffmpegInstallState.value = "idle"
            }
        }
    })

    listen("conversion-started", () => {
        fileStore.conversionStatus = "processing"
    })

    listen<string>("conversion-finished", (e) => {
        conversionOutput.value = JSON.parse(e.payload) as ConversionOutput
        fileStore.conversionStatus = "success"
    })

    listen<string>("conversion-error", (e) => {
        fileStore.conversionStatus = "error"
        errorTitle.value = "Conversion Failed"
        errorDescription.value = e.payload || "An error occurred during the conversion process. Please try again."
        isDialogOpen.value = true
    })
    
    listen("conversion-cancelled", () => {
        fileStore.conversionStatus = "cancelled"
        errorTitle.value = "Conversion Cancelled"
        errorDescription.value = "The conversion process has been cancelled by the user."
        isDialogOpen.value = true
    })

    listen("cancelling-failed", (e) => {
        fileStore.conversionStatus = "error"
        errorTitle.value = "Cancelling Failed"
        errorDescription.value = e.payload as string
        isDialogOpen.value = true
    })

    listen("conversion-progress", (e) => progressPercent.value = e.payload as number)

    listen<number>("ffmpeg-install-progress", (e) => {
        ffmpegInstallProgress.value = e.payload
    })

    listen<string>("ffmpeg-install-state", (e) => {
        ffmpegInstallState.value = e.payload
    })

    window.addEventListener("contextmenu", (event) => event.preventDefault());
})

const convert = async () => {
    fileStore.conversionStatus = "processing"
    invoke("convert", {
        path: file_path.value,
        extension: fileStore.selected_extension,
        category: fileStore.file_data?.file_type
    }).catch(e => console.log(e))
}

const checkConnection = () => {
    checkingConnection.value = true
    invoke("check_connection").then((online) => {
        isOnline.value = online as boolean
    })
    
    checkingConnection.value = false
    return isOnline.value
}

const cancelConversion = async () => {
    isCancelling.value = false
    appWindow.emit("cancel-conversion")
}

const selectFile = async () => {
    try {
        file_path.value = await open({
            multiple: false,
            directory: false
        });
        getFileInfo(file_path.value as string)
    } catch (e) {
        alert(e)
    }

}

const getFileInfo = async (path: string) => {
    if (!path) {
        fileStore.$reset()
    }

    await fileStore.getFileData(path)

    if(fileStore.error) {
        errorTitle.value = fileStore.error.title
        errorDescription.value = fileStore.error.message
        isDialogOpen.value = true
        return
    }

    const type_key = fileStore.file_data?.file_type as keyof typeof supported_types

    if (!type_key || !supported_types[type_key]) {
        isSupported.value = false
        convertibles.value = []
        return
    }

    isSupported.value = supported_types[type_key].inputs.find(type => type === fileStore.file_data?.file_extension.toLowerCase()) ? true : false
    getConvertibleTypes()
}

const getConvertibleTypes = () => {
    convertibles.value = supported_types[fileStore.file_data?.file_type as keyof typeof supported_types].outputs
    convertibles.value = convertibles.value ? convertibles.value.filter(type => type !== fileData?.file_extension) : []
}
</script>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.file-info-enter-active {
    animation: bounce-in 0.5s;
}

.file-info-leave-active {
    animation: bounce-in 0.2s reverse linear;
}

@keyframes bounce-in {
    0% {
        transform: scale(0);
    }

    100% {
        transform: scale(1);
    }
}
</style>
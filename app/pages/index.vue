<template>

    <main class="w-full h-full flex flex-col">
        <section v-if="fileStore.files.length === 1" class="w-full flex space-x-2 transition-all duration-200 h-32 mb-2">
            <div
                class="w-1/2 rounded-xl border border-dashed border-zinc-700 flex flex-col items-center justify-center space-y-2">
                <p v-if="!isDragEnter">Drag & Drop Here</p>
                <p v-else>Drop Anywhere</p>

                <Icon v-if="!isDragEnter" name="solar:copy-outline" class="text-4xl" />
                <Icon v-if="isDragEnter" name="solar:arrow-down-outline" class="text-4xl" />
            </div>

            <div class="w-1/2 rounded-xl border border-zinc-700 flex flex-col items-center justify-center space-y-2">
                <p>or Select File</p>
                <DButton @click="selectFile" icon="solar:folder-with-files-outline">Open</DButton>
            </div>
        </section>

        <div v-if="!fileStore.files.length" class="w-full h-full flex flex-col space-y-2 items-center justify-center p-4 bg-[#111]/50 backdrop-blur-lg border border-dashed border-zinc-700 rounded-xl">
            <p class="text-xl">Drag and drop files here</p>
            <p class="text-gray-400">Or click to select files</p>

            <DButton @click="selectFile" icon="solar:folder-with-files-outline">Select Files</DButton>
        </div>

        <Transition name="fade">
            <div v-if="fileStore.files.length > 1" class="w-full flex border border-zinc-700 p-2 rounded-xl mb-2">
                <div class="flex flex-col w-full">
                    <p>Total Files: {{ fileStore.files.length }}</p>
                    <p>Converted Files: {{ fileStore.convertedFiles.length }}</p>
                </div>

                <section class="flex w-full justify-end space-x-2 h-12 whitespace-nowrap">
                    <DButton @click="selectFile" :disabled="fileStore.isProcessing" variant="neutral" icon="ic:round-plus">Add New</DButton>
                    <DButton @click="() => dialog.isDeletingAll = true" :disabled="fileStore.isProcessing" variant="error" icon="solar:trash-bin-trash-outline">Delete All</DButton>
                    <DButton @click="convertAll" :disabled="fileStore.isProcessing" icon="solar:refresh-outline">Convert All</DButton>
                </section>
            </div>
        </Transition>

        <TransitionGroup name="file-info" tag="ul" class="flex flex-col space-y-2 transition-all relative" @before-leave="onBeforeLeave">
            <DConvertItem
                v-for="file_data in fileStore.files"
                :key="file_data.id"
                :id="file_data.id"
                @cancel-request="() => requestCancel(file_data.id)"
                :startConversion="() => convert(file_data.id)"
            />
        </TransitionGroup>

    </main>

    <DModal v-model="dialog.isCancelling">
        <p>Are you sure you want to cancel the process?</p>

        <template v-slot:footer>
            <div class="w-full flex space-x-2 justify-end">
                <DButton @click="dialog.isCancelling = false" class="font-semibold" variant="error">Close</DButton>
                <DButton @click="cancelConversion" class="font-semibold">Yes</DButton>
            </div>
        </template>
    </DModal>

    <DModal v-model="dialog.isDeletingAll">
        <p>Are you sure you want to delete all selected files?</p>

        <template v-slot:footer>
            <div class="w-full flex space-x-2 justify-end">
                <DButton @click="dialog.isDeletingAll = false" class="font-semibold" variant="error">Close</DButton>
                <DButton @click="() => { fileStore.$reset(); dialog.isDeletingAll = false }" class="font-semibold">Yes
                </DButton>
            </div>
        </template>
    </DModal>

    <DModal v-model="dialog.isDialogOpen" :title="(errorTitle as string) || 'Error'">
        <p class="overflow-auto">{{ errorDescription }}</p>

        <template v-slot:footer>
            <DButton @click="dialog.isDialogOpen = false" class="font-semibold self-end" variant="error">Close</DButton>
        </template>
    </DModal>

    <DModal v-model="dialog.isConversionInfoOpen" title="Conversion Successful">
        <p>The file has been successfully converted.</p>
        <p class="text-sm flex flex-col break-all">
            <span class="text-gray-300">Output File:</span>
            <span>{{ dialog.selectedConversionInfo?.new_file_path }}</span>
        </p>
        <p class="text-sm flex flex-col">
            <span class="text-gray-300">Total Time:</span>
            <span>{{ formatTime() }}</span>
        </p>

        <template v-slot:footer>
            <DButton @click="dialog.toggleConversionInfo" class="font-semibold self-end" variant="error">Close</DButton>
        </template>
    </DModal>

    <DModal v-if="!isOnline" title="No Internet">
        Draconv needs internet connection to install required files on first install.
        If it's back but you still have this error, wait 5-10 seconds and try again.
        If it doesn't work, <button @click="() => openUrl('https://github.com/Drackin/Draconv/issues/new')"
            class="underline cursor-pointer">click to report.</button>

        <template #footer>
            <DButton @click="checkConnection" :disabled="checkingConnection">Try Again</DButton>
        </template>
    </DModal>

    <SettingsModal v-model="dialog.isSettingsOpen" />

    <DButton class="!p-3 fixed bottom-5 right-5" @click="dialog.toggleSettings">
        <Icon name="solar:settings-outline" class="text-2xl" />
    </DButton>

    <FFmpegInstall v-model="dialog.isFfmpegInstalling" :progressPercent="ffmpegInstallProgress"
        :installState="ffmpegInstallState" />

</template>

<script setup lang="ts">

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event"
import { getCurrentWindow } from '@tauri-apps/api/window';
import { openUrl } from "@tauri-apps/plugin-opener";
import { open } from '@tauri-apps/plugin-dialog';
import { useFileStore } from "~/lib/useFileStore";
import { useDialogs } from "~/lib/useDialogs";
import type { CompletedJob } from "~/lib/types";

const appWindow = getCurrentWindow();

const isDragEnter = ref(false);
const isOnline = ref(true)
const checkingConnection = ref(false)

const file_path = ref<string | null>(null);
const errorTitle = ref<string | null>(null);
const errorDescription = ref<string | null>(null);
const idToCancel = ref<string | null>(null);
const ffmpegInstallProgress = ref(0);
const ffmpegInstallState = ref("idle");
const fileStore = useFileStore()
const dialog = useDialogs()

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
        const files = (event as any).payload.paths

        for (let index = 0; index < files.length; index++) {
            const file_path = files[index];
            getFileInfo(file_path as string)
        }
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
                dialog.isFfmpegInstalling = true
                ffmpegInstallState.value = "idle"
            }
        }
    })

    listen<{ id: string }>("job-started", (e) => {
        fileStore.files.find(f => f.id === e.payload.id)!.conversionStatus = "processing"
    })

    listen<CompletedJob>("job-completed", (e) => {
        fileStore.files.find(f => f.id === e.payload.id)!.conversionStatus = "success"
        fileStore.convertedFiles.push(e.payload)
    })

    listen<CompletedJob>("all-jobs-completed", (e) => {
        fileStore.isProcessing = false
    })

    listen<{ id: string, error: string }>("job-failed", (e) => {
        fileStore.files.find(f => f.id === e.payload.id)!.conversionStatus = "failed"
        errorTitle.value = "Conversion Failed"
        errorDescription.value = e.payload.error || "An error occurred during the conversion process. Please try again."
        fileStore.isProcessing = false
        dialog.isDialogOpen = true
    })
    
    listen<{ id: string }>("job-cancelled", (e) => {
        fileStore.files.find(f => f.id === e.payload.id)!.conversionStatus = "cancelled"
        errorTitle.value = "Conversion Cancelled"
        errorDescription.value = "The conversion process has been cancelled by the user."
        fileStore.isProcessing = false
        dialog.isDialogOpen = true
    })

    listen<{ id: string, progress: number }>("job-progress", (e) => {
        const payload = e.payload
        
        const file = fileStore.files.find(f => f.id === payload.id)
        
        if (file) {
            file.progress = payload.progress
        }
    })

    listen<number>("ffmpeg-install-progress", (e) => {
        ffmpegInstallProgress.value = e.payload
    })

    listen<string>("ffmpeg-install-state", (e) => {
        ffmpegInstallState.value = e.payload
    })

    window.addEventListener("contextmenu", (event) => event.preventDefault());
})

const convert = async (id: string) => {
    const file = fileStore.files.map(f => ({
        id: f.id,
        path: f.full_path,
        extension: f.selected_extension,
        category: f.file_type,
    })).find(f => f.id === id)

    fileStore.isProcessing = true
    
    await invoke("convert", file)
}

const convertAll = async () => {
    const jobs = fileStore.files
    .filter(file => file.selected_extension !== "unselected")
    .map(file => ({
        id: file.id,
        path: file.full_path,
        extension: file.selected_extension,
        category: file.file_type,
    }))

    fileStore.isProcessing = true

    await invoke("add_all_jobs", { files: jobs })
}

const checkConnection = () => {
    checkingConnection.value = true
    invoke("check_connection").then((online) => {
        isOnline.value = online as boolean
    })
    
    checkingConnection.value = false
    return isOnline.value
}

const requestCancel = (id: string) => {
    idToCancel.value = id
    dialog.isCancelling = true
}

const cancelConversion = async () => {
    dialog.isCancelling = false
    await invoke("cancel_job", { id: idToCancel.value })
    fileStore.isProcessing = false
    idToCancel.value = null
}

const formatTime = () => {
    const totalSeconds = dialog.selectedConversionInfo?.total_time || 0
    const timeArray: string[] = []
    const hours = Math.floor(totalSeconds / 3600)
    const minutes = Math.floor((totalSeconds % 3600) / 60)
    const seconds = Math.floor(totalSeconds % 60)

    if(hours > 0) hours === 1 ? timeArray.push("1 hour") : timeArray.push(`${hours} hours`)
    if(minutes > 0) minutes === 1 ? timeArray.push("1 minute") : timeArray.push(`${minutes} minutes`)
    timeArray.push(`${seconds} seconds`)

    return timeArray.join(" ")
}

const selectFile = async () => {
    try {
        const files = await open({
            multiple: true,
            directory: false
        });

        if (files) {
            for (let index = 0; index < files.length; index++) {
            const file_path = files[index];
                getFileInfo(file_path as string)
            }
        }
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
        dialog.isDialogOpen = true
        return
    }
}

// to avoid item teleports to top bug when an item removed from the conversion list
const onBeforeLeave = (el: Element) => {
    const element = el as HTMLElement;
    
    const { width, height } = window.getComputedStyle(element);

    element.style.left = `${element.offsetLeft}px`;
    element.style.top = `${element.offsetTop}px`;
    
    element.style.width = width; 
    element.style.height = height;

    element.style.position = 'absolute';
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
    opacity: 0;
    position: absolute;
    width: 100%;
}

.file-info-move {
    transition: transform 0.35s cubic-bezier(.22, 1, .36, 1);
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
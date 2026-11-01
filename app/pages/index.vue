<template>

    <main ref="pageContent" class="w-full min-h-full flex flex-col" :class="{ 'mb-20': hasVerticalOverflow }">
        <Transition name="layout" mode="out-in" appear>
            <section v-if="fileStore.files.length === 1" key="single-file" class="w-full shrink-0 flex space-x-2 transition-all duration-200 h-32 mb-2">
                <div
                    class="w-1/2 rounded-xl border border-dashed border-zinc-700 flex flex-col items-center justify-center space-y-2">
                    <p v-if="!isDragEnter">Drag & Drop Here</p>
                    <p v-else>Drop Anywhere</p>

                    <Icon v-if="!isDragEnter" name="lucide:copy" class="text-4xl" />
                    <Icon v-if="isDragEnter" name="lucide:arrow-down" class="text-4xl" />
                </div>

                <div class="w-1/2 rounded-xl border border-zinc-700 flex flex-col items-center justify-center space-y-2">
                    <p>or Select File</p>
                    <UiButton @click="selectFile">
                        <Icon name="lucide:folders" />
                        Select Files
                    </UiButton>
                </div>
            </section>

            <div v-else-if="!fileStore.files.length" key="empty-files" class="w-full flex-1 flex flex-col space-y-2 items-center justify-center p-4 bg-[#111]/50 backdrop-blur-lg border border-dashed border-zinc-700 rounded-xl">
                <p class="text-xl">Drag and drop files here</p>
                <p class="text-gray-400">Or click to select files</p>

                <UiButton @click="selectFile">
                    <Icon name="lucide:folders" />
                    Select Files
                </UiButton>
            </div>

            <div v-else key="multiple-files" class="relative isolate mb-3 shrink-0 overflow-hidden rounded-2xl border border-white/10 bg-zinc-950/65 shadow-xl shadow-black/10 backdrop-blur-xl">
                <div class="pointer-events-none absolute -right-16 -top-24 -z-10 h-56 w-56 rounded-full bg-white/10 blur-3xl"></div>
                <div class="relative flex flex-col gap-4 p-4 sm:p-5 lg:flex-row lg:items-center">
                    <div class="flex min-w-0 flex-1 items-center gap-3">
                        <div class="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-white text-black shadow-lg shadow-white/10">
                            <Icon name="lucide:layers-3" class="text-xl" />
                        </div>
                        <div class="min-w-0">
                            <div class="flex items-center gap-2">
                                <p class="font-semibold tracking-tight">Batch conversion</p>
                                <span class="rounded-full border border-white/10 bg-white/5 px-2 py-0.5 text-[10px] font-medium uppercase tracking-wider text-zinc-400">
                                    {{ fileStore.files.length }} files
                                </span>
                            </div>
                            <p class="mt-0.5 text-xs text-zinc-400">
                                {{ convertedFileCount }} of {{ fileStore.files.length }} files converted
                            </p>
                        </div>
                    </div>

                    <div class="w-full shrink-0 lg:max-w-xs lg:flex-1">
                        <div class="mb-1.5 flex items-center justify-between text-[11px] font-medium text-zinc-400">
                            <span>Progress</span>
                            <span class="text-zinc-200">{{ conversionProgress }}%</span>
                        </div>
                        <div class="h-2 overflow-hidden rounded-full bg-white/10" role="progressbar" aria-label="Batch conversion progress" :aria-valuenow="conversionProgress" aria-valuemin="0" aria-valuemax="100">
                            <div class="h-full rounded-full bg-white transition-[width] duration-500 ease-out" :style="{ width: `${conversionProgress}%` }"></div>
                        </div>
                    </div>

                    <section class="grid w-full grid-cols-2 gap-2 sm:flex sm:flex-wrap lg:w-auto lg:justify-end">
                        <UiButton @click="selectFile" :disabled="fileStore.isProcessing" variant="secondary" class="w-full sm:w-auto">
                            <Icon name="lucide:file-plus-2" />
                            Add New
                        </UiButton>
                        <UiButton @click="() => dialog.isDeletingAll = true" :disabled="fileStore.isProcessing" variant="destructive" class="w-full sm:w-auto">
                            <Icon name="lucide:trash-2" />
                            Delete All
                        </UiButton>
                        <UiButton @click="convertAll" :disabled="!canConvertAll" class="col-span-2 w-full sm:w-auto">
                            <Icon name="lucide:play" />
                            Convert All
                        </UiButton>
                    </section>
                </div>
            </div>
        </Transition>

        <TransitionGroup name="file-info" tag="ul" class="flex flex-col space-y-2 transition-all relative" @before-leave="onBeforeLeave">
            <ConvertItem
                v-for="file_data in fileStore.files"
                :key="file_data.id"
                :id="file_data.id"
                @cancel-request="() => requestCancel(file_data.id)"
                :startConversion="() => convert(file_data.id)"
            />
        </TransitionGroup>

    </main>

    <UiDialog v-model:open="dialog.isCancelling">
        <UiDialogContent>
            <UiDialogHeader>
                <UiDialogTitle>Cancel Conversion</UiDialogTitle>
            </UiDialogHeader>
            
            <p>Are you sure you want to cancel the process? You will lose all progress. But you will be able to restart it later.</p>
            <UiDialogFooter class="w-full flex space-x-2 justify-end">
                <UiButton @click="dialog.isCancelling = false" class="font-semibold" variant="destructive">No</UiButton>
                <UiButton @click="cancelConversion" class="font-semibold">
                    Yes
                </UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>

    <UiDialog v-model:open="dialog.isDeletingAll">
        <UiDialogContent class="w-full">
            <UiDialogHeader>
                <UiDialogTitle>Delete All Files</UiDialogTitle>
            </UiDialogHeader>

            <p>Are you sure you want to delete all selected files?</p>

            <UiDialogFooter class="w-full flex space-x-2 justify-end">
                <UiButton @click="dialog.isDeletingAll = false" class="font-semibold" variant="destructive">Cancel</UiButton>
                <UiButton @click="() => { fileStore.$reset(); dialog.isDeletingAll = false }" class="font-semibold">
                    Yes
                </UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>

    <UiDialog v-model:open="dialog.isDialogOpen">

        <UiDialogContent>
            
            <UiDialogHeader>
                <UiDialogTitle>{{ (errorTitle as string) || 'Error' }}</UiDialogTitle>
            </UiDialogHeader>

            <p class="overflow-auto">{{ errorDescription }}</p>

            <UiDialogFooter>
                <UiButton @click="() => dialog.isDialogOpen = false" variant="destructive">
                    <Icon name="lucide:check" />
                    Close
                </UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>

    <UiDialog v-model:open="dialog.isConversionInfoOpen">
        <UiDialogContent>
            <UiDialogHeader>
                <UiDialogTitle>Conversion Successful</UiDialogTitle>
                <UiDialogDescription>The file has been successfully converted.</UiDialogDescription>
            </UiDialogHeader>
            <p class="text-sm flex flex-col break-all">
                <span class="text-gray-300">Output File:</span>
                <span>{{ dialog.selectedConversionInfo?.new_file_path }}</span>
            </p>
            <p class="text-sm flex flex-col">
                <span class="text-gray-300">Total Time:</span>
                <span>{{ formatTime() }}</span>
            </p>

            <UiDialogFooter>
                <UiButton @click="openFile(dialog.selectedConversionInfo?.new_file_path)" :disabled="!dialog.selectedConversionInfo?.new_file_path" class="font-semibold self-end" variant="outline">
                    <Icon name="lucide:arrow-up-right" />
                    Open
                </UiButton>
                <UiButton @click="dialog.toggleConversionInfo" class="font-semibold self-end">
                    <Icon name="lucide:check" />
                    Close
                </UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>

    <UiDialog v-bind:open="!isOnline">
        <UiDialogContent>
            <UiDialogHeader>
                <UiDialogTitle>No Internet Connection</UiDialogTitle>
                <UiDialogDescription>
                    Draconv requires an internet connection to install necessary files during the first installation.
                </UiDialogDescription>
            </UiDialogHeader>

            <p class="text-sm">
                Please check your internet connection and try again.
                If your internet is back but you still have this error, wait 5-10 seconds and try again.
                If the issue persists, click on "Report" below to report the issue to the developer.
            </p>

            <UiDialogFooter>
                <UiButton @click="openUrl('https://github.com/Drackin/Draconv/issues/new')" variant="secondary">
                    <Icon name="lucide:arrow-up-right" />
                    Report Issue
                </UiButton>
                <UiButton @click="checkConnection" :disabled="checkingConnection">
                    <Icon name="lucide:refresh-cw" />
                    Retry
                </UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>

    <UiToaster position="top-center" rich-colors theme="dark" :duration="3000" />

    <DialogSettingsModal v-model="dialog.isSettingsOpen" />

    <UiButton class="size-11 p-0 fixed bottom-5 right-5" @click="dialog.toggleSettings">
        <Icon name="lucide:settings" class="text-2xl" />
    </UiButton>

    <DialogFFmpegInstall v-model="dialog.isFfmpegInstalling" :progressPercent="ffmpegInstallProgress"
        :installState="ffmpegInstallState" />

</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event"
import { openUrl } from "@tauri-apps/plugin-opener";
import { open } from '@tauri-apps/plugin-dialog';
import { toast } from "vue-sonner"
import { useResizeObserver } from "@vueuse/core"
import { useFileStore } from "~/lib/useFileStore";
import { useDialogs } from "~/lib/useDialogs";
import type { CompletedJob } from "~/lib/types";

const pageContent = ref<HTMLElement | null>(null)
const hasVerticalOverflow = ref(false)
const scrollContainer = computed(() => pageContent.value?.parentElement)

useResizeObserver([pageContent, scrollContainer], () => {
    const content = pageContent.value
    const container = scrollContainer.value
    if (!content || !container) return

    const style = window.getComputedStyle(container)
    const availableHeight = container.clientHeight - parseFloat(style.paddingTop) - parseFloat(style.paddingBottom)

    // Exclude the added margin so it disappears when the content fits again.
    hasVerticalOverflow.value = content.offsetHeight > availableHeight + 1
})

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
const filesReadyToConvert = computed(() =>
    fileStore.files.filter(file => file.selected_extension && file.selected_extension !== "unselected")
)
const canConvertAll = computed(() =>
    !fileStore.isProcessing && filesReadyToConvert.value.length > 0
)
const convertedFileCount = computed(() =>
    fileStore.files.filter(file => file.conversionStatus === "success").length
)
const conversionProgress = computed(() => {
    if (!fileStore.files.length) return 0

    return Math.round((convertedFileCount.value / fileStore.files.length) * 100)
})

onMounted(async () => {
    listen("tauri://drag-enter", () => isDragEnter.value = true)
    listen("tauri://drag-leave", () => isDragEnter.value = false)
    listen("tauri://drag-drop", (event) => {
        isDragEnter.value = false
        const files = (event as any).payload.paths
        console.log("Dropped files:", files)

        for (let index = 0; index < files.length; index++) {
            const file_path = files[index];
            getFileInfo(file_path as string)
        }

        toast.success(files.length > 1 ? `${files.length} files added successfully.` : `File added successfully.`)
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
        toast.success(`File converted successfully.`, {
            duration: 4000,
        })
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
        outputFileName: f.output_file_name,
        extension: f.selected_extension,
        category: f.file_type,
    })).find(f => f.id === id)

    fileStore.isProcessing = true
    
    await invoke("convert", file)
}

const convertAll = async () => {
    if (!canConvertAll.value) return

    const jobs = filesReadyToConvert.value
    .filter(file => file.conversionStatus !== "success")
    .map(file => ({
        id: file.id,
        path: file.full_path,
        output_file_name: file.output_file_name,
        extension: file.selected_extension,
        category: file.file_type,
    }))

    fileStore.isProcessing = true

    await invoke("add_all_jobs", { files: jobs })
}

const openFile = async (path?: string) => {
    if (!path) return

    try {
        await invoke("open_file", { path })
    } catch (error) {
        dialog.isConversionInfoOpen = false
        errorTitle.value = "Unable to Open File"
        errorDescription.value = error instanceof Error ? error.message : String(error)
        dialog.isDialogOpen = true
    }
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
                await getFileInfo(file_path as string)
            }

            toast.success(files.length > 1 ? `${files.length} files added successfully.` : `File added successfully.`, {
                duration: 4000,
            })
        }
    } catch (e) {
        errorTitle.value = "File Selection Error"
        errorDescription.value = e instanceof Error ? e.message : String(e)
        dialog.isDialogOpen = true
    }

}

const getFileInfo = async (path: string) => {
    if (!path) {
        fileStore.$reset()
        return
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
.layout-enter-active,
.layout-leave-active {
    transition: opacity 0.25s ease, transform 0.3s cubic-bezier(.22, 1, .36, 1);
}

.layout-enter-from {
    opacity: 0;
    transform: translateY(8px) scale(0.985);
}

.layout-leave-to {
    opacity: 0;
    transform: translateY(-8px) scale(0.985);
}

.file-info-enter-active,
.file-info-leave-active {
    transition: opacity 0.25s ease, transform 0.3s cubic-bezier(.22, 1, .36, 1);
}

.file-info-enter-from {
    opacity: 0;
    transform: translateY(10px) scale(0.985);
}

.file-info-leave-to {
    opacity: 0;
    transform: translateY(-6px) scale(0.985);
}

.file-info-leave-active {
    position: absolute;
    width: 100%;
}

.file-info-move {
    transition: transform 0.35s cubic-bezier(.22, 1, .36, 1);
}

</style>

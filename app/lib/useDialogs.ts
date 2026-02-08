import type { CompletedJob } from "./types";
import { useFileStore } from "./useFileStore";

export const useDialogs = defineStore("dialog", () => {
    const isDialogOpen = ref(false);
    const errorTitle = ref<string | null>(null);
    const errorDescription = ref<string | null>(null);
    
    const isCancelling = ref(false);
    const isSettingsOpen = ref(false);
    const isTableOpen = ref(false);
    const isModeInfoOpen = ref(false);
    const isEncoderInfoOpen = ref(false);
    const isFfmpegInstalling = ref(false);
    const isDeletingAll = ref(false);
    const disableClickOutsideSettings = ref(false);
    const selectedConversionInfo = ref<CompletedJob | null>(null);
    const isConversionInfoOpen = ref(false);
    
    const ffmpegInstallState = ref("idle");
    
    const openErrorDialog = (title: string, description: string) => {
        errorTitle.value = title;
        errorDescription.value = description;
        isDialogOpen.value = true;
    };

    const toggleSettings = async () => {
        isSettingsOpen.value = !isSettingsOpen.value;
    };

    const toggleConversionInfo = async (id?: string) => {
        if(isConversionInfoOpen.value) {
            isConversionInfoOpen.value = false;
            selectedConversionInfo.value = null;
            return;
        }
        selectedConversionInfo.value = useFileStore().convertedFiles.find(f => f.id === id) as CompletedJob;
        if(selectedConversionInfo.value) isConversionInfoOpen.value = true;
    };

    watch(
        [isDialogOpen, isSettingsOpen, isFfmpegInstalling, isDeletingAll, isCancelling],
        ([dialog, settings, ffmpeg, deleting, cancelling]) => {
            if (dialog || settings || ffmpeg || deleting || cancelling) {
                document.body.style.overflow = "hidden";
            } else {
                document.body.style.overflow = "auto";
            }
        }
    );

    return {
        isDialogOpen,
        errorTitle,
        errorDescription,
        isCancelling,
        isSettingsOpen,
        isTableOpen,
        isModeInfoOpen,
        isEncoderInfoOpen,
        isFfmpegInstalling,
        isDeletingAll,
        isConversionInfoOpen,
        disableClickOutsideSettings,
        selectedConversionInfo,
        openErrorDialog,
        toggleSettings,
        toggleConversionInfo,
    };
})
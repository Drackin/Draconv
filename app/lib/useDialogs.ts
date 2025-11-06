import { save } from "@tauri-apps/plugin-dialog";

export const useDialogs = () => {
    const isDialogOpen = ref(false);
    const errorTitle = ref<string | null>(null);
    const errorDescription = ref<string | null>(null);

    const isCancelling = ref(false);
    const isSettingsOpen = ref(false);
    const isTableOpen = ref(false);
    const isModeInfoOpen = ref(false);
    const isFfmpegInstalling = ref(false);

    const ffmpegInstallState = ref("idle");

    const openErrorDialog = (title: string, description: string) => {
        errorTitle.value = title;
        errorDescription.value = description;
        isDialogOpen.value = true;
    };

    const toggleSettings = async () => {
        isSettingsOpen.value = !isSettingsOpen.value;
    };

    return {
        isDialogOpen,
        errorTitle,
        errorDescription,
        isCancelling,
        isSettingsOpen,
        isTableOpen,
        isModeInfoOpen,
        isFfmpegInstalling,
        openErrorDialog,
        toggleSettings,
    };
}
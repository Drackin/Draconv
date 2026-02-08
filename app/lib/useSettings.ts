import { invoke } from "@tauri-apps/api/core";

type Settings = {
    conversion_mode: string,
    max_concurrency: number,
    open_when_finished: boolean,
    default_encoder: string,
}

export const useSettings = () => {
    const settings = ref<Settings>({
        conversion_mode: "",
        max_concurrency: 1,
        open_when_finished: true,
        default_encoder: "libx264"
    });

    /* const saveSettings = async () => {
        if(!settings.value) return;
        await invoke("save_settings", { newSettings: settings.value });
    } */

    const loadSettings = async () => {
        settings.value = await invoke<Settings>("load_settings");
    }

    watch(settings, async (changedSettings) => {
        if(changedSettings) await invoke("save_settings", { newSettings: changedSettings });
    }, { deep: true });

    return { settings, loadSettings };
}
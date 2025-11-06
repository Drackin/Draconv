import { invoke } from "@tauri-apps/api/core";

type Settings = {
    conversion_mode: string,
    open_when_finished: boolean,
}

export const useSettings = () => {
    const settings = ref<Settings>({
        conversion_mode: "",
        open_when_finished: true,
    });

    /* const saveSettings = async () => {
        if(!settings.value) return;
        await invoke("save_settings", { newSettings: settings.value });
    } */

    const loadSettings = async () => {
        settings.value = await invoke<Settings>("load_settings");
    }

    watch(settings, async (changedSettings) => {
        if(changedSettings)
        await invoke("save_settings", { newSettings: changedSettings });
    }, { deep: true });

    return { settings, loadSettings };
}
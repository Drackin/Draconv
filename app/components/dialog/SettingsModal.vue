<template>
    <UiDialog v-model:open="isOpen">
        <UiDialogContent class="h-[80vh] w-[70vw] p-0">
            
            <UiDialogHeader class="px-6 py-6 pb-0!">
                <UiDialogTitle>Settings</UiDialogTitle>
                <UiDialogDescription>
                    You can change the settings of the app here.
                </UiDialogDescription>
            </UiDialogHeader>
        
            <ul class="flex min-h-0 flex-col px-4 space-y-2 overflow-x-hidden overflow-y-auto w-full">

                <DialogSettingsItem title="Open File When Finished" subtitle="Auto open the file when the conversion is finished." position="row">
                    <label class="inline-flex items-center cursor-pointer">
                        <input v-model="settings.open_when_finished" type="checkbox" value="" class="sr-only peer" />
                        <div class="relative w-11 h-6 bg-muted rounded-full peer peer-checked:bg-white peer-checked:after:bg-muted peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full after:content-[''] after:absolute after:top-0.5 after:inset-s-0.5 after:bg-white after:border-muted after:border after:rounded-full after:h-5 after:w-5 after:transition-all"></div>
                    </label>
                </DialogSettingsItem>

                <DialogSettingsItem title="Max Concurrency" subtitle="Count of processes at once." position="row">
                    <UiNumberField id="max-concurrency" v-model="settings.max_concurrency" :default-value="settings.max_concurrency" :min="1">
                        <UiNumberFieldContent>
                            <UiNumberFieldDecrement />
                            <UiNumberFieldInput />
                            <UiNumberFieldIncrement />
                        </UiNumberFieldContent>
                    </UiNumberField>
                </DialogSettingsItem>

                <DialogSettingsItem title="Default Encoder" class="w-full" subtitle="Click on the warning icon before change." subtitle-variant="warning">

                    <UiSelect v-model="settings.default_encoder">
                        <UiSelectTrigger class="w-full">
                            <UiSelectValue placeholder="Select an encoder" />
                        </UiSelectTrigger>

                        <UiSelectContent>
                            <UiSelectGroup>
                                <UiSelectLabel>Encoders</UiSelectLabel>

                                <UiSelectItem v-for="encoder in options.encoders" :key="encoder.value" :value="encoder.value">
                                    {{ encoder.name }}
                                </UiSelectItem>
                            </UiSelectGroup>
                        </UiSelectContent>
                    </UiSelect>

                    <UiButton variant="destructive" class="" @click="isEncoderInfoOpen = true">
                        <Icon name="lucide:triangle-alert" class="text-xl" />
                    </UiButton>
                </DialogSettingsItem>

                <DialogSettingsItem title="Conversion Mode" subtitle="Click on the warning icon before change." subtitle-variant="warning">
                    <UiSelect v-model="settings.conversion_mode">
                        <UiSelectTrigger class="w-full!">
                            <UiSelectValue placeholder="Select a mode" />
                        </UiSelectTrigger>

                        <UiSelectContent>
                            <UiSelectGroup>
                                <UiSelectLabel>Modes</UiSelectLabel>

                                <UiSelectItem v-for="mode in options.modes" :key="mode.value" :value="mode.value">
                                    {{ mode.name }}
                                </UiSelectItem>
                            </UiSelectGroup>
                        </UiSelectContent>
                    </UiSelect>

                    <UiButton variant="destructive" @click="isModeInfoOpen = true">
                        <Icon name="lucide:triangle-alert" class="text-xl" />
                    </UiButton>
                </DialogSettingsItem>

                <DialogSettingsItem title="Supported Formats" subtitle="All supported input and output formats." position="row">
                    <UiButton @click="openFormatsTable">
                        <Icon name="lucide:table" />
                        Click to View
                    </UiButton>
                </DialogSettingsItem>

                <DialogSettingsItem
                    title="Links & Donations"
                    subtitle="
                        You can find the links about the app here.
                        Also, this app is totally free but if you would like to
                        support me for motivation and my education, you can find the links below.
                    "
                >
                    <div class="flex flex-wrap gap-2 justify-center">
                        <span class="text-gray-400 text-xs mt-2">
                            NOTE: You make me the happiest person ever if you give repository a star ⭐
                        </span>
                        <UiButton @click="goTo('github')">
                            <Icon name="simple-icons:github" />
                            GitHub Repo
                        </UiButton>
                        
                        <UiButton @click="goTo('buymeacoffee')" class="bg-[#FFDD00] hover:bg-[#FFDD00]/90">
                            <Icon name="simple-icons:buymeacoffee" />
                            BuyMeACoffee
                        </UiButton>
                        
                        <UiButton @click="goTo('patreon')" class="bg-[#65CD91] hover:bg-[#65CD91]/90">
                            <Icon name="simple-icons:patreon" />
                            Patreon
                        </UiButton>
                        
                        <UiButton @click="goTo('sponsor')" class="bg-[#DB61A2] hover:bg-[#DB61A2]/90">
                            <Icon name="simple-icons:githubsponsors" class="text-[#9f326c]" />
                            GitHub Sponsor
                        </UiButton>

                        <UiButton @click="goTo('drackin')">
                            <Icon name="lucide:code-xml" />
                            Developer Website
                        </UiButton>
                    </div>

                </DialogSettingsItem>

                <p class="text-xs text-muted-foreground self-center">
                    Version: v{{ version }}
                </p>
            </ul>

            <UiDialogFooter class="flex justify-end gap-2 pb-6 px-6">
                <UiButton @click="reset" variant="secondary">
                    <Icon name="lucide:rotate-ccw" />
                    Reset
                </UiButton>

                <UiButton @click="() => isOpen = false">
                    <Icon name="lucide:check" />
                    Save
                </UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>

    <DialogModeInfo v-model="isModeInfoOpen" />
    <FormatsTable v-model="isTableOpen" />
    <DialogEncoderInfo v-model="isEncoderInfoOpen" />
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { getVersion } from '@tauri-apps/api/app';
import { useDialogs } from '~/lib/useDialogs';
import { useSettings } from '~/lib/useSettings';

const isOpen = defineModel<boolean>();
const version = ref<string | null>(null)

const dialog = useDialogs()
const { isModeInfoOpen, isTableOpen, isEncoderInfoOpen } = toRefs(dialog)
const disableSettingsClose = computed(() => isModeInfoOpen || isTableOpen);

onMounted(async () => {
    version.value = await getVersion()
})

const options = {
    modes: [
        {
            name: "Normal",
            value: "normal"
        },
        {
            name: "Hardware Acceleration",
            value: "hwaccel"
        },
        {
            name: "Lossless",
            value: "lossless"
        }
    ],
    encoders: [
        {
            name: "H.264",
            value: "libx264",
        },
        {
            name: "H.265 (Recommended)",
            value: "libx265",
        },
        {
            name: "AV1 (Limited GPU Support)",
            value: "libsvtav1",
        },
    ]
}

const goTo = async (id: string) => {
    const links = {
        github: "https://github.com/Drackin/Draconv",
        buymeacoffee: "https://buymeacoffee.com/drackin",
        patreon: "https://patreon.com/c/Drackin",
        sponsor: "https://github.com/sponsors/Drackin",
        drackin: "https://drackin.dev",
    }

    await openUrl(links[id as keyof typeof links])
}

const openFormatsTable = async () => {
    isOpen.value = false;
    await nextTick();
    isTableOpen.value = true;
}

const reset = async () => {
    await invoke("reset_settings")
    loadSettings()
}

const { settings, loadSettings } = useSettings();

watch(isOpen, (newIsOpen) => {
    if(newIsOpen) {
        loadSettings();
    }
})
</script>

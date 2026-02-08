<template>
    <DModal v-model="isOpen" :close-click-outside="!disableSettingsClose" title="Settings" class="w-[50vw]">
        <ul class="flex flex-col space-y-5">

            <SettingItem title="Open File When Finished" position="row">
                <label class="inline-flex items-center cursor-pointer">
                    <input v-model="settings.open_when_finished" type="checkbox" value="" class="sr-only peer" />
                    <div class="relative w-11 h-6 bg-neutral-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-green-600 dark:peer-checked:bg-green-600"></div>
                </label>
            </SettingItem>

            <SettingItem title="Max Concurrency" position="row">
                <input v-model="settings.max_concurrency" type="number" class="p-2 w-20 bg-transparent border border-neutral-800 rounded-xl" />
            </SettingItem>

            <SettingItem title="Default Encoder" subtitle="Click on the warning icon before change." subtitle-variant="warning">
                <DSelect v-model="settings.default_encoder" class="w-full" :placeholder="settings.default_encoder" :options="options.encoders" />

                <DButton class="" variant="warning" @click="isEncoderInfoOpen = true">
                    <Icon name="solar:danger-triangle-outline" class="text-xl" />
                </DButton>
            </SettingItem>

            <SettingItem title="Conversion Mode" subtitle="Click on the warning icon before change." subtitle-variant="warning">
                <DSelect v-model="settings.conversion_mode" class="w-full" :placeholder="settings.conversion_mode" :options="options.modes" />

                <DButton variant="warning" @click="isModeInfoOpen = true">
                    <Icon name="solar:danger-triangle-outline" class="text-xl" />
                </DButton>
            </SettingItem>

            <SettingItem title="Supported Formats" subtitle="All supported input and output formats.">
                <DButton variant="neutral" class="w-full" icon="solar:square-top-down-outline" @click="() => isTableOpen = true">
                    Click to View
                </DButton>
            </SettingItem>

            <SettingItem
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
                    <DButton @click="goTo('github')" icon="simple-icons:github" variant="neutral">
                        GitHub Repo
                    </DButton>
                    
                    <DButton @click="goTo('buymeacoffee')" icon="simple-icons:buymeacoffee" class="!bg-[#FFDD00] text-black">
                        BuyMeACoffee
                    </DButton>
                    
                    <DButton @click="goTo('patreon')" icon="simple-icons:patreon" class="!bg-[#65CD91] text-black">
                        Patreon
                    </DButton>
                    
                    <DButton @click="goTo('sponsor')" class="!bg-[#212830]">
                        <Icon name="simple-icons:githubsponsors" class="text-[#DB61A2] text-xl mr-2" />
                        GitHub Sponsor
                    </DButton>

                    <DButton @click="goTo('drackin')" icon="solar:global-outline" variant="neutral">
                        Developer Website
                    </DButton>
                </div>

            </SettingItem>
        </ul>

        <template v-slot:footer>
            <div class="w-full flex space-x-2 justify-end">
                <DButton @click="reset" variant="neutral" class="font-semibold text-sm w-full" icon="solar:refresh-outline">Reset</DButton>
                <DButton @click="isOpen = false" class="font-semibold text-sm w-full" icon="ic:round-check">Done</DButton>
            </div>
        </template>
    </DModal>

    <ModeInfos v-model="isModeInfoOpen" />
    <FormatsTable v-model="isTableOpen" />
    <EncoderInfos v-model="isEncoderInfoOpen" />
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { useDialogs } from '~/lib/useDialogs';
import { useSettings } from '~/lib/useSettings';

const isOpen = defineModel<boolean>();

const dialog = useDialogs()
const { isModeInfoOpen, isTableOpen, isEncoderInfoOpen } = toRefs(dialog)
const disableSettingsClose = computed(() => isModeInfoOpen || isTableOpen);

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
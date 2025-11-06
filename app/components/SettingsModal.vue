<template>
    <DModal v-model="isOpen" :close-click-outside="!disableSettingsClose" title="Settings" class="w-[50vw]">
        <ul class="flex flex-col space-y-5">
            <li class="flex w-full justify-between">
                <p>Open file when finished</p>

                <label class="inline-flex items-center cursor-pointer">
                    <input v-model="settings.open_when_finished" type="checkbox" value="" class="sr-only peer" />
                    <div class="relative w-11 h-6 bg-neutral-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-green-600 dark:peer-checked:bg-green-600"></div>
                </label>
            </li>

            <li class="flex flex-col w-full">
                <p class="flex flex-col">
                    <span>
                        Conversion Mode
                    </span>

                    <span class="text-yellow-600 text-xs">
                        Click on the warning icon before change.
                    </span>
                </p>

                <section class="flex items-center w-full space-x-2">
                    <select v-model="settings.conversion_mode" name="type" id="type-selector" class="w-28 px-1 py-2 bg-[#111] w-full h-full rounded-lg backdrop-blur-sm border border-neutral-700">
                        <option v-for="mode in conversionModes" :value="mode.value"
                            :selected="settings.conversion_mode === mode.value">{{ mode.name }}</option>
                    </select>

                    <DButton class="w-10 h-10" variant="warning" @click="isModeInfoOpen = true">
                        <Icon name="solar:danger-triangle-outline" class="text-xl" />
                    </DButton>
                </section>
            </li>

            <li class="flex flex-col w-full">
                <p class="flex flex-col">
                    <span>
                        Supported Formats
                    </span>

                    <span class="text-gray-400 text-xs">
                        All supported input and output formats.
                    </span>
                </p>

                <DButton variant="neutral" class="w-full border border-neutral-700 h-10" @click="() => isTableOpen = true">
                    Click to View
                </DButton>
            </li>
        </ul>

        <template v-slot:footer>
            <div class="w-full flex space-x-2 justify-end">
                <DButton @click="isOpen = false" class="font-semibold text-sm w-full">Done</DButton>
            </div>
        </template>
    </DModal>

    <ModeInfos v-model="isModeInfoOpen" />
    <FormatsTable v-model="isTableOpen" />
</template>

<script setup lang="ts">
import { useDialogs } from '~/lib/useDialogs';
import { useSettings } from '~/lib/useSettings';

const isOpen = defineModel<boolean>();

const { isModeInfoOpen, isTableOpen } = useDialogs()
const disableSettingsClose = computed(() => isModeInfoOpen.value || isTableOpen.value);

const conversionModes = [
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
]

const { settings, loadSettings } = useSettings();

watch(isOpen, (newIsOpen) => {
    if(newIsOpen) {
        loadSettings();
    }
})
</script>
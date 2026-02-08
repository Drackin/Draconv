<template>
    <DModal v-model="isOpen" :close-click-outside="false" title="Installing FFmpeg">

        <div class="flex flex-col space-y-4 relative">
            <section v-if="installState === 'idle'">
                FFmpeg not found. The Draconv requires FFmpeg to function properly. Click "Install" to begin the installation process.
            </section>

            <p class="text-gray-300 p-2 bg-neutral-900 border border-neutral-800 rounded-lg text-xs">
                This application uses FFmpeg, a free software licensed under the GNU General Public License (GPL) version 3.
                <br />
                <br />
                FFmpeg source code and license details are available at: <button class="underline cursor-pointer" @click="goFFmpegWebsite">https://ffmpeg.org/</button>
            </p>

            <section v-if="installState === 'downloading'" class="space-y-2">
                <p>{{ progressPercent }}% installed</p>
                <DProgress mode="percent" :percent="progressPercent" class="h-2" />
            </section>

            <section v-else-if="installState === 'installing'" class="space-y-2">
                <p>Installing FFmpeg...</p>
                <DProgress mode="immediate" class="h-2" />
            </section>

            <section v-else-if="installState === 'cleaning'" class="space-y-2">
                <p>Finalizing installation...</p>
                <DProgress mode="immediate" class="h-2" />
            </section>

            <section v-else-if="installState === 'completed'" class="space-y-2">
                <p>FFmpeg has been successfully installed. Now you are ready to use the app completely. All you have to do is click on "Done".</p>
            </section>
        </div>

        <template v-slot:footer>
            <div class="w-full flex space-x-2 justify-end">
                <DButton v-if="installState === 'idle'" @click="() => invoke('install_ffmpeg')" class="font-semibold">Install</DButton>
                <DButton v-else-if="installState === 'completed'" @click="() => isOpen = false" class="font-semibold">Done</DButton>
                <DButton v-else disabled class="font-semibold">Installing...</DButton>
            </div>
        </template>
    </DModal>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';

const { progressPercent, installState = "downloading" } = defineProps<{
    progressPercent: number;
    installState: string;
}>();

const isOpen = defineModel<boolean>()

const goFFmpegWebsite = async () => {
    await openUrl('https://ffmpeg.org/');
}

</script>
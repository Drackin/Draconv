<template>
    <UiDialog v-model:open="isOpen" title="Install FFmpeg">

        <UiDialogContent>
            
            <UiDialogHeader>
                <UiDialogTitle>Install FFmpeg</UiDialogTitle>
                <UiDialogDescription>
                    FFmpeg is a tool that Draconv uses to convert videos. It is required for the app to function properly.
                </UiDialogDescription>
            </UiDialogHeader>

            <div class="flex flex-col space-y-4 relative">
                <section v-if="installState === 'idle'">
                    FFmpeg not found. The Draconv requires FFmpeg to function properly. Click "Install" to begin the installation process.
                </section>
                
                <section v-if="installState === 'downloading'" class="space-y-2">
                    <p>{{ progressPercent }}% installed</p>
                    <ProgressBar mode="percent" :percent="progressPercent" class="h-2" />
                </section>
                
                <section v-else-if="installState === 'installing'" class="space-y-2">
                    <p>Installing FFmpeg...</p>
                    <ProgressBar mode="immediate" class="h-2" />
                </section>
                
                <section v-else-if="installState === 'cleaning'" class="space-y-2">
                    <p>Finalizing installation...</p>
                    <ProgressBar mode="immediate" class="h-2" />
                </section>
                
                <section v-else-if="installState === 'completed'" class="space-y-2">
                    <p>FFmpeg has been successfully installed. Now you are ready to use the app completely. All you have to do is click on "Done".</p>
                </section>

                <p class="text-gray-300 p-2 bg-neutral-900 border border-neutral-800 rounded-lg text-xs">
                    FFmpeg is a free software licensed under the GNU General Public License (GPL) version 3.
                    The source code and license details are available at: <button class="underline cursor-pointer" @click="goFFmpegWebsite">https://ffmpeg.org/</button>
                </p>
            </div>

            <UiDialogFooter>
                <UiButton v-if="installState === 'idle'" @click="() => invoke('install_ffmpeg')">
                    <Icon name="lucide:download" />
                    Install
                </UiButton>
                <UiButton v-else-if="installState === 'completed'" @click="() => isOpen = false">
                    <Icon name="lucide:check" />
                    Done
                </UiButton>
                <UiButton v-else disabled>
                    <Icon name="lucide:loader-circle" class="animate-spin" />
                    Installing...
                </UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>
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
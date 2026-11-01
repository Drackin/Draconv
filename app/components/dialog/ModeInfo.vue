<template>
    <UiDialog v-model:open="isOpen">
        <UiDialogContent>
            <UiDialogHeader>
                <UiDialogTitle>Conversion Modes</UiDialogTitle>
                <UiDialogDescription>
                    Conversion modes to suit your needs. Each mode has its own advantages and is designed for different scenarios. Please read the descriptions below to understand the differences between them.
                </UiDialogDescription>
            </UiDialogHeader>

            <ul class="flex flex-col space-y-3">
                <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">Normal</span>: Standard
                    conversion mode balancing speed and quality.</li>
                    <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">Hardware Acceleration</span>:
                        Utilizes your GPU for faster conversions. Recommended for supported formats.</li>
                    <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">Lossless</span>: Prioritizes
                        quality over speed, ensuring no loss in the output file. Ideal for high-quality requirements. But
                        conversion durations may take longer depend on the file size.</li>
            </ul>
    
            <UiDialogFooter>
                <UiButton @click="isOpen = false" variant="default">Done</UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>
</template>

<script setup lang="ts">
const isOpen = defineModel<boolean>();
</script>
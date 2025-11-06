<template>
    <DModal v-model="isOpen" title="Conversion Mode" class="!z-[999]">
        <div class="flex flex-col space-y-3">
            <p>Draconv offers three conversion modes to suit your needs:</p>
            <ul class="flex flex-col space-y-3">
                <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">Normal</span>: Standard
                    conversion mode balancing speed and quality.</li>
                <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">Hardware Acceleration</span>:
                    Utilizes your GPU for faster conversions. Recommended for supported formats.</li>
                <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">Lossless</span>: Prioritizes
                    quality over speed, ensuring no loss in the output file. Ideal for high-quality requirements. But
                    conversion durations may take longer depend on the file size.</li>
            </ul>
            <p class="text-gray-400 p-2 bg-neutral-900 border border-neutral-800 rounded-lg">Please choose the mode that best fits your conversion needs.</p>
        </div>

        <template v-slot:footer>
            <DButton @click="isOpen = false" class="font-semibold self-end" variant="error">Close</DButton>
        </template>
    </DModal>
</template>

<script setup lang="ts">
const isOpen = defineModel<boolean>();
</script>
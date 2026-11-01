<template>
    <UiDialog v-model:open="isOpen">
        <UiDialogContent class="h-[80vh] w-[70vw]">
            <UiDialogHeader>
                <UiDialogTitle>Encoder Information</UiDialogTitle>
                <UiDialogDescription>
                    Encoders to suit your needs. Each encoder has its own advantages and is designed for different scenarios. Please read the descriptions below to understand the differences between them.
                </UiDialogDescription>
            </UiDialogHeader>
            <div class="flex min-h-0 flex-col space-y-3 overflow-x-hidden overflow-y-auto">
                <ul class="flex flex-col space-y-3">
                    <li>
                        <span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">H.264</span>:
                        This is the legacy industry standard that offers nearly universal compatibility across almost every device and
                        browser in existence, though it provides the least efficient compression by modern standards.
                    </li>
                    <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">H.265</span>: This successor
                        roughly doubles the compression efficiency of H.264, making it the primary choice for 4K and HDR
                        content with better compression without quality loss but its adoption is hindered by higher processing
                        requirements.</li>
                    <li><span class="p-1 bg-neutral-900 rounded-lg border border-neutral-800">AV1</span>: This is a
                        next-generation, royalty-free codec that delivers the best compression efficiency available today,
                        allowing for high-quality streaming at very low bitrates while being the most computationally
                        demanding to encode and decode. Recommended for streamers.</li>
                </ul>
                <div class="bg-yellow-700 border border-yellow-600 p-2 rounded-lg flex flex-col space-y-2">
                    <p>AV1 Hardware Acceleration is supported by limited GPUs:</p>

                    <section>
                        <p class="font-bold">NVIDIA</p>
                        <ul>
                            <li>GeForce RTX 40 / 50 Series</li>
                            <li>RTX 2000/3500/4000/5000/6000 Ada Series</li>
                        </ul>
                    </section>

                    <section>
                        <p class="font-bold">AMD</p>
                        <ul>
                            <li>Radeon RX 7000 / 9000 Series</li>
                            <li>Radeon 700M/800M iGPU (Integrated)</li>
                        </ul>
                    </section>

                    <section>
                        <p class="font-bold">Intel</p>
                        <ul>
                            <li>Intel Arc A & B Series</li>
                            <li>Integrated Graphics Intel Core Ultra (Meteor Lake, Lunar Lake and later)</li>
                        </ul>
                    </section>
                    <p>GeForce RTX 30 Series and Radeon 6000 Series only supported for decode. So, the app will use the standart CPU-based AV1 encoding.</p>
                </div>
            </div>

            <UiDialogFooter>
                <UiButton @click="isOpen = false" class="font-semibold">Close</UiButton>
            </UiDialogFooter>
        </UiDialogContent>
    </UiDialog>
</template>

<script setup lang="ts">
const isOpen = defineModel<boolean>();
</script>

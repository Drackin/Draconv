<template>
    <Transition name="fade">
        <div v-if="isOpen" class="fixed top-0 left-0 w-screen h-screen bg-[#111] z-40">
            <header class="flex items-center w-full">
                <button
                    class="flex justify-center items-center hover:bg-neutral-900 transition duration-200 w-14 h-14 border-b border-zinc-700"
                    @click="isOpen = false">
                    <Icon name="solar:arrow-left-outline" class="text-2xl" />
                </button>
                <DNavbar title="Supported Input and Output Formats" />
            </header>

            <section class="p-3">
                <table class="w-full  mt-5 border-collapse border border-zinc-700 rounded-xl overflow-hidden">
                    <thead>
                        <tr class="bg-zinc-800">
                            <th class="text-left p-3 font-semibold">Category</th>
                            <th class="text-left p-3 font-semibold">Input</th>
                            <th class="text-left p-3 font-semibold">Output</th>
                        </tr>
                    </thead>

                    <tbody>
                        <tr class="border-t border-zinc-700">
                            <td class="p-3 align-middle font-medium">Video</td>
                            <td class="p-3">
                                <div class="flex flex-wrap gap-2">
                                    <span v-for="input in supported_types.video.inputs" :key="input" class="p-1 border border-zinc-700 rounded-lg text-xs">
                                        {{ input }}
                                    </span>
                                </div>
                            </td>
                            <td class="p-3">
                                <div class="flex flex-wrap gap-2">
                                    <span v-for="output in supported_types.video.outputs" :key="output" class="p-1 border border-zinc-700 rounded-lg text-xs">
                                        {{ output }}
                                    </span>
                                </div>
                            </td>
                        </tr>
                        <tr class="border-t border-zinc-700">
                            <td class="p-3 align-middle font-medium">Audio</td>
                            <td class="p-3">
                                <div class="flex flex-wrap gap-2">
                                    <span v-for="input in supported_types.audio.inputs" :key="input" class="p-1 border border-zinc-700 rounded-lg text-xs">
                                        {{ input }}
                                    </span>
                                </div>
                            </td>
                            <td class="p-3">
                                <div class="flex flex-wrap gap-2">
                                    <span v-for="output in supported_types.audio.outputs" :key="output" class="p-1 border border-zinc-700 rounded-lg text-xs">
                                        {{ output }}
                                    </span>
                                </div>
                            </td>
                        </tr>
                        <tr class="border-t border-zinc-700">
                            <td class="p-3 align-middle font-medium">Image</td>
                            <td class="p-3">
                                <div class="flex flex-wrap gap-2">
                                    <span v-for="input in supported_types.image.inputs" :key="input" class="p-1 border border-zinc-700 rounded-lg text-xs">
                                        {{ input }}
                                    </span>
                                </div>
                            </td>
                            <td class="p-3">
                                <div class="flex flex-wrap gap-2">
                                    <span v-for="output in supported_types.image.outputs" :key="output" class="p-1 border border-zinc-700 rounded-lg text-xs">
                                        {{ output }}
                                    </span>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </section>

        </div>
    </Transition>
</template>

<script setup lang="ts">
import supported_types from "@/assets/supported.json"

const isOpen = defineModel<boolean>();
</script>
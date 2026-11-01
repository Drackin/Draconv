<template>
    <Transition name="formats-panel">
        <div
            v-if="isOpen"
            class="fixed inset-x-0 bottom-0 top-14 z-40 flex min-h-0 flex-col overflow-hidden bg-[#111] text-white"
            role="dialog"
            aria-modal="true"
            aria-labelledby="formats-title"
            @keydown.esc.window="isOpen = false"
        >
        <div class="ambient-lights fixed inset-0 -z-10 overflow-hidden" aria-hidden="true">
            <div class="ambient-light ambient-light-top"></div>
            <div class="ambient-light ambient-light-bottom"></div>
        </div>
            <div class="pointer-events-none absolute -left-28 -top-32 h-80 w-80 rounded-full bg-white/[0.07] blur-3xl" aria-hidden="true"></div>
            <div class="pointer-events-none absolute -bottom-40 -right-24 h-96 w-96 rounded-full bg-white/5 blur-3xl" aria-hidden="true"></div>

            <header class="relative shrink-0 border-b border-white/10 p-3 flex items-center space-x-4">
                <button
                    type="button"
                    aria-label="Back to settings"
                    class="group h-12 flex shrink-0 items-center gap-2 rounded-xl border border-white/10 bg-white/4 px-3 py-2 text-sm font-medium text-zinc-300 transition hover:border-white/20 hover:bg-white/9 hover:text-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/70"
                    @click="isOpen = false"
                >
                    <Icon name="lucide:arrow-left" class="text-base transition-transform group-hover:-translate-x-0.5" />
                    <span class="hidden sm:inline">Back</span>
                </button>

                <div class="min-w-0 flex-1">
                    <h1 id="formats-title" class="text-2xl font-semibold tracking-tight sm:text-3xl">Supported formats</h1>
                </div>

                <div class="flex gap-2 sm:grid-cols-3 sm:gap-3">
                    <div class="h-18  flex flex-col items-center justify-center rounded-xl border border-white/10 bg-white/4 px-3 py-2.5 sm:px-4">
                        <p class="text-lg font-semibold">{{ categoryOrder.length }}</p>
                        <p class="text-xs font-medium uppercase tracking-wider text-zinc-500">Categories</p>
                    </div>
                    <div class="size-18 flex flex-col items-center justify-center rounded-xl border border-white/10 bg-white/4 px-3 py-2.5 sm:px-4">
                        <p class="text-lg font-semibold">{{ totalInputFormats }}</p>
                        <p class="text-xs font-medium text-zinc-500">Input</p>
                    </div>
                    <div class="size-18 flex flex-col items-center justify-center rounded-xl border border-white/10 bg-white/4 px-3 py-2.5 sm:col-span-1 sm:px-4">
                        <p class="text-lg font-semibold">{{ totalOutputFormats }}</p>
                        <p class="text-xs font-medium text-zinc-500">Output</p>
                    </div>
                </div>
            </header>

            <section class="relative min-h-0 flex-1 overflow-y-auto" aria-label="Format reference">
                <div class="mx-auto w-full max-w-6xl px-4 py-5 sm:px-6 sm:py-6 lg:px-8 lg:py-8">
                    <div class="mb-5 flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                        <div class="relative w-full lg:max-w-sm">
                            <Icon name="lucide:search" class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-base text-zinc-500" />
                            <UiInput
                                v-model="query"
                                type="search"
                                placeholder="Search extensions"
                                aria-label="Search supported extensions"
                                class="h-10 w-full rounded-xl border border-white/10 bg-white/4 pl-9 pr-10 text-sm text-white outline-none transition placeholder:text-zinc-500 hover:border-white/20 focus:border-white/30 focus:ring-2 focus:ring-white/20"
                            />
                            <button
                                v-if="query"
                                type="button"
                                aria-label="Clear search"
                                class="absolute right-2 top-1/2 flex size-7 -translate-y-1/2 items-center justify-center rounded-lg text-zinc-500 transition hover:bg-white/10 hover:text-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60"
                                @click="query = ''"
                            >
                                <Icon name="lucide:x" class="text-sm" />
                            </button>
                        </div>

                        <div class="flex min-w-0 items-center gap-1.5 overflow-x-auto pb-1 lg:pb-0" aria-label="Filter by category">
                            <button
                                v-for="filter in filters"
                                :key="filter.key"
                                type="button"
                                :aria-pressed="activeCategory === filter.key"
                                class="flex shrink-0 items-center gap-1.5 rounded-lg border px-3 py-2 text-xs font-medium transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60"
                                :class="activeCategory === filter.key
                                    ? 'border-white/25 bg-white text-black'
                                    : 'border-white/10 bg-white/3 text-zinc-400 hover:border-white/20 hover:bg-white/8 hover:text-white'"
                                @click="activeCategory = filter.key"
                            >
                                <Icon v-if="filter.icon" :name="filter.icon" class="text-sm" />
                                {{ filter.label }}
                            </button>
                        </div>
                    </div>

                    <div class="mb-4 flex items-center justify-between gap-3 text-xs text-zinc-500">
                        <p>
                            Showing <span class="font-medium text-zinc-300">{{ displayedCategories.length }}</span>
                            {{ displayedCategories.length === 1 ? 'category' : 'categories' }}
                        </p>
                        <button
                            v-if="hasFilters"
                            type="button"
                            class="rounded-md px-2 py-1 text-zinc-400 transition hover:bg-white/6 hover:text-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60"
                            @click="resetFilters"
                        >
                            Clear filters
                        </button>
                    </div>

                    <div v-if="displayedCategories.length" class="grid gap-4 xl:grid-cols-2">
                        <article
                            v-for="category in displayedCategories"
                            :key="category.key"
                            class="overflow-hidden rounded-2xl border border-white/10 bg-zinc-950/65 shadow-xl shadow-black/10 backdrop-blur-xl"
                        >
                            <header class="flex items-center justify-between gap-3 border-b border-white/10 px-4 py-4 sm:px-5">
                                <div class="flex min-w-0 items-center gap-3">
                                    <div class="flex size-10 shrink-0 items-center justify-center rounded-xl border border-white/10 bg-white/[0.07]" :class="category.iconColor">
                                        <Icon :name="category.icon" class="text-xl" />
                                    </div>
                                    <div class="min-w-0">
                                        <h2 class="font-semibold tracking-tight">{{ category.label }}</h2>
                                        <p class="truncate text-xs text-zinc-500">{{ category.description }}</p>
                                    </div>
                                </div>
                                <span class="shrink-0 rounded-full border border-white/10 bg-white/4 px-2.5 py-1 text-[11px] font-medium text-zinc-400">
                                    {{ category.inputCount + category.outputCount }} formats
                                </span>
                            </header>

                            <div class="grid divide-y divide-white/10 sm:grid-cols-2 sm:divide-x sm:divide-y-0">
                                <section class="min-w-0 p-4 sm:p-5" :aria-labelledby="`input-formats-${category.key}`">
                                    <div class="mb-3 flex items-center justify-between gap-2">
                                        <div>
                                            <h3 :id="`input-formats-${category.key}`" class="text-sm font-medium text-zinc-200">Input</h3>
                                            <p class="mt-0.5 text-[11px] text-zinc-500">Formats you can add</p>
                                        </div>
                                        <span class="rounded-md bg-sky-400/10 px-2 py-1 text-[11px] font-semibold text-sky-300">{{ category.inputCount }}</span>
                                    </div>
                                    <div v-if="category.inputs.length" class="flex flex-wrap gap-1.5">
                                        <span
                                            v-for="extension in category.inputs"
                                            :key="`input-${category.key}-${extension}`"
                                            class="rounded-lg border border-sky-300/15 bg-sky-300/[0.07] px-2 py-1 text-xs font-medium text-sky-100"
                                        >
                                            .{{ extension }}
                                        </span>
                                    </div>
                                    <p v-else class="rounded-lg border border-dashed border-white/10 px-3 py-2 text-xs text-zinc-500">No matching input formats</p>
                                </section>

                                <section class="min-w-0 p-4 sm:p-5" :aria-labelledby="`output-formats-${category.key}`">
                                    <div class="mb-3 flex items-center justify-between gap-2">
                                        <div>
                                            <h3 :id="`output-formats-${category.key}`" class="text-sm font-medium text-zinc-200">Output</h3>
                                            <p class="mt-0.5 text-[11px] text-zinc-500">Formats you can convert to</p>
                                        </div>
                                        <span class="rounded-md bg-emerald-400/10 px-2 py-1 text-[11px] font-semibold text-emerald-300">{{ category.outputCount }}</span>
                                    </div>
                                    <div v-if="category.outputs.length" class="flex flex-wrap gap-1.5">
                                        <span
                                            v-for="extension in category.outputs"
                                            :key="`output-${category.key}-${extension}`"
                                            class="rounded-lg border border-emerald-300/15 bg-emerald-300/[0.07] px-2 py-1 text-xs font-medium text-emerald-100"
                                        >
                                            .{{ extension }}
                                        </span>
                                    </div>
                                    <p v-else class="rounded-lg border border-dashed border-white/10 px-3 py-2 text-xs text-zinc-500">No matching output formats</p>
                                </section>
                            </div>
                        </article>
                    </div>

                    <div v-else class="flex min-h-64 flex-col items-center justify-center rounded-2xl border border-dashed border-white/15 bg-white/[0.02] px-6 py-12 text-center">
                        <div class="mb-4 flex size-12 items-center justify-center rounded-2xl border border-white/10 bg-white/[0.05] text-zinc-400">
                            <Icon name="lucide:search-x" class="text-xl" />
                        </div>
                        <h2 class="font-semibold">No formats found</h2>
                        <p class="mt-1 max-w-sm text-sm text-zinc-500">Try a different extension or clear the current category filter.</p>
                        <button
                            type="button"
                            class="mt-4 rounded-lg border border-white/15 bg-white/[0.06] px-3 py-2 text-xs font-medium text-zinc-300 transition hover:bg-white/10 hover:text-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60"
                            @click="resetFilters"
                        >
                            Reset filters
                        </button>
                    </div>

                    <div class="mt-5 flex flex-wrap items-center gap-x-5 gap-y-2 text-[11px] text-zinc-500">
                        <span class="flex items-center gap-2"><i class="size-2 rounded-full bg-sky-300"></i>Input formats</span>
                        <span class="flex items-center gap-2"><i class="size-2 rounded-full bg-emerald-300"></i>Output formats</span>
                        <span class="ml-auto">Extensions are shown without file names.</span>
                    </div>
                </div>
            </section>
        </div>
    </Transition>
</template>

<script setup lang="ts">
import supportedTypes from "@/assets/supported.json"

const isOpen = defineModel<boolean>()

const categoryOrder = ["video", "audio", "image"] as const
type CategoryKey = typeof categoryOrder[number]
type FilterKey = "all" | CategoryKey

const categoryMeta: Record<CategoryKey, {
    label: string
    description: string
    icon: string
    iconColor: string
}> = {
    video: {
        label: "Video",
        description: "Movies, clips, and animated media",
        icon: "lucide:film",
        iconColor: "text-sky-300",
    },
    audio: {
        label: "Audio",
        description: "Music, podcasts, and soundtracks",
        icon: "lucide:music-2",
        iconColor: "text-violet-300",
    },
    image: {
        label: "Image",
        description: "Photos, graphics, and illustrations",
        icon: "lucide:image",
        iconColor: "text-amber-300",
    },
}

const query = ref("")
const activeCategory = ref<FilterKey>("all")

const filters = computed(() => [
    { key: "all" as const, label: "All formats", icon: "lucide:layers-3" },
    ...categoryOrder.map((key) => ({ key, label: categoryMeta[key].label, icon: categoryMeta[key].icon })),
])

const normalizedQuery = computed(() => query.value.trim().toLowerCase())

const displayedCategories = computed(() => {
    return categoryOrder
        .map((key) => {
            const category = supportedTypes[key]
            const inputs = category.inputs.filter((extension) => extension.toLowerCase().includes(normalizedQuery.value))
            const outputs = category.outputs.filter((extension) => extension.toLowerCase().includes(normalizedQuery.value))

            return {
                key,
                ...categoryMeta[key],
                inputs,
                outputs,
                inputCount: inputs.length,
                outputCount: outputs.length,
                matchesQuery: !normalizedQuery.value || inputs.length > 0 || outputs.length > 0,
            }
        })
        .filter((category) => {
            const matchesCategory = activeCategory.value === "all" || activeCategory.value === category.key
            return matchesCategory && category.matchesQuery
        })
})

const totalInputFormats = computed(() => categoryOrder.reduce((total, key) => total + supportedTypes[key].inputs.length, 0))
const totalOutputFormats = computed(() => categoryOrder.reduce((total, key) => total + supportedTypes[key].outputs.length, 0))
const hasFilters = computed(() => Boolean(normalizedQuery.value) || activeCategory.value !== "all")

const resetFilters = () => {
    query.value = ""
    activeCategory.value = "all"
}
</script>

<style scoped>
.formats-panel-enter-active,
.formats-panel-leave-active {
    transition: opacity 180ms ease, transform 220ms cubic-bezier(.22, 1, .36, 1);
}

.formats-panel-enter-from,
.formats-panel-leave-to {
    opacity: 0;
    transform: translateY(12px);
}
</style>

<template>
    <div class="relative isolate w-screen h-screen overflow-hidden bg-[#111]">
        <div class="ambient-lights fixed inset-0 -z-10 overflow-hidden" aria-hidden="true">
            <div class="ambient-light ambient-light-top"></div>
            <div class="ambient-light ambient-light-bottom"></div>
        </div>
        <Navbar />
        <main class="p-3 w-full overflow-auto" style="height: calc(100% - 3.5rem);">
            <NuxtPage />
        </main>
    </div>
</template>

<style>

.ambient-light {
    position: absolute;
    width: 30rem;
    height: 30rem;
    border-radius: 9999px;
    background: #fff;
    filter: blur(110px);
    opacity: 0.08;
    pointer-events: none;
}

.ambient-light-top {
    top: -15rem;
    left: 8%;
}

.ambient-light-bottom {
    right: -12rem;
    bottom: -18rem;
    opacity: 0.06;
}

@font-face {
    font-family: 'Space Grotesk';
    src: url("/SpaceGrotesk/space-grotesk-variable.ttf") format("truetype");
}
</style>

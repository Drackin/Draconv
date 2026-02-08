<template>
    <div class="w-screen h-screen">
        <div class="fixed w-full -bottom-5 left-0 z-[-1]">
            <img src="/bg.svg" class="w-full h-full" alt="">
        </div>
        <DNavbar />
        <main class="p-3 w-full overflow-auto" style="height: calc(100% - 3.5rem);">
            <NuxtPage />
        </main>
    </div>
</template>

<style>
:root, body {
    color: #fff;
    color-scheme: dark;
    font-family: "Space Grotesk", sans-serif;
    user-select: none;
}

body {
    background: #111;
}

@font-face {
    font-family: 'Space Grotesk';
    src: url("/SpaceGrotesk/space-grotesk-variable.ttf") format("truetype");
}
</style>
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const id = ref("");

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("greet", { id: id.value });
}
</script>

<template>
    <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="id" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
    </form>

    <p>{{ greetMsg }}</p>
</template>

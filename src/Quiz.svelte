<script lang="ts">
    import ChoicesQuiz from "./ChoicesQuiz.svelte";
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let name: string;

    let inquiz = false;
    let fetch_info = invoke("get_quizinfo", {
        name: name,
    });
    let fetch_lastplay = invoke("get_lastplay", {
        name: name,
    });

    let quiznum = 10;
</script>

{#if inquiz}
    <ChoicesQuiz
        num={quiznum}
        {name}
        on:quizend={() => {
            inquiz = false;
            fetch_info = invoke("get_quizinfo", {
                name: name,
            });
            fetch_lastplay = invoke("get_quizinfo", {
                name: name,
            });
        }}
    />
{:else}
    {#await fetch_info then info}
        {#await fetch_lastplay then lastplay}
            <h1>{info["title"]}</h1>
            <label>
                <p>問題数</p>
                <input type="number" bind:value={quiznum} min="1" max="50" />
                <input type="range" bind:value={quiznum} min="1" max="50" />
            </label>
            <p>
                最近のプレイ : {lastplay != "" ? lastplay : "未プレイ"}
            </p>
            <button
                in:fade
                on:click={() => {
                    inquiz = true;
                }}>始める</button
            >
            <button
                in:fade
                on:click={() => {
                    dispatch("back_quiz");
                }}>戻る</button
            >
        {/await}
    {/await}
{/if}

<style>
    h1 {
        font-weight: 300;
        font-size: 2em;
    }
    label {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }
    label * {
        margin: 5px;
    }
    input[type="range"] {
        background-color: #0044dd;
    }
    button {
        background-color: #fff;
        margin: 15px;
    }

    button:hover {
        background-color: #555;
        color: #fff;
    }

    button:active {
        background-color: #222;
    }
</style>

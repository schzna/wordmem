<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { slide, fade } from "svelte/transition";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let num;
    export let name;

    let quizes;
    let fetch_quizes = invoke("generate_quiz", {
        name: name,
        num: num,
    });

    function save() {
        invoke("save_play", {
            name: name,
            datetime: date.toLocaleString(),
            mode: 0,
            results: results,
        }).then(() => {
            dispatch("quizend");
        });
    }

    fetch_quizes.then((responce) => {
        quizes = responce;
    });

    const date = new Date();
    let results = [];
    $: correct_num = results.filter((e) => e[1]).length;
    $: solved_num = results.length;
    $: qi = solved_num;

    let toggles = [];
    let show_result = false;

    function judge(quizes, i) {
        toggles = [];
        toggles[i] = quizes[qi].correct == i;
        toggles[quizes[qi].correct] = true;
        setTimeout(next, 300, i, quizes);
    }

    function next(i, quizes) {
        if (toggles[i] != null) {
            toggles = [];
            results.push([qi, quizes[qi].correct == i]);
            results = results;
            if (qi >= num - 1) {
                setTimeout(() => {
                    show_result = true;
                }, 1000);
            } else {
                qi += 1;
            }
        }
    }
</script>

{#await fetch_quizes}
    <h1>Loading</h1>
{:then}
    <container>
        {#if solved_num < num}
            <p transition:fade={{ duration: 100 }}>
                問題数: {qi + 1}/{num} 正答率: {correct_num}/{solved_num}
            </p>
            <word transition:fade={{ duration: 100 }}>{quizes[qi].word}</word>
            <choices>
                {#each quizes[qi].choices as choice, i}
                    <choice
                        transition:slide
                        toggle={toggles[i] ?? "none"}
                        on:click={() => {
                            judge(quizes, i);
                        }}>{choice}</choice
                    >
                {/each}
            </choices>
        {:else if show_result}
            <h1 transition:fade>結果</h1>
            <p>正答率{correct_num}/{solved_num}</p>
            <list in:slide out:fade>
                {#each results as res, i}
                    <result res={res[1]}
                        >{quizes[i].word}:{res[1] ? "正解" : "不正解"}</result
                    >
                {/each}
            </list>
            <button
                transition:fade
                on:click={() => {
                    show_result = false;
                }}
                on:outroend={save}>戻る</button
            >
        {/if}
    </container>
{/await}

<style>
    :root {
        --correct-color: #00a36a;
        --wrong-color: #dd3300;
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

    h1 {
        font-weight: 300;
        margin: 5px;
    }

    p {
        margin: 5px;
    }

    result {
        font-size: 1.5em;
        border: 1px solid;
        margin: 3px;
        border-radius: 3px;
        width: 80%;
        color: white;
    }

    result[res="true"] {
        background-color: var(--correct-color);
    }

    result[res="false"] {
        background-color: var(--wrong-color);
    }

    list {
        scrollbar-width: thin;
        display: flex;
        width: 100%;
        flex-direction: column;
        align-items: center;
        overflow-y: scroll;
    }

    container {
        height: 100%;
        font-family: "Noto Sans";
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-around;
        font-size: x-large;
        width: 80%;
    }

    container choices {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        margin: 10px;
        transition-duration: 0.2s;
    }

    choice {
        cursor: default;
        width: 60%;
        padding: 5px;
        padding-left: 10px;
        padding-right: 10px;
        text-align: center;
        border: solid 2px;
        margin: 5px;

        background-color: #fff;
        color: #222;

        transition-duration: 50ms;

        border-radius: 3px;

        border-color: #444;
    }

    choice:hover {
        background-color: #444;
        color: #fff;
    }

    choice[toggle="true"] {
        color: #fff;
        background-color: var(--correct-color);
        border-color: var(--correct-color);
    }

    choice[toggle="true"]:hover {
        color: #fff;
        background-color: var(--correct-color);
        border-color: var(--correct-color);
    }

    choice[toggle="false"] {
        color: #fff;
        background-color: var(--wrong-color);
        border-color: var(--wrong-color);
    }

    choice[toggle="false"]:hover {
        color: #fff;
        background-color: var(--wrong-color);
        border-color: var(--wrong-color);
    }

    word {
        font-size: 3em;
        font-weight: 300;
    }
</style>

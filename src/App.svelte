<script>
    import Quiz from "./Quiz.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    let fetch_quizes = invoke("list_quizes");
    let selected = null;
</script>

<main>
    {#if selected != null}
        <Quiz
            name={selected}
            on:back_quiz={() => {
                selected = null;
            }}
        />
    {:else}
        {#await fetch_quizes}
            <h1>Loading...</h1>
        {:then quizes}
            <h1>クイズ一覧</h1>
            <quizlist>
                {#each quizes as quiz}
                    <quizname
                        on:click={() => {
                            selected = quiz;
                        }}
                        >{quiz}
                        <hr />
                        ここに説明文。
                    </quizname>
                {/each}
            </quizlist>
        {/await}
    {/if}
</main>

<style>
    quizlist {
        display: flex;
        flex-wrap: wrap;
        row-gap: 0;
        width: 90%;
        height: 80%;
        margin: 10px;
        align-content: flex-start;
        transition-duration: 0.2s;
        overflow-y: scroll;
    }

    quizname {
        font-size: 1.5em;
        font-weight: 300;
        cursor: default;
        width: 45%;
        height: 100px;
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

    quizname:hover {
        background-color: #444;
        color: #fff;
    }
    h1 {
        font-weight: 300;
        font-size: 2em;
    }
    main {
        font-family: "Noto Sans";
        display: flex;
        flex-direction: column;
        justify-content: center;
        height: 90%;
        text-align: center;
        align-items: center;
        padding: 1em;
        max-width: 240px;
        margin: 0;
        padding: 0;
    }
    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>

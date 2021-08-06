<script lang="ts">
    import { scanCode } from "./uwu.js"

    /**
     * List of verified inbuilt JavaScript functions.
     * TODO: expand this list...
     */
    const GLOBALS = ["Math", "parseInt", "parseFloat"]

    /**
     * Smartweave contract API.
     */
    const SmartWeaveGLOBALS = [...GLOBALS, "Smartweave"]

    // initial source example
    let source = `window["local" + "Storage"].getItem("apiKey");`

    // refresh reactively (re-scan) when changes occur
    $: diagnostics = scanCode(source, SmartWeaveGLOBALS)
</script>

<main>
    <h2>Below is demo component usage:</h2>
    {#if source}
        source:
        <div class="source">
            <textarea bind:value={source} rows="10" cols="100" />
        </div>
    {/if}

    {#if diagnostics}
        <div class="container">
            <div class="checker">DAPP Checker<span class="flag">🏁</span></div>
            <div class="diagnostics">
                {#await diagnostics then diagnostics}
                    <textarea readonly rows="12" cols="100"
                        >{diagnostics}</textarea
                    >
                {/await}
            </div>
        </div>
    {/if}
</main>

<style>
    textarea {
        font-family: monospace;
    }

    .container {
        text-align: center;
    }
    .diagnostics textarea {
        color: #ff3e00;
    }

    .flag {
        font-size: 1.25em;
    }
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    .checker {
        color: #444444;
        font-size: 2em;
        font-weight: 800;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>

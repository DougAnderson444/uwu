<script lang="ts">
    import { scanCode } from "./uwu.js"

    // initial source example
    export let source

    let diagnostics

    /**
     * List of verified inbuilt JavaScript functions.
     * TODO: expand this list...
     */
    const GLOBALS = ["Math", "parseInt", "parseFloat"]

    /**
     * Smartweave contract API.
     */
    const SmartWeaveGLOBALS = [...GLOBALS, "Smartweave"]

    // refresh reactively (re-scan) when changes occur
    $: if (source) diagnostics = scanCode(source, SmartWeaveGLOBALS)
</script>

{#if diagnostics}
    {#await diagnostics then diagnostics}
        <div class="container">
            <div class="checker">DAPP Checker!<span class="flag">🏁</span></div>
            {#if diagnostics === "true"}
                ✔️ Code is clean
            {:else}
                <div class="diagnostics">
                    <textarea readonly rows="12" cols="100"
                        >{diagnostics}</textarea
                    >
                </div>
            {/if}
        </div>
    {/await}
{/if}

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

    .checker {
        color: #444444;
        font-size: 2em;
        font-weight: 800;
    }
</style>

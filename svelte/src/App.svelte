<script lang="ts">
    import UWU from "./UWU.svelte"

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
    let source = `// comment window["local" + "Storage"].getItem("apiKey");`
</script>

<main>
    <h2>Below is demo component usage:</h2>
    {#if source}
        source:
        <div class="source">
            <textarea bind:value={source} rows="10" cols="100" />
        </div>
    {/if}

    <UWU {source} />
</main>

<style>
    textarea {
        font-family: monospace;
    }

    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>

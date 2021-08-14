    import uwuwasm from "../../crates/uwu-wasm/Cargo.toml"

    let uwu

    export const scanCode = async (sourceCode, globals) => {

        // only loads the first time
        if(!uwu) {
            uwu = await uwuwasm()
        }

        const { default: init, scan } = uwu

        // Scan, collect diagnostics.
        let diagnostics = scan(sourceCode, globals)
        
        // If no diagnostics, evalutate code...
        if (diagnostics === "true") {
            // good code
            console.log('good code',diagnostics)
        }else{
            console.error('bad code',diagnostics)
        }

        return diagnostics

    }

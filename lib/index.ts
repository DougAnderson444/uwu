import Arweave from "arweave"
import Transaction from "arweave/node/lib/transaction"

// web browser version
import initWasm, { scan as webScan } from "../crates/uwu-wasm/pkg/uwu_wasm"
import { scan as nodeScan } from "../crates/uwu-wasm/pkg-node/uwu_wasm"

/**
 * List of verified inbuilt JavaScript functions.
 * TODO: expand this list...
 */
export const GLOBALS  = ["Math", "parseInt", "parseFloat"]

/**
 * Smartweave contract API.
 */
export const SmartWeaveGLOBALS = [...GLOBALS, "Smartweave"]

/**
 * Scope is a pre-defined set of items exposed to the script's scope.
 * eg: ['String', 'Math', ...]
 */
export type Scope = Array<string>

// https://github.com/ArweaveTeam/SmartWeave/blob/78dd343228511161ae820cf6bd6661bf7fa6b6b3/src/utils.ts#L9
export function getTag(tx: Transaction, name: string) {
    const tags = tx.get("tags") as any

    for (const tag of tags) {
        // decoding tags can throw on invalid utf8 data.
        try {
            if (tag.get("name", { decode: true, string: true }) === name) {
                return tag.get("value", { decode: true, string: true })
            }
        } catch (e) {}
    }

    return false
}

export function scan(source: string, scope?: Scope) {

    scope = scope || [] 
    // TODO: Pass in scope to the scanner.
    return (typeof window == "undefined") ? nodeScan(source) : webScan(source)
}

export async function scanTx(arweave: Arweave, txID: string, scope?: Scope) {
    scope = scope || ["ContractAssert", "ContractThrow"]
    let contractTx = await arweave.transactions.get(txID)
    let contractSrc = getTag(contractTx, "Contract-Src")
    const contractSrcTX = await arweave.transactions.get(contractSrc)
    const source = contractSrcTX.get("data", { decode: true, string: true })
    return scan(source as string)
}

export { initWasm as init } // re-export the wasm init function

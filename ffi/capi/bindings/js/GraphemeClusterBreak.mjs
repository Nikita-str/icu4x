// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** See the [Rust documentation for `GraphemeClusterBreak`](https://docs.rs/icu/latest/icu/properties/props/struct.GraphemeClusterBreak.html) for more information.
*/
export class GraphemeClusterBreak {
    #value = undefined;

    static #values = new Map([
        ["Other", 0],
        ["Control", 1],
        ["Cr", 2],
        ["Extend", 3],
        ["L", 4],
        ["Lf", 5],
        ["Lv", 6],
        ["Lvt", 7],
        ["T", 8],
        ["V", 9],
        ["SpacingMark", 10],
        ["Prepend", 11],
        ["RegionalIndicator", 12],
        ["EBase", 13],
        ["EBaseGaz", 14],
        ["EModifier", 15],
        ["GlueAfterZwj", 16],
        ["Zwj", 17]
    ]);

    static getAllEntries() {
        return GraphemeClusterBreak.#values.entries();
    }

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return GraphemeClusterBreak.#objectValues[arguments[1]];
        }

        if (value instanceof GraphemeClusterBreak) {
            return value;
        }

        let intVal = GraphemeClusterBreak.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return GraphemeClusterBreak.#objectValues[intVal];
        }

        throw TypeError(value + " is not a GraphemeClusterBreak and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...GraphemeClusterBreak.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 4),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 5),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 6),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 7),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 8),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 9),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 10),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 11),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 12),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 13),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 14),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 15),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 16),
        new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 17),
    ];

    static Other = GraphemeClusterBreak.#objectValues[0];
    static Control = GraphemeClusterBreak.#objectValues[1];
    static Cr = GraphemeClusterBreak.#objectValues[2];
    static Extend = GraphemeClusterBreak.#objectValues[3];
    static L = GraphemeClusterBreak.#objectValues[4];
    static Lf = GraphemeClusterBreak.#objectValues[5];
    static Lv = GraphemeClusterBreak.#objectValues[6];
    static Lvt = GraphemeClusterBreak.#objectValues[7];
    static T = GraphemeClusterBreak.#objectValues[8];
    static V = GraphemeClusterBreak.#objectValues[9];
    static SpacingMark = GraphemeClusterBreak.#objectValues[10];
    static Prepend = GraphemeClusterBreak.#objectValues[11];
    static RegionalIndicator = GraphemeClusterBreak.#objectValues[12];
    static EBase = GraphemeClusterBreak.#objectValues[13];
    static EBaseGaz = GraphemeClusterBreak.#objectValues[14];
    static EModifier = GraphemeClusterBreak.#objectValues[15];
    static GlueAfterZwj = GraphemeClusterBreak.#objectValues[16];
    static Zwj = GraphemeClusterBreak.#objectValues[17];

    toInteger() {
        const result = wasm.icu4x_GraphemeClusterBreak_to_integer_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    static fromInteger(other) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_GraphemeClusterBreak_from_integer_mv1(diplomatReceive.buffer, other);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return new GraphemeClusterBreak(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
        }
        
        finally {
            diplomatReceive.free();
        }
    }
}
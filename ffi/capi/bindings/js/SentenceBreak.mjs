// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** See the [Rust documentation for `SentenceBreak`](https://docs.rs/icu/latest/icu/properties/props/struct.SentenceBreak.html) for more information.
*/
export class SentenceBreak {
    #value = undefined;

    static #values = new Map([
        ["Other", 0],
        ["ATerm", 1],
        ["Close", 2],
        ["Format", 3],
        ["Lower", 4],
        ["Numeric", 5],
        ["OLetter", 6],
        ["Sep", 7],
        ["Sp", 8],
        ["STerm", 9],
        ["Upper", 10],
        ["Cr", 11],
        ["Extend", 12],
        ["Lf", 13],
        ["SContinue", 14]
    ]);

    static getAllEntries() {
        return SentenceBreak.#values.entries();
    }

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return SentenceBreak.#objectValues[arguments[1]];
        }

        if (value instanceof SentenceBreak) {
            return value;
        }

        let intVal = SentenceBreak.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return SentenceBreak.#objectValues[intVal];
        }

        throw TypeError(value + " is not a SentenceBreak and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...SentenceBreak.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 4),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 5),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 6),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 7),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 8),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 9),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 10),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 11),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 12),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 13),
        new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 14),
    ];

    static Other = SentenceBreak.#objectValues[0];
    static ATerm = SentenceBreak.#objectValues[1];
    static Close = SentenceBreak.#objectValues[2];
    static Format = SentenceBreak.#objectValues[3];
    static Lower = SentenceBreak.#objectValues[4];
    static Numeric = SentenceBreak.#objectValues[5];
    static OLetter = SentenceBreak.#objectValues[6];
    static Sep = SentenceBreak.#objectValues[7];
    static Sp = SentenceBreak.#objectValues[8];
    static STerm = SentenceBreak.#objectValues[9];
    static Upper = SentenceBreak.#objectValues[10];
    static Cr = SentenceBreak.#objectValues[11];
    static Extend = SentenceBreak.#objectValues[12];
    static Lf = SentenceBreak.#objectValues[13];
    static SContinue = SentenceBreak.#objectValues[14];

    toInteger() {
        const result = wasm.icu4x_SentenceBreak_to_integer_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    static fromInteger(other) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_SentenceBreak_from_integer_mv1(diplomatReceive.buffer, other);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return new SentenceBreak(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
        }
        
        finally {
            diplomatReceive.free();
        }
    }
}
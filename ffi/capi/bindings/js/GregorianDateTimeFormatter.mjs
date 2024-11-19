// generated by diplomat-tool
import { DataProvider } from "./DataProvider.mjs"
import { DateTimeFormatterLoadError } from "./DateTimeFormatterLoadError.mjs"
import { DateTimeLength } from "./DateTimeLength.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { Locale } from "./Locale.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X FixedCalendarDateTimeFormatter object capable of formatting a [`IsoDateTime`] as a string,
*using the Gregorian Calendar.
*
*See the [Rust documentation for `datetime`](https://docs.rs/icu/latest/icu/datetime/index.html) for more information.
*/
const GregorianDateTimeFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_GregorianDateTimeFormatter_destroy_mv1(ptr);
});

export class GregorianDateTimeFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("GregorianDateTimeFormatter is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            GregorianDateTimeFormatter_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static createWithLength(provider, locale, length) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_GregorianDateTimeFormatter_create_with_length_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue, length.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DateTimeFormatterLoadError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DateTimeFormatterLoadError: ' + cause.value, { cause });
            }
            return new GregorianDateTimeFormatter(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    formatIsoDatetime(value) {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.icu4x_GregorianDateTimeFormatter_format_iso_datetime_mv1(this.ffiValue, value.ffiValue, write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            write.free();
        }
    }
}
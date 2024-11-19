// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { DateTime } from "./DateTime"
import type { DateTimeFormatError } from "./DateTimeFormatError"
import type { DateTimeFormatterLoadError } from "./DateTimeFormatterLoadError"
import type { DateTimeLength } from "./DateTimeLength"
import type { IsoDateTime } from "./IsoDateTime"
import type { Locale } from "./Locale"
import type { TimeZoneInfo } from "./TimeZoneInfo"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An object capable of formatting a date time with time zone to a string.
*
*See the [Rust documentation for `datetime`](https://docs.rs/icu/latest/icu/datetime/index.html) for more information.
*/
export class ZonedDateTimeFormatter {
    

    get ffiValue(): pointer;

    static createWithLength(provider: DataProvider, locale: Locale, length: DateTimeLength): ZonedDateTimeFormatter;

    formatDatetimeWithCustomTimeZone(datetime: DateTime, timeZone: TimeZoneInfo): string;

    formatIsoDatetimeWithCustomTimeZone(datetime: IsoDateTime, timeZone: TimeZoneInfo): string;
}
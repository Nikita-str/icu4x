// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_list_unit_v1 {
    ($ req : expr) => {
        ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"].binary_search_by(|k| $req.locale.strict_cmp(k.as_bytes()).reverse()).ok().map(|i| unsafe {
            static RU: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }]);
            static JA: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("", 0u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("", 0u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("", 0u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("", 0u8), special_case: None }]);
            static TH: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" และ ", 11u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" และ ", 11u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" และ ", 11u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }]);
            static FR: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" et ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" et ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" et ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" et ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }]);
            static SR_LATN: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" i ", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" i ", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" i ", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" i ", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" i ", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" i ", 3u8), special_case: None }]);
            static ES: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" y ", 3u8), special_case: Some(icu_list::provider::SpecialCasePattern { condition: unsafe { icu_list::provider::SerdeDFA::from_dfa_bytes_unchecked(if cfg!(target_endian = "little") { b"rust-regex-automata-dfa-sparse\0\0\xFF\xFE\0\0\x02\0\0\0\0\0\0\0\x0E\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x02\x02\x02\x03\x04\x04\x05\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\t\t\t\n\x0B\x0B\x0C\r\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x12\x12\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x14\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x16\x17\x17\x18\x19\x19\x19\x1A\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B(\x01\0\0\x01\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x01\x80\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x05\0\x05\x05\x06\x06\x0C\x0C\r\r\0\0S\0\0\0D\0\0\0S\0\0\0D\0\0\0\0\0\0\0\0\x02\0\0\x1B\0\0\x12\0\0\0\x12\0\0\0\0\x03\0\x06\x06\r\r\0\0h\0\0\0h\0\0\0\0\0\0\0\0\x0E\0\0\0\x02\x02\x04\x07\t\t\x0B\x0E\x13\x13\x14\x14\x15\x15\x16\x16\x17\x17\x18\x18\x19\x19\x1A\x1A\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0\xBF\0\0\0\xCE\0\0\0\xDD\0\0\0\xEC\0\0\0\xDD\0\0\0\xFB\0\0\0\n\x01\0\0\x19\x01\0\0\x12\0\0\0\0\x02\0\x0F\x11\0\0D\0\0\0\0\0\0\0\0\x02\0\x11\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x10\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x10\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x0F\0\0\xDD\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0\t\0\0\0\x12\0\0\0\x12\0\0\0\0\0\0\0\0\0\0\0#\0\0\0#\0\0\0" } else { b"rust-regex-automata-dfa-sparse\0\0\0\0\xFE\xFF\0\0\0\x02\0\0\0\0\0\0\0\x0E\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x02\x02\x02\x03\x04\x04\x05\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\t\t\t\n\x0B\x0B\x0C\r\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x12\x12\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x14\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x16\x17\x17\x18\x19\x19\x19\x1A\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\0\0\x01(\x01\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x01\x80\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x05\0\x05\x05\x06\x06\x0C\x0C\r\r\0\0S\0\0\0D\0\0\0S\0\0\0D\0\0\0\0\0\0\0\0\x02\0\0\x1B\0\0\x12\0\0\0\x12\0\0\0\0\x03\0\x06\x06\r\r\0\0h\0\0\0h\0\0\0\0\0\0\0\0\x0E\0\0\0\x02\x02\x04\x07\t\t\x0B\x0E\x13\x13\x14\x14\x15\x15\x16\x16\x17\x17\x18\x18\x19\x19\x1A\x1A\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0\xBF\0\0\0\xCE\0\0\0\xDD\0\0\0\xEC\0\0\0\xDD\0\0\0\xFB\0\0\0\n\x01\0\0\x19\x01\0\0\x12\0\0\0\0\x02\0\x0F\x11\0\0D\0\0\0\0\0\0\0\0\x02\0\x11\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x10\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x10\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x0F\0\0\xDD\0\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0\0\0\0#\0\0\0\t\0\0\0\x12\0\0\0\x12\0\0\0\0\0\0\0\0\0\0\0#\0\0\0#" }) }, pattern: icu_list::provider::ListJoinerPattern::from_parts(" e ", 3u8) }) }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" y ", 3u8), special_case: Some(icu_list::provider::SpecialCasePattern { condition: unsafe { icu_list::provider::SerdeDFA::from_dfa_bytes_unchecked(if cfg!(target_endian = "little") { b"rust-regex-automata-dfa-sparse\0\0\xFF\xFE\0\0\x02\0\0\0\0\0\0\0\x0E\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x02\x02\x02\x03\x04\x04\x05\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\t\t\t\n\x0B\x0B\x0C\r\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x12\x12\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x14\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x16\x17\x17\x18\x19\x19\x19\x1A\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B(\x01\0\0\x01\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x01\x80\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x05\0\x05\x05\x06\x06\x0C\x0C\r\r\0\0S\0\0\0D\0\0\0S\0\0\0D\0\0\0\0\0\0\0\0\x02\0\0\x1B\0\0\x12\0\0\0\x12\0\0\0\0\x03\0\x06\x06\r\r\0\0h\0\0\0h\0\0\0\0\0\0\0\0\x0E\0\0\0\x02\x02\x04\x07\t\t\x0B\x0E\x13\x13\x14\x14\x15\x15\x16\x16\x17\x17\x18\x18\x19\x19\x1A\x1A\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0\xBF\0\0\0\xCE\0\0\0\xDD\0\0\0\xEC\0\0\0\xDD\0\0\0\xFB\0\0\0\n\x01\0\0\x19\x01\0\0\x12\0\0\0\0\x02\0\x0F\x11\0\0D\0\0\0\0\0\0\0\0\x02\0\x11\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x10\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x10\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x0F\0\0\xDD\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0\t\0\0\0\x12\0\0\0\x12\0\0\0\0\0\0\0\0\0\0\0#\0\0\0#\0\0\0" } else { b"rust-regex-automata-dfa-sparse\0\0\0\0\xFE\xFF\0\0\0\x02\0\0\0\0\0\0\0\x0E\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x02\x02\x02\x03\x04\x04\x05\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\t\t\t\n\x0B\x0B\x0C\r\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x12\x12\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x14\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x16\x17\x17\x18\x19\x19\x19\x1A\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\0\0\x01(\x01\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x01\x80\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x05\0\x05\x05\x06\x06\x0C\x0C\r\r\0\0S\0\0\0D\0\0\0S\0\0\0D\0\0\0\0\0\0\0\0\x02\0\0\x1B\0\0\x12\0\0\0\x12\0\0\0\0\x03\0\x06\x06\r\r\0\0h\0\0\0h\0\0\0\0\0\0\0\0\x0E\0\0\0\x02\x02\x04\x07\t\t\x0B\x0E\x13\x13\x14\x14\x15\x15\x16\x16\x17\x17\x18\x18\x19\x19\x1A\x1A\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0\xBF\0\0\0\xCE\0\0\0\xDD\0\0\0\xEC\0\0\0\xDD\0\0\0\xFB\0\0\0\n\x01\0\0\x19\x01\0\0\x12\0\0\0\0\x02\0\x0F\x11\0\0D\0\0\0\0\0\0\0\0\x02\0\x11\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x10\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x10\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x0F\0\0\xDD\0\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0\0\0\0#\0\0\0\t\0\0\0\x12\0\0\0\x12\0\0\0\0\0\0\0\0\0\0\0#\0\0\0#" }) }, pattern: icu_list::provider::ListJoinerPattern::from_parts(" e ", 3u8) }) }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" y ", 3u8), special_case: Some(icu_list::provider::SpecialCasePattern { condition: unsafe { icu_list::provider::SerdeDFA::from_dfa_bytes_unchecked(if cfg!(target_endian = "little") { b"rust-regex-automata-dfa-sparse\0\0\xFF\xFE\0\0\x02\0\0\0\0\0\0\0\x0E\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x02\x02\x02\x03\x04\x04\x05\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\t\t\t\n\x0B\x0B\x0C\r\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x12\x12\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x14\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x16\x17\x17\x18\x19\x19\x19\x1A\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B(\x01\0\0\x01\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x01\x80\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x05\0\x05\x05\x06\x06\x0C\x0C\r\r\0\0S\0\0\0D\0\0\0S\0\0\0D\0\0\0\0\0\0\0\0\x02\0\0\x1B\0\0\x12\0\0\0\x12\0\0\0\0\x03\0\x06\x06\r\r\0\0h\0\0\0h\0\0\0\0\0\0\0\0\x0E\0\0\0\x02\x02\x04\x07\t\t\x0B\x0E\x13\x13\x14\x14\x15\x15\x16\x16\x17\x17\x18\x18\x19\x19\x1A\x1A\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0\xBF\0\0\0\xCE\0\0\0\xDD\0\0\0\xEC\0\0\0\xDD\0\0\0\xFB\0\0\0\n\x01\0\0\x19\x01\0\0\x12\0\0\0\0\x02\0\x0F\x11\0\0D\0\0\0\0\0\0\0\0\x02\0\x11\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x10\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x10\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x0F\0\0\xDD\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0\t\0\0\0\x12\0\0\0\x12\0\0\0\0\0\0\0\0\0\0\0#\0\0\0#\0\0\0" } else { b"rust-regex-automata-dfa-sparse\0\0\0\0\xFE\xFF\0\0\0\x02\0\0\0\0\0\0\0\x0E\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x02\x02\x02\x03\x04\x04\x05\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\t\t\t\n\x0B\x0B\x0C\r\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x12\x12\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x14\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x15\x16\x17\x17\x18\x19\x19\x19\x1A\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\x1B\0\0\x01(\x01\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x01\x80\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x05\0\x05\x05\x06\x06\x0C\x0C\r\r\0\0S\0\0\0D\0\0\0S\0\0\0D\0\0\0\0\0\0\0\0\x02\0\0\x1B\0\0\x12\0\0\0\x12\0\0\0\0\x03\0\x06\x06\r\r\0\0h\0\0\0h\0\0\0\0\0\0\0\0\x0E\0\0\0\x02\x02\x04\x07\t\t\x0B\x0E\x13\x13\x14\x14\x15\x15\x16\x16\x17\x17\x18\x18\x19\x19\x1A\x1A\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0D\0\0\0\xBF\0\0\0\xCE\0\0\0\xDD\0\0\0\xEC\0\0\0\xDD\0\0\0\xFB\0\0\0\n\x01\0\0\x19\x01\0\0\x12\0\0\0\0\x02\0\x0F\x11\0\0D\0\0\0\0\0\0\0\0\x02\0\x11\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x0F\x10\0\0\xBF\0\0\0\0\0\0\0\0\x02\0\x10\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x11\0\0\xDD\0\0\0\0\0\0\0\0\x02\0\x0F\x0F\0\0\xDD\0\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0#\0\0\0#\0\0\0#\0\0\0#\0\0\0\0\0\0#\0\0\0\t\0\0\0\x12\0\0\0\x12\0\0\0\0\0\0\0\0\0\0\0#\0\0\0#" }) }, pattern: icu_list::provider::ListJoinerPattern::from_parts(" e ", 3u8) }) }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }]);
            static SR: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" и ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" и ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" и ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" и ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" и ", 4u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" и ", 4u8), special_case: None }]);
            static EN: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" ", 1u8), special_case: None }]);
            static BN: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(", ", 2u8), special_case: None }]);
            static AR: <icu_list::provider::UnitListV1Marker as icu_provider::DataMarker>::Yokeable = icu_list::provider::ListFormatterPatternsV1([icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("، و", 5u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("، و", 5u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("، و", 5u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" و", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("، و", 5u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("، و", 5u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts("، و", 5u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" و", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" و", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" و", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" و", 3u8), special_case: None }, icu_list::provider::ConditionalListJoinerPattern { default: icu_list::provider::ListJoinerPattern::from_parts(" و", 3u8), special_case: None }]);
            *[&AR, &AR, &BN, &BN, &EN, &EN, &EN, &ES, &ES, &EN, &FR, &JA, &RU, &SR, &SR, &SR_LATN, &TH, &RU, &BN].get_unchecked(i)
        })
    };
}
/// Implement [`DataProvider<UnitListV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_list_unit_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_list::provider::UnitListV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_list::provider::UnitListV1Marker>, icu_provider::DataError> {
                lookup_list_unit_v1!(req).map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_list::provider::UnitListV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}

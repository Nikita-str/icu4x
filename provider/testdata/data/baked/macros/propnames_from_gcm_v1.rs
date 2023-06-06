// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_from_gcm_v1 {
    () => {
        icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
            map: unsafe {
                #[allow(unused_unsafe)]
                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"P\0\0\0\0\0\x01\0\r\0\x0F\0\x11\0\"\0$\0)\0+\09\0N\0U\0W\0f\0v\0\x84\0\x89\0\x97\0\xA8\0\xAE\0\xC1\0\xC2\0\xC4\0\xCA\0\xD7\0\xE5\0\xE7\0\xE9\0\xEB\0\xFB\0\xFD\0\xFF\0\0\x01\x04\x01\x0F\x01\x11\x01\x13\x01\x15\x01$\x013\x014\x016\x018\x01:\x01I\x01O\x01_\x01d\x01p\x01|\x01\x8D\x01\x99\x01\x9A\x01\xAD\x01\xAF\x01\xB1\x01\xB3\x01\xB5\x01\xB7\x01\xB9\x01\xC4\x01\xC6\x01\xCB\x01\xD6\x01\xD7\x01\xD9\x01\xE2\x01\xE4\x01\xE6\x01\xE8\x01\xF7\x01\x03\x02\x0C\x02\x12\x02\"\x02,\x02<\x02=\x02?\x02A\x02CCased_LetterCcCfClose_PunctuationCncntrlCoCombining_MarkConnector_PunctuationControlCsCurrency_SymbolDash_PunctuationDecimal_NumberdigitEnclosing_MarkFinal_PunctuationFormatInitial_PunctuationLLCLetterLetter_NumberLine_SeparatorLlLmLoLowercase_LetterLtLuMMarkMath_SymbolMcMeMnModifier_LetterModifier_SymbolNNdNlNoNonspacing_MarkNumberOpen_PunctuationOtherOther_LetterOther_NumberOther_PunctuationOther_SymbolPParagraph_SeparatorPcPdPePfPiPoPrivate_UsePspunctPunctuationSScSeparatorSkSmSoSpace_SeparatorSpacing_MarkSurrogateSymbolTitlecase_LetterUnassignedUppercase_LetterZZlZpZs") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xFA\xFF\xFF\xFF\x0F\0\x10\0\x15\0\0\0\x0F\0\x11\0\xFD\xFF\x16\0\x0F\0\x12\0\x19\0\x13\0\t\0\t\0\x07\0\x1D\0\x10\0\x1C\0\xFE\xFF\xFF\xFF\xFE\xFF\n\0\r\0\x02\0\x04\0\x05\0\x02\0\x03\0\x01\0\xFD\xFF\xFD\xFF\x18\0\x08\0\x07\0\x06\0\x04\0\x1A\0\xFC\xFF\t\0\n\0\x0B\0\x06\0\xFC\xFF\x14\0\xFA\xFF\x05\0\x0B\0\x17\0\x1B\0\xF9\xFF\x0E\0\x16\0\x13\0\x15\0\x1D\0\x1C\0\x17\0\x11\0\x14\0\xF9\xFF\xF9\xFF\xF8\xFF\x19\0\xFB\xFF\x1A\0\x18\0\x1B\0\x0C\0\x08\0\x12\0\xF8\xFF\x03\0\0\0\x01\0\xFB\xFF\r\0\x0E\0\x0C\0") })
            },
        }
    };
}
/// Implement [`DataProvider<GeneralCategoryMaskNameToValueV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_gcm_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::names::GeneralCategoryMaskNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::names::GeneralCategoryMaskNameToValueV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::names::GeneralCategoryMaskNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_from_gcm_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::names::GeneralCategoryMaskNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}

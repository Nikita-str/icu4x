// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_di_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xAD\0\0\0\xAE\0\0\0O\x03\0\0P\x03\0\0\x1C\x06\0\0\x1D\x06\0\0_\x11\0\0a\x11\0\0\xB4\x17\0\0\xB6\x17\0\0\x0B\x18\0\0\x10\x18\0\0\x0B \0\0\x10 \0\0* \0\0/ \0\0` \0\0p \0\0d1\0\0e1\0\0\0\xFE\0\0\x10\xFE\0\0\xFF\xFE\0\0\0\xFF\0\0\xA0\xFF\0\0\xA1\xFF\0\0\xF0\xFF\0\0\xF9\xFF\0\0\xA0\xBC\x01\0\xA4\xBC\x01\0s\xD1\x01\0{\xD1\x01\0\0\0\x0E\0\0\x10\x0E\0") }, 4174usize)
        })
    };
}
/// Implement [`DataProvider<DefaultIgnorableCodePointV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_di_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::DefaultIgnorableCodePointV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::DefaultIgnorableCodePointV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::DefaultIgnorableCodePointV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_di_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::DefaultIgnorableCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}

// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::EastAsianWidthV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct = &::icu_properties::provider::PropertyCodePointMapV1::CodePointTrie(
    ::icu_collections::codepointtrie::CodePointTrie::from_parts(
        ::icu_collections::codepointtrie::CodePointTrieHeader {
            high_start: 1114112u32,
            shifted12_high_start: 272u16,
            index3_null_offset: 18u16,
            data_null_offset: 359u32,
            null_value: 0u32,
            trie_type: ::icu_collections::codepointtrie::TrieType::Small,
        },
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                0u8, 0u8, 64u8, 0u8, 127u8, 0u8, 191u8, 0u8, 252u8, 0u8, 59u8, 1u8, 103u8, 1u8,
                153u8, 1u8, 103u8, 1u8, 200u8, 1u8, 103u8, 1u8, 4u8, 2u8, 68u8, 2u8, 84u8, 2u8,
                132u8, 2u8, 194u8, 2u8, 1u8, 3u8, 49u8, 3u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 152u8, 4u8, 178u8, 4u8, 192u8, 4u8, 214u8, 4u8, 246u8, 4u8, 255u8, 4u8,
                28u8, 5u8, 54u8, 5u8, 86u8, 5u8, 86u8, 5u8, 86u8, 5u8, 87u8, 5u8, 86u8, 5u8, 86u8,
                5u8, 86u8, 5u8, 87u8, 5u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8,
                4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8,
                4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8,
                4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8,
                4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8,
                4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 246u8,
                4u8, 119u8, 5u8, 246u8, 4u8, 246u8, 4u8, 246u8, 4u8, 151u8, 5u8, 151u8, 5u8, 151u8,
                5u8, 152u8, 5u8, 151u8, 5u8, 151u8, 5u8, 151u8, 5u8, 152u8, 5u8, 0u8, 0u8, 16u8,
                0u8, 32u8, 0u8, 48u8, 0u8, 64u8, 0u8, 80u8, 0u8, 96u8, 0u8, 112u8, 0u8, 127u8, 0u8,
                143u8, 0u8, 159u8, 0u8, 175u8, 0u8, 191u8, 0u8, 207u8, 0u8, 223u8, 0u8, 239u8, 0u8,
                252u8, 0u8, 12u8, 1u8, 28u8, 1u8, 44u8, 1u8, 59u8, 1u8, 75u8, 1u8, 91u8, 1u8,
                107u8, 1u8, 103u8, 1u8, 119u8, 1u8, 135u8, 1u8, 151u8, 1u8, 153u8, 1u8, 169u8, 1u8,
                185u8, 1u8, 201u8, 1u8, 103u8, 1u8, 119u8, 1u8, 135u8, 1u8, 151u8, 1u8, 200u8, 1u8,
                216u8, 1u8, 232u8, 1u8, 248u8, 1u8, 103u8, 1u8, 119u8, 1u8, 135u8, 1u8, 151u8, 1u8,
                4u8, 2u8, 20u8, 2u8, 36u8, 2u8, 52u8, 2u8, 68u8, 2u8, 84u8, 2u8, 100u8, 2u8, 116u8,
                2u8, 84u8, 2u8, 100u8, 2u8, 116u8, 2u8, 132u8, 2u8, 132u8, 2u8, 148u8, 2u8, 164u8,
                2u8, 180u8, 2u8, 194u8, 2u8, 210u8, 2u8, 226u8, 2u8, 242u8, 2u8, 1u8, 3u8, 17u8,
                3u8, 33u8, 3u8, 49u8, 3u8, 49u8, 3u8, 65u8, 3u8, 81u8, 3u8, 97u8, 3u8, 103u8, 1u8,
                119u8, 1u8, 135u8, 1u8, 151u8, 1u8, 103u8, 1u8, 119u8, 1u8, 135u8, 1u8, 151u8, 1u8,
                103u8, 1u8, 119u8, 1u8, 135u8, 1u8, 151u8, 1u8, 103u8, 1u8, 119u8, 1u8, 135u8, 1u8,
                151u8, 1u8, 103u8, 1u8, 119u8, 1u8, 135u8, 1u8, 151u8, 1u8, 103u8, 1u8, 119u8, 1u8,
                135u8, 1u8, 151u8, 1u8, 103u8, 1u8, 119u8, 1u8, 135u8, 1u8, 151u8, 1u8, 103u8, 1u8,
                119u8, 1u8, 135u8, 1u8, 151u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                129u8, 3u8, 145u8, 3u8, 161u8, 3u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 19u8, 1u8,
                176u8, 3u8, 103u8, 1u8, 183u8, 3u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 196u8, 3u8, 209u8, 3u8, 224u8, 3u8, 103u8, 1u8, 103u8, 1u8, 237u8, 3u8,
                120u8, 2u8, 122u8, 2u8, 198u8, 0u8, 122u8, 2u8, 103u8, 1u8, 252u8, 3u8, 103u8, 1u8,
                10u8, 4u8, 16u8, 1u8, 103u8, 1u8, 26u8, 4u8, 36u8, 4u8, 51u8, 4u8, 66u8, 4u8, 80u8,
                4u8, 100u8, 1u8, 96u8, 4u8, 103u8, 1u8, 104u8, 4u8, 115u8, 4u8, 18u8, 1u8, 145u8,
                0u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 122u8, 4u8, 134u8,
                4u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8,
                1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 145u8, 4u8, 161u8, 4u8, 103u8,
                1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 68u8, 2u8, 68u8,
                2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 156u8, 2u8,
                68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 120u8, 2u8, 68u8, 2u8, 68u8,
                2u8, 128u8, 2u8, 68u8, 2u8, 175u8, 4u8, 164u8, 2u8, 189u8, 4u8, 201u8, 4u8, 77u8,
                1u8, 213u8, 4u8, 229u8, 4u8, 244u8, 4u8, 4u8, 5u8, 103u8, 1u8, 103u8, 1u8, 16u8,
                5u8, 28u8, 5u8, 44u8, 5u8, 98u8, 3u8, 103u8, 1u8, 60u8, 5u8, 76u8, 5u8, 88u8, 5u8,
                97u8, 5u8, 107u8, 5u8, 121u8, 5u8, 135u8, 5u8, 151u8, 5u8, 103u8, 1u8, 163u8, 5u8,
                204u8, 1u8, 172u8, 5u8, 187u8, 5u8, 103u8, 1u8, 62u8, 2u8, 103u8, 1u8, 198u8, 5u8,
                103u8, 1u8, 205u8, 5u8, 103u8, 1u8, 103u8, 1u8, 221u8, 5u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 235u8, 5u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 231u8, 4u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 251u8, 5u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 113u8, 3u8,
                11u8, 6u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 28u8, 5u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 26u8, 5u8,
                103u8, 1u8, 22u8, 6u8, 38u8, 6u8, 113u8, 3u8, 113u8, 3u8, 39u8, 6u8, 112u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 47u8, 6u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 108u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                112u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 39u8, 6u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 28u8, 5u8, 113u8, 3u8, 39u8, 6u8,
                113u8, 3u8, 113u8, 3u8, 56u8, 6u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 72u8, 6u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 25u8, 5u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 113u8, 3u8, 72u8, 6u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 28u8, 5u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8,
                68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8,
                2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8,
                68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8,
                2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 68u8, 2u8, 75u8, 6u8, 103u8, 1u8, 113u8, 3u8,
                113u8, 3u8, 18u8, 6u8, 91u8, 6u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 106u8, 6u8, 107u8, 6u8,
                107u8, 6u8, 107u8, 6u8, 107u8, 6u8, 107u8, 6u8, 122u8, 6u8, 123u8, 6u8, 123u8, 6u8,
                123u8, 6u8, 123u8, 6u8, 124u8, 6u8, 139u8, 6u8, 147u8, 6u8, 163u8, 6u8, 204u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 27u8, 5u8, 204u8, 5u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                24u8, 5u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                26u8, 5u8, 103u8, 1u8, 103u8, 1u8, 179u8, 6u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                195u8, 6u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 203u8, 5u8, 103u8, 1u8,
                103u8, 1u8, 203u8, 5u8, 210u8, 6u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 22u8, 6u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                167u8, 5u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 98u8, 3u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 121u8, 2u8, 68u8, 2u8, 118u8, 2u8, 68u8, 2u8, 68u8, 2u8,
                68u8, 2u8, 122u8, 2u8, 68u8, 2u8, 226u8, 6u8, 241u8, 6u8, 119u8, 2u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 203u8, 5u8, 113u8, 3u8, 113u8, 3u8,
                22u8, 6u8, 179u8, 6u8, 204u8, 5u8, 26u8, 5u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 113u8, 3u8,
                113u8, 3u8, 1u8, 7u8, 15u8, 6u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 14u8, 7u8,
                113u8, 3u8, 28u8, 5u8, 113u8, 3u8, 113u8, 3u8, 28u8, 7u8, 28u8, 5u8, 113u8, 3u8,
                43u8, 7u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 39u8, 6u8, 58u8, 7u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 41u8, 6u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 40u8, 6u8,
                74u8, 7u8, 113u8, 3u8, 24u8, 5u8, 210u8, 5u8, 103u8, 1u8, 127u8, 4u8, 167u8, 5u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 102u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 90u8, 7u8, 106u8, 7u8, 231u8, 4u8, 115u8, 7u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                22u8, 6u8, 127u8, 7u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 101u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                16u8, 7u8, 15u8, 6u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8,
                113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 103u8, 1u8,
                103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 143u8, 7u8,
                25u8, 5u8, 113u8, 3u8, 72u8, 6u8, 74u8, 6u8, 26u8, 5u8, 75u8, 6u8, 24u8, 5u8, 25u8,
                5u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8,
                1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8,
                1u8, 103u8, 1u8, 103u8, 1u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8,
                3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8,
                3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8,
                3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8,
                3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 113u8, 3u8, 40u8, 6u8, 103u8,
                1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8,
                1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8, 1u8, 103u8,
                1u8, 103u8, 1u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8,
                68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8,
                2u8, 68u8, 2u8, 103u8, 1u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8,
                68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8,
                2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8,
                68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 68u8,
                2u8, 68u8, 2u8, 68u8, 2u8, 68u8, 2u8, 118u8, 2u8, 132u8, 0u8, 164u8, 0u8, 196u8,
                0u8, 204u8, 0u8, 204u8, 0u8, 204u8, 0u8, 204u8, 0u8, 204u8, 0u8, 236u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 11u8, 1u8,
                43u8, 1u8, 75u8, 1u8, 107u8, 1u8, 138u8, 1u8, 163u8, 1u8, 18u8, 0u8, 187u8, 1u8,
                219u8, 1u8, 250u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 3u8, 2u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 35u8, 2u8,
                18u8, 0u8, 48u8, 2u8, 18u8, 0u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                80u8, 2u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 112u8, 2u8, 112u8, 2u8,
                112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8,
                112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 128u8, 2u8, 144u8, 2u8, 18u8, 0u8, 176u8, 2u8,
                18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8,
                18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8,
                18u8, 0u8, 18u8, 0u8, 208u8, 2u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 240u8, 2u8, 255u8, 1u8, 255u8, 1u8,
                16u8, 3u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8,
                18u8, 0u8, 18u8, 0u8, 33u8, 3u8, 65u8, 3u8, 88u8, 3u8, 18u8, 0u8, 18u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8,
                18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 120u8, 3u8,
                152u8, 3u8, 184u8, 3u8, 216u8, 3u8, 248u8, 3u8, 24u8, 4u8, 18u8, 0u8, 18u8, 0u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8,
                255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 255u8, 1u8, 56u8, 4u8, 88u8, 4u8, 18u8, 0u8,
                18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8,
                18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8,
                0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8, 18u8, 0u8,
                112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8,
                112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8,
                112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8,
                112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8,
                112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 112u8, 2u8, 120u8, 4u8,
            ])
        },
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8,
                4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8,
                4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8,
                4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8,
                4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8,
                4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                1u8, 4u8, 4u8, 1u8, 4u8, 4u8, 1u8, 1u8, 0u8, 1u8, 0u8, 4u8, 1u8, 1u8, 4u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8,
                1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 0u8, 0u8, 1u8,
                0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 1u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8,
                0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 0u8, 1u8,
                0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8,
                0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 1u8, 0u8, 1u8, 1u8, 1u8,
                0u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8,
                0u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 1u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 0u8, 0u8, 1u8, 1u8, 0u8,
                0u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 0u8, 1u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 1u8,
                0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                2u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 1u8, 1u8, 0u8, 0u8,
                0u8, 1u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
                0u8, 1u8, 1u8, 1u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
                1u8, 1u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 1u8, 1u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8,
                5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8,
                0u8, 5u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                1u8, 0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                1u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8,
                0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 0u8, 1u8,
                1u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8, 5u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 0u8, 5u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 5u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 1u8, 5u8, 1u8, 1u8, 1u8, 1u8, 1u8, 5u8, 5u8, 1u8, 5u8, 1u8, 1u8, 1u8,
                1u8, 5u8, 1u8, 1u8, 5u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
                0u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 5u8, 0u8, 0u8, 0u8, 5u8, 5u8,
                5u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                4u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8,
                5u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 3u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 5u8, 0u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 3u8, 3u8, 3u8, 3u8, 3u8,
                3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 2u8, 2u8, 2u8, 2u8, 2u8,
                2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 0u8, 0u8, 2u8, 2u8, 2u8,
                2u8, 2u8, 2u8, 0u8, 0u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 0u8, 0u8, 2u8, 2u8, 2u8,
                0u8, 0u8, 0u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 0u8, 2u8, 2u8, 2u8, 2u8, 2u8,
                2u8, 2u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8,
                5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                5u8, 1u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 1u8, 1u8, 1u8, 1u8,
                1u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8,
                5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 5u8,
                0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 5u8, 5u8, 5u8, 5u8,
                5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 0u8, 0u8, 5u8,
                5u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8,
                5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 5u8, 5u8, 5u8, 5u8, 5u8, 0u8, 0u8, 0u8, 0u8,
            ])
        },
        ::icu_properties::EastAsianWidth(0u8),
    ),
);

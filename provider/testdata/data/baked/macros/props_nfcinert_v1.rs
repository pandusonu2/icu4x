// @generated
/// Implement [`DataProvider<NfcInertV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_nfcinert_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_NFCINERT_V1: &'static <icu_properties::provider::NfcInertV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\0\0<\0\0\0?\0\0\0A\0\0\0Q\0\0\0R\0\0\0[\0\0\0a\0\0\0q\0\0\0r\0\0\0{\0\0\0\xA8\0\0\0\xA9\0\0\0\xC0\0\0\0\xD0\0\0\0\xD1\0\0\0\xD7\0\0\0\xD8\0\0\0\xDE\0\0\0\xE0\0\0\0\xF0\0\0\0\xF1\0\0\0\xF7\0\0\0\xF8\0\0\0\xFE\0\0\0\xFF\0\0\0\x04\x01\0\0\x06\x01\0\0\x10\x01\0\0\x12\x01\0\0\x18\x01\0\0\x1A\x01\0\0\"\x01\0\0$\x01\0\0&\x01\0\0(\x01\0\0.\x01\0\x000\x01\0\x001\x01\0\09\x01\0\0;\x01\0\0=\x01\0\0?\x01\0\0C\x01\0\0E\x01\0\0G\x01\0\0I\x01\0\0L\x01\0\0R\x01\0\0T\x01\0\0V\x01\0\0X\x01\0\0^\x01\0\0`\x01\0\0b\x01\0\0d\x01\0\0f\x01\0\0h\x01\0\0r\x01\0\0t\x01\0\0\x80\x01\0\0\xA0\x01\0\0\xA2\x01\0\0\xAF\x01\0\0\xB1\x01\0\0\xB7\x01\0\0\xB8\x01\0\0\xCD\x01\0\0\xDD\x01\0\0\xDE\x01\0\0\xE2\x01\0\0\xE6\x01\0\0\xEC\x01\0\0\xF4\x01\0\0\xF6\x01\0\0\xF8\x01\0\0\xFC\x01\0\0\0\x02\0\0\x1C\x02\0\0\x1E\x02\0\0 \x02\0\0&\x02\0\x004\x02\0\0\x92\x02\0\0\x93\x02\0\0\0\x03\0\0O\x03\0\0P\x03\0\0p\x03\0\0t\x03\0\0u\x03\0\0~\x03\0\0\x7F\x03\0\0\x87\x03\0\0\x88\x03\0\0\x91\x03\0\0\x92\x03\0\0\x95\x03\0\0\x96\x03\0\0\x97\x03\0\0\x98\x03\0\0\x99\x03\0\0\x9A\x03\0\0\x9F\x03\0\0\xA0\x03\0\0\xA1\x03\0\0\xA2\x03\0\0\xA5\x03\0\0\xA6\x03\0\0\xA9\x03\0\0\xAA\x03\0\0\xAC\x03\0\0\xAD\x03\0\0\xAE\x03\0\0\xAF\x03\0\0\xB1\x03\0\0\xB2\x03\0\0\xB5\x03\0\0\xB6\x03\0\0\xB7\x03\0\0\xB8\x03\0\0\xB9\x03\0\0\xBA\x03\0\0\xBF\x03\0\0\xC0\x03\0\0\xC1\x03\0\0\xC2\x03\0\0\xC5\x03\0\0\xC6\x03\0\0\xC9\x03\0\0\xCC\x03\0\0\xCE\x03\0\0\xCF\x03\0\0\xD2\x03\0\0\xD3\x03\0\0\x06\x04\0\0\x07\x04\0\0\x10\x04\0\0\x11\x04\0\0\x13\x04\0\0\x14\x04\0\0\x15\x04\0\0\x19\x04\0\0\x1A\x04\0\0\x1B\x04\0\0\x1E\x04\0\0\x1F\x04\0\0#\x04\0\0$\x04\0\0'\x04\0\0(\x04\0\0+\x04\0\0,\x04\0\0-\x04\0\0.\x04\0\x000\x04\0\x001\x04\0\x003\x04\0\x004\x04\0\x005\x04\0\09\x04\0\0:\x04\0\0;\x04\0\0>\x04\0\0?\x04\0\0C\x04\0\0D\x04\0\0G\x04\0\0H\x04\0\0K\x04\0\0L\x04\0\0M\x04\0\0N\x04\0\0V\x04\0\0W\x04\0\0t\x04\0\0v\x04\0\0\x83\x04\0\0\x88\x04\0\0\xD8\x04\0\0\xDA\x04\0\0\xE8\x04\0\0\xEA\x04\0\0\x91\x05\0\0\xBE\x05\0\0\xBF\x05\0\0\xC0\x05\0\0\xC1\x05\0\0\xC3\x05\0\0\xC4\x05\0\0\xC6\x05\0\0\xC7\x05\0\0\xC8\x05\0\0\x10\x06\0\0\x1B\x06\0\0\"\x06\0\0$\x06\0\0'\x06\0\0(\x06\0\0H\x06\0\0I\x06\0\0J\x06\0\0`\x06\0\0p\x06\0\0q\x06\0\0\xC1\x06\0\0\xC2\x06\0\0\xD2\x06\0\0\xD3\x06\0\0\xD5\x06\0\0\xDD\x06\0\0\xDF\x06\0\0\xE5\x06\0\0\xE7\x06\0\0\xE9\x06\0\0\xEA\x06\0\0\xEE\x06\0\0\x11\x07\0\0\x12\x07\0\x000\x07\0\0K\x07\0\0\xEB\x07\0\0\xF4\x07\0\0\xFD\x07\0\0\xFE\x07\0\0\x16\x08\0\0\x1A\x08\0\0\x1B\x08\0\0$\x08\0\0%\x08\0\0(\x08\0\0)\x08\0\0.\x08\0\0Y\x08\0\0\\\x08\0\0\x98\x08\0\0\xA0\x08\0\0\xCA\x08\0\0\xE2\x08\0\0\xE3\x08\0\0\0\t\0\0(\t\0\0)\t\0\x000\t\0\x001\t\0\x003\t\0\x004\t\0\0<\t\0\0=\t\0\0M\t\0\0N\t\0\0Q\t\0\0U\t\0\0X\t\0\0`\t\0\0\xBC\t\0\0\xBD\t\0\0\xBE\t\0\0\xBF\t\0\0\xC7\t\0\0\xC8\t\0\0\xCD\t\0\0\xCE\t\0\0\xD7\t\0\0\xD8\t\0\0\xDC\t\0\0\xDE\t\0\0\xDF\t\0\0\xE0\t\0\0\xFE\t\0\0\xFF\t\0\x003\n\0\x004\n\0\x006\n\0\x007\n\0\0<\n\0\0=\n\0\0M\n\0\0N\n\0\0Y\n\0\0\\\n\0\0^\n\0\0_\n\0\0\xBC\n\0\0\xBD\n\0\0\xCD\n\0\0\xCE\n\0\0<\x0B\0\0=\x0B\0\0>\x0B\0\0?\x0B\0\0G\x0B\0\0H\x0B\0\0M\x0B\0\0N\x0B\0\0V\x0B\0\0X\x0B\0\0\\\x0B\0\0^\x0B\0\0\x92\x0B\0\0\x93\x0B\0\0\xBE\x0B\0\0\xBF\x0B\0\0\xC6\x0B\0\0\xC8\x0B\0\0\xCD\x0B\0\0\xCE\x0B\0\0\xD7\x0B\0\0\xD8\x0B\0\0<\x0C\0\0=\x0C\0\0F\x0C\0\0G\x0C\0\0M\x0C\0\0N\x0C\0\0U\x0C\0\0W\x0C\0\0\xBC\x0C\0\0\xBD\x0C\0\0\xBF\x0C\0\0\xC0\x0C\0\0\xC2\x0C\0\0\xC3\x0C\0\0\xC6\x0C\0\0\xC7\x0C\0\0\xCA\x0C\0\0\xCB\x0C\0\0\xCD\x0C\0\0\xCE\x0C\0\0\xD5\x0C\0\0\xD7\x0C\0\0;\r\0\0=\r\0\0>\r\0\0?\r\0\0F\r\0\0H\r\0\0M\r\0\0N\r\0\0W\r\0\0X\r\0\0\xCA\r\0\0\xCB\r\0\0\xCF\r\0\0\xD0\r\0\0\xD9\r\0\0\xDA\r\0\0\xDC\r\0\0\xDD\r\0\0\xDF\r\0\0\xE0\r\0\08\x0E\0\0;\x0E\0\0H\x0E\0\0L\x0E\0\0\xB8\x0E\0\0\xBB\x0E\0\0\xC8\x0E\0\0\xCC\x0E\0\0\x18\x0F\0\0\x1A\x0F\0\x005\x0F\0\x006\x0F\0\x007\x0F\0\08\x0F\0\09\x0F\0\0:\x0F\0\0C\x0F\0\0D\x0F\0\0M\x0F\0\0N\x0F\0\0R\x0F\0\0S\x0F\0\0W\x0F\0\0X\x0F\0\0\\\x0F\0\0]\x0F\0\0i\x0F\0\0j\x0F\0\0q\x0F\0\0w\x0F\0\0x\x0F\0\0y\x0F\0\0z\x0F\0\0~\x0F\0\0\x80\x0F\0\0\x85\x0F\0\0\x86\x0F\0\0\x88\x0F\0\0\x93\x0F\0\0\x94\x0F\0\0\x9D\x0F\0\0\x9E\x0F\0\0\xA2\x0F\0\0\xA3\x0F\0\0\xA7\x0F\0\0\xA8\x0F\0\0\xAC\x0F\0\0\xAD\x0F\0\0\xB9\x0F\0\0\xBA\x0F\0\0\xC6\x0F\0\0\xC7\x0F\0\0%\x10\0\0&\x10\0\0.\x10\0\0/\x10\0\x007\x10\0\08\x10\0\09\x10\0\0;\x10\0\0\x8D\x10\0\0\x8E\x10\0\0\0\x11\0\0\x13\x11\0\0a\x11\0\0v\x11\0\0\xA8\x11\0\0\xC3\x11\0\0]\x13\0\0`\x13\0\0\x14\x17\0\0\x16\x17\0\x004\x17\0\x005\x17\0\0\xD2\x17\0\0\xD3\x17\0\0\xDD\x17\0\0\xDE\x17\0\0\xA9\x18\0\0\xAA\x18\0\09\x19\0\0<\x19\0\0\x17\x1A\0\0\x19\x1A\0\0`\x1A\0\0a\x1A\0\0u\x1A\0\0}\x1A\0\0\x7F\x1A\0\0\x80\x1A\0\0\xB0\x1A\0\0\xBE\x1A\0\0\xBF\x1A\0\0\xCF\x1A\0\0\x05\x1B\0\0\x06\x1B\0\0\x07\x1B\0\0\x08\x1B\0\0\t\x1B\0\0\n\x1B\0\0\x0B\x1B\0\0\x0C\x1B\0\0\r\x1B\0\0\x0E\x1B\0\0\x11\x1B\0\0\x12\x1B\0\x004\x1B\0\x006\x1B\0\0:\x1B\0\0;\x1B\0\0<\x1B\0\0=\x1B\0\0>\x1B\0\0@\x1B\0\0B\x1B\0\0C\x1B\0\0D\x1B\0\0E\x1B\0\0k\x1B\0\0t\x1B\0\0\xAA\x1B\0\0\xAC\x1B\0\0\xE6\x1B\0\0\xE7\x1B\0\0\xF2\x1B\0\0\xF4\x1B\0\x007\x1C\0\08\x1C\0\0\xD0\x1C\0\0\xD3\x1C\0\0\xD4\x1C\0\0\xE1\x1C\0\0\xE2\x1C\0\0\xE9\x1C\0\0\xED\x1C\0\0\xEE\x1C\0\0\xF4\x1C\0\0\xF5\x1C\0\0\xF8\x1C\0\0\xFA\x1C\0\0\xC0\x1D\0\0\x04\x1E\0\0\n\x1E\0\0\x10\x1E\0\0\x12\x1E\0\0\x1C\x1E\0\0 \x1E\0\0(\x1E\0\0*\x1E\0\0B\x1E\0\0D\x1E\0\0T\x1E\0\0X\x1E\0\0~\x1E\0\0\x80\x1E\0\0\x88\x1E\0\0\x8E\x1E\0\0\x92\x1E\0\0\x96\x1E\0\0\x9A\x1E\0\0\xA0\x1E\0\0\xF4\x1E\0\0\xF6\x1E\0\0\xFA\x1E\0\0\0\x1F\0\0\x12\x1F\0\0\x18\x1F\0\0\x1A\x1F\0\0 \x1F\0\x002\x1F\0\08\x1F\0\0:\x1F\0\0@\x1F\0\0B\x1F\0\0H\x1F\0\0J\x1F\0\0P\x1F\0\0R\x1F\0\0Y\x1F\0\0Z\x1F\0\0`\x1F\0\0r\x1F\0\0s\x1F\0\0v\x1F\0\0w\x1F\0\0x\x1F\0\0y\x1F\0\0z\x1F\0\0{\x1F\0\0~\x1F\0\0\x80\x1F\0\0\x82\x1F\0\0\x88\x1F\0\0\x8A\x1F\0\0\x90\x1F\0\0\x92\x1F\0\0\x98\x1F\0\0\x9A\x1F\0\0\xA0\x1F\0\0\xA2\x1F\0\0\xA8\x1F\0\0\xAA\x1F\0\0\xB3\x1F\0\0\xB4\x1F\0\0\xB6\x1F\0\0\xB7\x1F\0\0\xBB\x1F\0\0\xBD\x1F\0\0\xBE\x1F\0\0\xC0\x1F\0\0\xC3\x1F\0\0\xC4\x1F\0\0\xC6\x1F\0\0\xC7\x1F\0\0\xC9\x1F\0\0\xCA\x1F\0\0\xCB\x1F\0\0\xCD\x1F\0\0\xD3\x1F\0\0\xD4\x1F\0\0\xDB\x1F\0\0\xDC\x1F\0\0\xE3\x1F\0\0\xE4\x1F\0\0\xEB\x1F\0\0\xEC\x1F\0\0\xEE\x1F\0\0\xF0\x1F\0\0\xF3\x1F\0\0\xF4\x1F\0\0\xF6\x1F\0\0\xF7\x1F\0\0\xF9\x1F\0\0\xFA\x1F\0\0\xFB\x1F\0\0\xFF\x1F\0\0\0 \0\0\x02 \0\0\xD0 \0\0\xDD \0\0\xE1 \0\0\xE2 \0\0\xE5 \0\0\xF1 \0\0&!\0\0'!\0\0*!\0\0,!\0\0\x90!\0\0\x91!\0\0\x92!\0\0\x93!\0\0\x94!\0\0\x95!\0\0\xD0!\0\0\xD1!\0\0\xD2!\0\0\xD3!\0\0\xD4!\0\0\xD5!\0\0\x03\"\0\0\x04\"\0\0\x08\"\0\0\t\"\0\0\x0B\"\0\0\x0C\"\0\0#\"\0\0$\"\0\0%\"\0\0&\"\0\0<\"\0\0=\"\0\0C\"\0\0D\"\0\0E\"\0\0F\"\0\0H\"\0\0I\"\0\0M\"\0\0N\"\0\0a\"\0\0b\"\0\0d\"\0\0f\"\0\0r\"\0\0t\"\0\0v\"\0\0x\"\0\0z\"\0\0~\"\0\0\x82\"\0\0\x84\"\0\0\x86\"\0\0\x88\"\0\0\x91\"\0\0\x93\"\0\0\xA2\"\0\0\xA3\"\0\0\xA8\"\0\0\xAA\"\0\0\xAB\"\0\0\xAC\"\0\0\xB2\"\0\0\xB6\"\0\0)#\0\0+#\0\0\xDC*\0\0\xDD*\0\0\xEF,\0\0\xF2,\0\0\x7F-\0\0\x80-\0\0\xE0-\0\0\0.\0\0*0\0\x0000\0\0F0\0\0G0\0\0K0\0\0L0\0\0M0\0\0N0\0\0O0\0\0P0\0\0Q0\0\0R0\0\0S0\0\0T0\0\0U0\0\0V0\0\0W0\0\0X0\0\0Y0\0\0Z0\0\0[0\0\0\\0\0\0]0\0\0^0\0\0_0\0\0`0\0\0a0\0\0b0\0\0d0\0\0e0\0\0f0\0\0g0\0\0h0\0\0i0\0\0o0\0\0p0\0\0r0\0\0s0\0\0u0\0\0v0\0\0x0\0\0y0\0\0{0\0\0|0\0\0\x990\0\0\x9B0\0\0\x9D0\0\0\x9E0\0\0\xA60\0\0\xA70\0\0\xAB0\0\0\xAC0\0\0\xAD0\0\0\xAE0\0\0\xAF0\0\0\xB00\0\0\xB10\0\0\xB20\0\0\xB30\0\0\xB40\0\0\xB50\0\0\xB60\0\0\xB70\0\0\xB80\0\0\xB90\0\0\xBA0\0\0\xBB0\0\0\xBC0\0\0\xBD0\0\0\xBE0\0\0\xBF0\0\0\xC00\0\0\xC10\0\0\xC20\0\0\xC40\0\0\xC50\0\0\xC60\0\0\xC70\0\0\xC80\0\0\xC90\0\0\xCF0\0\0\xD00\0\0\xD20\0\0\xD30\0\0\xD50\0\0\xD60\0\0\xD80\0\0\xD90\0\0\xDB0\0\0\xDC0\0\0\xEF0\0\0\xF30\0\0\xFD0\0\0\xFE0\0\0o\xA6\0\0p\xA6\0\0t\xA6\0\0~\xA6\0\0\x9E\xA6\0\0\xA0\xA6\0\0\xF0\xA6\0\0\xF2\xA6\0\0\x06\xA8\0\0\x07\xA8\0\0,\xA8\0\0-\xA8\0\0\xC4\xA8\0\0\xC5\xA8\0\0\xE0\xA8\0\0\xF2\xA8\0\0+\xA9\0\0.\xA9\0\0S\xA9\0\0T\xA9\0\0\xB3\xA9\0\0\xB4\xA9\0\0\xC0\xA9\0\0\xC1\xA9\0\0\xB0\xAA\0\0\xB1\xAA\0\0\xB2\xAA\0\0\xB5\xAA\0\0\xB7\xAA\0\0\xB9\xAA\0\0\xBE\xAA\0\0\xC0\xAA\0\0\xC1\xAA\0\0\xC2\xAA\0\0\xF6\xAA\0\0\xF7\xAA\0\0\xED\xAB\0\0\xEE\xAB\0\0\0\xAC\0\0\x01\xAC\0\0\x1C\xAC\0\0\x1D\xAC\0\08\xAC\0\09\xAC\0\0T\xAC\0\0U\xAC\0\0p\xAC\0\0q\xAC\0\0\x8C\xAC\0\0\x8D\xAC\0\0\xA8\xAC\0\0\xA9\xAC\0\0\xC4\xAC\0\0\xC5\xAC\0\0\xE0\xAC\0\0\xE1\xAC\0\0\xFC\xAC\0\0\xFD\xAC\0\0\x18\xAD\0\0\x19\xAD\0\x004\xAD\0\x005\xAD\0\0P\xAD\0\0Q\xAD\0\0l\xAD\0\0m\xAD\0\0\x88\xAD\0\0\x89\xAD\0\0\xA4\xAD\0\0\xA5\xAD\0\0\xC0\xAD\0\0\xC1\xAD\0\0\xDC\xAD\0\0\xDD\xAD\0\0\xF8\xAD\0\0\xF9\xAD\0\0\x14\xAE\0\0\x15\xAE\0\x000\xAE\0\x001\xAE\0\0L\xAE\0\0M\xAE\0\0h\xAE\0\0i\xAE\0\0\x84\xAE\0\0\x85\xAE\0\0\xA0\xAE\0\0\xA1\xAE\0\0\xBC\xAE\0\0\xBD\xAE\0\0\xD8\xAE\0\0\xD9\xAE\0\0\xF4\xAE\0\0\xF5\xAE\0\0\x10\xAF\0\0\x11\xAF\0\0,\xAF\0\0-\xAF\0\0H\xAF\0\0I\xAF\0\0d\xAF\0\0e\xAF\0\0\x80\xAF\0\0\x81\xAF\0\0\x9C\xAF\0\0\x9D\xAF\0\0\xB8\xAF\0\0\xB9\xAF\0\0\xD4\xAF\0\0\xD5\xAF\0\0\xF0\xAF\0\0\xF1\xAF\0\0\x0C\xB0\0\0\r\xB0\0\0(\xB0\0\0)\xB0\0\0D\xB0\0\0E\xB0\0\0`\xB0\0\0a\xB0\0\0|\xB0\0\0}\xB0\0\0\x98\xB0\0\0\x99\xB0\0\0\xB4\xB0\0\0\xB5\xB0\0\0\xD0\xB0\0\0\xD1\xB0\0\0\xEC\xB0\0\0\xED\xB0\0\0\x08\xB1\0\0\t\xB1\0\0$\xB1\0\0%\xB1\0\0@\xB1\0\0A\xB1\0\0\\\xB1\0\0]\xB1\0\0x\xB1\0\0y\xB1\0\0\x94\xB1\0\0\x95\xB1\0\0\xB0\xB1\0\0\xB1\xB1\0\0\xCC\xB1\0\0\xCD\xB1\0\0\xE8\xB1\0\0\xE9\xB1\0\0\x04\xB2\0\0\x05\xB2\0\0 \xB2\0\0!\xB2\0\0<\xB2\0\0=\xB2\0\0X\xB2\0\0Y\xB2\0\0t\xB2\0\0u\xB2\0\0\x90\xB2\0\0\x91\xB2\0\0\xAC\xB2\0\0\xAD\xB2\0\0\xC8\xB2\0\0\xC9\xB2\0\0\xE4\xB2\0\0\xE5\xB2\0\0\0\xB3\0\0\x01\xB3\0\0\x1C\xB3\0\0\x1D\xB3\0\08\xB3\0\09\xB3\0\0T\xB3\0\0U\xB3\0\0p\xB3\0\0q\xB3\0\0\x8C\xB3\0\0\x8D\xB3\0\0\xA8\xB3\0\0\xA9\xB3\0\0\xC4\xB3\0\0\xC5\xB3\0\0\xE0\xB3\0\0\xE1\xB3\0\0\xFC\xB3\0\0\xFD\xB3\0\0\x18\xB4\0\0\x19\xB4\0\x004\xB4\0\x005\xB4\0\0P\xB4\0\0Q\xB4\0\0l\xB4\0\0m\xB4\0\0\x88\xB4\0\0\x89\xB4\0\0\xA4\xB4\0\0\xA5\xB4\0\0\xC0\xB4\0\0\xC1\xB4\0\0\xDC\xB4\0\0\xDD\xB4\0\0\xF8\xB4\0\0\xF9\xB4\0\0\x14\xB5\0\0\x15\xB5\0\x000\xB5\0\x001\xB5\0\0L\xB5\0\0M\xB5\0\0h\xB5\0\0i\xB5\0\0\x84\xB5\0\0\x85\xB5\0\0\xA0\xB5\0\0\xA1\xB5\0\0\xBC\xB5\0\0\xBD\xB5\0\0\xD8\xB5\0\0\xD9\xB5\0\0\xF4\xB5\0\0\xF5\xB5\0\0\x10\xB6\0\0\x11\xB6\0\0,\xB6\0\0-\xB6\0\0H\xB6\0\0I\xB6\0\0d\xB6\0\0e\xB6\0\0\x80\xB6\0\0\x81\xB6\0\0\x9C\xB6\0\0\x9D\xB6\0\0\xB8\xB6\0\0\xB9\xB6\0\0\xD4\xB6\0\0\xD5\xB6\0\0\xF0\xB6\0\0\xF1\xB6\0\0\x0C\xB7\0\0\r\xB7\0\0(\xB7\0\0)\xB7\0\0D\xB7\0\0E\xB7\0\0`\xB7\0\0a\xB7\0\0|\xB7\0\0}\xB7\0\0\x98\xB7\0\0\x99\xB7\0\0\xB4\xB7\0\0\xB5\xB7\0\0\xD0\xB7\0\0\xD1\xB7\0\0\xEC\xB7\0\0\xED\xB7\0\0\x08\xB8\0\0\t\xB8\0\0$\xB8\0\0%\xB8\0\0@\xB8\0\0A\xB8\0\0\\\xB8\0\0]\xB8\0\0x\xB8\0\0y\xB8\0\0\x94\xB8\0\0\x95\xB8\0\0\xB0\xB8\0\0\xB1\xB8\0\0\xCC\xB8\0\0\xCD\xB8\0\0\xE8\xB8\0\0\xE9\xB8\0\0\x04\xB9\0\0\x05\xB9\0\0 \xB9\0\0!\xB9\0\0<\xB9\0\0=\xB9\0\0X\xB9\0\0Y\xB9\0\0t\xB9\0\0u\xB9\0\0\x90\xB9\0\0\x91\xB9\0\0\xAC\xB9\0\0\xAD\xB9\0\0\xC8\xB9\0\0\xC9\xB9\0\0\xE4\xB9\0\0\xE5\xB9\0\0\0\xBA\0\0\x01\xBA\0\0\x1C\xBA\0\0\x1D\xBA\0\08\xBA\0\09\xBA\0\0T\xBA\0\0U\xBA\0\0p\xBA\0\0q\xBA\0\0\x8C\xBA\0\0\x8D\xBA\0\0\xA8\xBA\0\0\xA9\xBA\0\0\xC4\xBA\0\0\xC5\xBA\0\0\xE0\xBA\0\0\xE1\xBA\0\0\xFC\xBA\0\0\xFD\xBA\0\0\x18\xBB\0\0\x19\xBB\0\x004\xBB\0\x005\xBB\0\0P\xBB\0\0Q\xBB\0\0l\xBB\0\0m\xBB\0\0\x88\xBB\0\0\x89\xBB\0\0\xA4\xBB\0\0\xA5\xBB\0\0\xC0\xBB\0\0\xC1\xBB\0\0\xDC\xBB\0\0\xDD\xBB\0\0\xF8\xBB\0\0\xF9\xBB\0\0\x14\xBC\0\0\x15\xBC\0\x000\xBC\0\x001\xBC\0\0L\xBC\0\0M\xBC\0\0h\xBC\0\0i\xBC\0\0\x84\xBC\0\0\x85\xBC\0\0\xA0\xBC\0\0\xA1\xBC\0\0\xBC\xBC\0\0\xBD\xBC\0\0\xD8\xBC\0\0\xD9\xBC\0\0\xF4\xBC\0\0\xF5\xBC\0\0\x10\xBD\0\0\x11\xBD\0\0,\xBD\0\0-\xBD\0\0H\xBD\0\0I\xBD\0\0d\xBD\0\0e\xBD\0\0\x80\xBD\0\0\x81\xBD\0\0\x9C\xBD\0\0\x9D\xBD\0\0\xB8\xBD\0\0\xB9\xBD\0\0\xD4\xBD\0\0\xD5\xBD\0\0\xF0\xBD\0\0\xF1\xBD\0\0\x0C\xBE\0\0\r\xBE\0\0(\xBE\0\0)\xBE\0\0D\xBE\0\0E\xBE\0\0`\xBE\0\0a\xBE\0\0|\xBE\0\0}\xBE\0\0\x98\xBE\0\0\x99\xBE\0\0\xB4\xBE\0\0\xB5\xBE\0\0\xD0\xBE\0\0\xD1\xBE\0\0\xEC\xBE\0\0\xED\xBE\0\0\x08\xBF\0\0\t\xBF\0\0$\xBF\0\0%\xBF\0\0@\xBF\0\0A\xBF\0\0\\\xBF\0\0]\xBF\0\0x\xBF\0\0y\xBF\0\0\x94\xBF\0\0\x95\xBF\0\0\xB0\xBF\0\0\xB1\xBF\0\0\xCC\xBF\0\0\xCD\xBF\0\0\xE8\xBF\0\0\xE9\xBF\0\0\x04\xC0\0\0\x05\xC0\0\0 \xC0\0\0!\xC0\0\0<\xC0\0\0=\xC0\0\0X\xC0\0\0Y\xC0\0\0t\xC0\0\0u\xC0\0\0\x90\xC0\0\0\x91\xC0\0\0\xAC\xC0\0\0\xAD\xC0\0\0\xC8\xC0\0\0\xC9\xC0\0\0\xE4\xC0\0\0\xE5\xC0\0\0\0\xC1\0\0\x01\xC1\0\0\x1C\xC1\0\0\x1D\xC1\0\08\xC1\0\09\xC1\0\0T\xC1\0\0U\xC1\0\0p\xC1\0\0q\xC1\0\0\x8C\xC1\0\0\x8D\xC1\0\0\xA8\xC1\0\0\xA9\xC1\0\0\xC4\xC1\0\0\xC5\xC1\0\0\xE0\xC1\0\0\xE1\xC1\0\0\xFC\xC1\0\0\xFD\xC1\0\0\x18\xC2\0\0\x19\xC2\0\x004\xC2\0\x005\xC2\0\0P\xC2\0\0Q\xC2\0\0l\xC2\0\0m\xC2\0\0\x88\xC2\0\0\x89\xC2\0\0\xA4\xC2\0\0\xA5\xC2\0\0\xC0\xC2\0\0\xC1\xC2\0\0\xDC\xC2\0\0\xDD\xC2\0\0\xF8\xC2\0\0\xF9\xC2\0\0\x14\xC3\0\0\x15\xC3\0\x000\xC3\0\x001\xC3\0\0L\xC3\0\0M\xC3\0\0h\xC3\0\0i\xC3\0\0\x84\xC3\0\0\x85\xC3\0\0\xA0\xC3\0\0\xA1\xC3\0\0\xBC\xC3\0\0\xBD\xC3\0\0\xD8\xC3\0\0\xD9\xC3\0\0\xF4\xC3\0\0\xF5\xC3\0\0\x10\xC4\0\0\x11\xC4\0\0,\xC4\0\0-\xC4\0\0H\xC4\0\0I\xC4\0\0d\xC4\0\0e\xC4\0\0\x80\xC4\0\0\x81\xC4\0\0\x9C\xC4\0\0\x9D\xC4\0\0\xB8\xC4\0\0\xB9\xC4\0\0\xD4\xC4\0\0\xD5\xC4\0\0\xF0\xC4\0\0\xF1\xC4\0\0\x0C\xC5\0\0\r\xC5\0\0(\xC5\0\0)\xC5\0\0D\xC5\0\0E\xC5\0\0`\xC5\0\0a\xC5\0\0|\xC5\0\0}\xC5\0\0\x98\xC5\0\0\x99\xC5\0\0\xB4\xC5\0\0\xB5\xC5\0\0\xD0\xC5\0\0\xD1\xC5\0\0\xEC\xC5\0\0\xED\xC5\0\0\x08\xC6\0\0\t\xC6\0\0$\xC6\0\0%\xC6\0\0@\xC6\0\0A\xC6\0\0\\\xC6\0\0]\xC6\0\0x\xC6\0\0y\xC6\0\0\x94\xC6\0\0\x95\xC6\0\0\xB0\xC6\0\0\xB1\xC6\0\0\xCC\xC6\0\0\xCD\xC6\0\0\xE8\xC6\0\0\xE9\xC6\0\0\x04\xC7\0\0\x05\xC7\0\0 \xC7\0\0!\xC7\0\0<\xC7\0\0=\xC7\0\0X\xC7\0\0Y\xC7\0\0t\xC7\0\0u\xC7\0\0\x90\xC7\0\0\x91\xC7\0\0\xAC\xC7\0\0\xAD\xC7\0\0\xC8\xC7\0\0\xC9\xC7\0\0\xE4\xC7\0\0\xE5\xC7\0\0\0\xC8\0\0\x01\xC8\0\0\x1C\xC8\0\0\x1D\xC8\0\08\xC8\0\09\xC8\0\0T\xC8\0\0U\xC8\0\0p\xC8\0\0q\xC8\0\0\x8C\xC8\0\0\x8D\xC8\0\0\xA8\xC8\0\0\xA9\xC8\0\0\xC4\xC8\0\0\xC5\xC8\0\0\xE0\xC8\0\0\xE1\xC8\0\0\xFC\xC8\0\0\xFD\xC8\0\0\x18\xC9\0\0\x19\xC9\0\x004\xC9\0\x005\xC9\0\0P\xC9\0\0Q\xC9\0\0l\xC9\0\0m\xC9\0\0\x88\xC9\0\0\x89\xC9\0\0\xA4\xC9\0\0\xA5\xC9\0\0\xC0\xC9\0\0\xC1\xC9\0\0\xDC\xC9\0\0\xDD\xC9\0\0\xF8\xC9\0\0\xF9\xC9\0\0\x14\xCA\0\0\x15\xCA\0\x000\xCA\0\x001\xCA\0\0L\xCA\0\0M\xCA\0\0h\xCA\0\0i\xCA\0\0\x84\xCA\0\0\x85\xCA\0\0\xA0\xCA\0\0\xA1\xCA\0\0\xBC\xCA\0\0\xBD\xCA\0\0\xD8\xCA\0\0\xD9\xCA\0\0\xF4\xCA\0\0\xF5\xCA\0\0\x10\xCB\0\0\x11\xCB\0\0,\xCB\0\0-\xCB\0\0H\xCB\0\0I\xCB\0\0d\xCB\0\0e\xCB\0\0\x80\xCB\0\0\x81\xCB\0\0\x9C\xCB\0\0\x9D\xCB\0\0\xB8\xCB\0\0\xB9\xCB\0\0\xD4\xCB\0\0\xD5\xCB\0\0\xF0\xCB\0\0\xF1\xCB\0\0\x0C\xCC\0\0\r\xCC\0\0(\xCC\0\0)\xCC\0\0D\xCC\0\0E\xCC\0\0`\xCC\0\0a\xCC\0\0|\xCC\0\0}\xCC\0\0\x98\xCC\0\0\x99\xCC\0\0\xB4\xCC\0\0\xB5\xCC\0\0\xD0\xCC\0\0\xD1\xCC\0\0\xEC\xCC\0\0\xED\xCC\0\0\x08\xCD\0\0\t\xCD\0\0$\xCD\0\0%\xCD\0\0@\xCD\0\0A\xCD\0\0\\\xCD\0\0]\xCD\0\0x\xCD\0\0y\xCD\0\0\x94\xCD\0\0\x95\xCD\0\0\xB0\xCD\0\0\xB1\xCD\0\0\xCC\xCD\0\0\xCD\xCD\0\0\xE8\xCD\0\0\xE9\xCD\0\0\x04\xCE\0\0\x05\xCE\0\0 \xCE\0\0!\xCE\0\0<\xCE\0\0=\xCE\0\0X\xCE\0\0Y\xCE\0\0t\xCE\0\0u\xCE\0\0\x90\xCE\0\0\x91\xCE\0\0\xAC\xCE\0\0\xAD\xCE\0\0\xC8\xCE\0\0\xC9\xCE\0\0\xE4\xCE\0\0\xE5\xCE\0\0\0\xCF\0\0\x01\xCF\0\0\x1C\xCF\0\0\x1D\xCF\0\08\xCF\0\09\xCF\0\0T\xCF\0\0U\xCF\0\0p\xCF\0\0q\xCF\0\0\x8C\xCF\0\0\x8D\xCF\0\0\xA8\xCF\0\0\xA9\xCF\0\0\xC4\xCF\0\0\xC5\xCF\0\0\xE0\xCF\0\0\xE1\xCF\0\0\xFC\xCF\0\0\xFD\xCF\0\0\x18\xD0\0\0\x19\xD0\0\x004\xD0\0\x005\xD0\0\0P\xD0\0\0Q\xD0\0\0l\xD0\0\0m\xD0\0\0\x88\xD0\0\0\x89\xD0\0\0\xA4\xD0\0\0\xA5\xD0\0\0\xC0\xD0\0\0\xC1\xD0\0\0\xDC\xD0\0\0\xDD\xD0\0\0\xF8\xD0\0\0\xF9\xD0\0\0\x14\xD1\0\0\x15\xD1\0\x000\xD1\0\x001\xD1\0\0L\xD1\0\0M\xD1\0\0h\xD1\0\0i\xD1\0\0\x84\xD1\0\0\x85\xD1\0\0\xA0\xD1\0\0\xA1\xD1\0\0\xBC\xD1\0\0\xBD\xD1\0\0\xD8\xD1\0\0\xD9\xD1\0\0\xF4\xD1\0\0\xF5\xD1\0\0\x10\xD2\0\0\x11\xD2\0\0,\xD2\0\0-\xD2\0\0H\xD2\0\0I\xD2\0\0d\xD2\0\0e\xD2\0\0\x80\xD2\0\0\x81\xD2\0\0\x9C\xD2\0\0\x9D\xD2\0\0\xB8\xD2\0\0\xB9\xD2\0\0\xD4\xD2\0\0\xD5\xD2\0\0\xF0\xD2\0\0\xF1\xD2\0\0\x0C\xD3\0\0\r\xD3\0\0(\xD3\0\0)\xD3\0\0D\xD3\0\0E\xD3\0\0`\xD3\0\0a\xD3\0\0|\xD3\0\0}\xD3\0\0\x98\xD3\0\0\x99\xD3\0\0\xB4\xD3\0\0\xB5\xD3\0\0\xD0\xD3\0\0\xD1\xD3\0\0\xEC\xD3\0\0\xED\xD3\0\0\x08\xD4\0\0\t\xD4\0\0$\xD4\0\0%\xD4\0\0@\xD4\0\0A\xD4\0\0\\\xD4\0\0]\xD4\0\0x\xD4\0\0y\xD4\0\0\x94\xD4\0\0\x95\xD4\0\0\xB0\xD4\0\0\xB1\xD4\0\0\xCC\xD4\0\0\xCD\xD4\0\0\xE8\xD4\0\0\xE9\xD4\0\0\x04\xD5\0\0\x05\xD5\0\0 \xD5\0\0!\xD5\0\0<\xD5\0\0=\xD5\0\0X\xD5\0\0Y\xD5\0\0t\xD5\0\0u\xD5\0\0\x90\xD5\0\0\x91\xD5\0\0\xAC\xD5\0\0\xAD\xD5\0\0\xC8\xD5\0\0\xC9\xD5\0\0\xE4\xD5\0\0\xE5\xD5\0\0\0\xD6\0\0\x01\xD6\0\0\x1C\xD6\0\0\x1D\xD6\0\08\xD6\0\09\xD6\0\0T\xD6\0\0U\xD6\0\0p\xD6\0\0q\xD6\0\0\x8C\xD6\0\0\x8D\xD6\0\0\xA8\xD6\0\0\xA9\xD6\0\0\xC4\xD6\0\0\xC5\xD6\0\0\xE0\xD6\0\0\xE1\xD6\0\0\xFC\xD6\0\0\xFD\xD6\0\0\x18\xD7\0\0\x19\xD7\0\x004\xD7\0\x005\xD7\0\0P\xD7\0\0Q\xD7\0\0l\xD7\0\0m\xD7\0\0\x88\xD7\0\0\x89\xD7\0\0\0\xF9\0\0\x0E\xFA\0\0\x10\xFA\0\0\x11\xFA\0\0\x12\xFA\0\0\x13\xFA\0\0\x15\xFA\0\0\x1F\xFA\0\0 \xFA\0\0!\xFA\0\0\"\xFA\0\0#\xFA\0\0%\xFA\0\0'\xFA\0\0*\xFA\0\0n\xFA\0\0p\xFA\0\0\xDA\xFA\0\0\x1D\xFB\0\0 \xFB\0\0*\xFB\0\x007\xFB\0\08\xFB\0\0=\xFB\0\0>\xFB\0\0?\xFB\0\0@\xFB\0\0B\xFB\0\0C\xFB\0\0E\xFB\0\0F\xFB\0\0O\xFB\0\0 \xFE\0\x000\xFE\0\0\xFD\x01\x01\0\xFE\x01\x01\0\xE0\x02\x01\0\xE1\x02\x01\0v\x03\x01\0{\x03\x01\0\r\n\x01\0\x0E\n\x01\0\x0F\n\x01\0\x10\n\x01\08\n\x01\0;\n\x01\0?\n\x01\0@\n\x01\0\xE5\n\x01\0\xE7\n\x01\0$\r\x01\0(\r\x01\0\xAB\x0E\x01\0\xAD\x0E\x01\0\xFD\x0E\x01\0\0\x0F\x01\0F\x0F\x01\0Q\x0F\x01\0\x82\x0F\x01\0\x86\x0F\x01\0F\x10\x01\0G\x10\x01\0p\x10\x01\0q\x10\x01\0\x7F\x10\x01\0\x80\x10\x01\0\x99\x10\x01\0\x9A\x10\x01\0\x9B\x10\x01\0\x9C\x10\x01\0\xA5\x10\x01\0\xA6\x10\x01\0\xB9\x10\x01\0\xBB\x10\x01\0\0\x11\x01\0\x03\x11\x01\0'\x11\x01\0(\x11\x01\x001\x11\x01\x005\x11\x01\0s\x11\x01\0t\x11\x01\0\xC0\x11\x01\0\xC1\x11\x01\0\xCA\x11\x01\0\xCB\x11\x01\x005\x12\x01\x007\x12\x01\0\xE9\x12\x01\0\xEB\x12\x01\0;\x13\x01\0=\x13\x01\0>\x13\x01\0?\x13\x01\0G\x13\x01\0H\x13\x01\0M\x13\x01\0N\x13\x01\0W\x13\x01\0X\x13\x01\0f\x13\x01\0m\x13\x01\0p\x13\x01\0u\x13\x01\0B\x14\x01\0C\x14\x01\0F\x14\x01\0G\x14\x01\0^\x14\x01\0_\x14\x01\0\xB0\x14\x01\0\xB1\x14\x01\0\xB9\x14\x01\0\xBB\x14\x01\0\xBD\x14\x01\0\xBE\x14\x01\0\xC2\x14\x01\0\xC4\x14\x01\0\xAF\x15\x01\0\xB0\x15\x01\0\xB8\x15\x01\0\xBA\x15\x01\0\xBF\x15\x01\0\xC1\x15\x01\0?\x16\x01\0@\x16\x01\0\xB6\x16\x01\0\xB8\x16\x01\0+\x17\x01\0,\x17\x01\09\x18\x01\0;\x18\x01\x000\x19\x01\x001\x19\x01\x005\x19\x01\x006\x19\x01\0=\x19\x01\0?\x19\x01\0C\x19\x01\0D\x19\x01\0\xE0\x19\x01\0\xE1\x19\x01\x004\x1A\x01\x005\x1A\x01\0G\x1A\x01\0H\x1A\x01\0\x99\x1A\x01\0\x9A\x1A\x01\0?\x1C\x01\0@\x1C\x01\0B\x1D\x01\0C\x1D\x01\0D\x1D\x01\0F\x1D\x01\0\x97\x1D\x01\0\x98\x1D\x01\0A\x1F\x01\0C\x1F\x01\0\xF0j\x01\0\xF5j\x01\x000k\x01\x007k\x01\0\xF0o\x01\0\xF2o\x01\0\x9E\xBC\x01\0\x9F\xBC\x01\0^\xD1\x01\0j\xD1\x01\0m\xD1\x01\0s\xD1\x01\0{\xD1\x01\0\x83\xD1\x01\0\x85\xD1\x01\0\x8C\xD1\x01\0\xAA\xD1\x01\0\xAE\xD1\x01\0\xBB\xD1\x01\0\xC1\xD1\x01\0B\xD2\x01\0E\xD2\x01\0\0\xE0\x01\0\x07\xE0\x01\0\x08\xE0\x01\0\x19\xE0\x01\0\x1B\xE0\x01\0\"\xE0\x01\0#\xE0\x01\0%\xE0\x01\0&\xE0\x01\0+\xE0\x01\0\x8F\xE0\x01\0\x90\xE0\x01\x000\xE1\x01\x007\xE1\x01\0\xAE\xE2\x01\0\xAF\xE2\x01\0\xEC\xE2\x01\0\xF0\xE2\x01\0\xEC\xE4\x01\0\xF0\xE4\x01\0\xD0\xE8\x01\0\xD7\xE8\x01\0D\xE9\x01\0K\xE9\x01\0\0\xF8\x02\0\x1E\xFA\x02\0\0\0\x11\0") }, 1110818usize)
            });
            #[doc(hidden)]
            pub fn lookup_props_nfcinert_v1(locale: &icu_provider::DataLocale) -> Result<&'static <icu_properties::provider::NfcInertV1Marker as icu_provider::DataMarker>::Yokeable, icu_provider::DataErrorKind> {
                if locale.is_empty() {
                    Ok(Self::SINGLETON_PROPS_NFCINERT_V1)
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale)
                }
            }
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::NfcInertV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::NfcInertV1Marker>, icu_provider::DataError> {
                match Self::lookup_props_nfcinert_v1(&req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::NfcInertV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}

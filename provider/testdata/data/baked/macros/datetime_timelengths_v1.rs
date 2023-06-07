// @generated
/// Implement [`DataProvider<TimeLengthsV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_timelengths_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub fn lookup_datetime_timelengths_v1(locale: &icu_provider::DataLocale) -> Result<&'static <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable, icu_provider::DataErrorKind> {
                static TR: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\0 /\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\0 /\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\0 /\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\0 /\x80q\x01\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H23H24 };
                static JA: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\x80p\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\x80p\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\x80p\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80`\x01\x80p\x01\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0fB\x80\x80\x02\0R\x06\x80\x90\x02\0y\xD2\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H23H24 };
                static EN: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0 /\x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H11H12 };
                static ES: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0 /\x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \0\0(\x80\xA0\x04\0\0)") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H23H24 };
                static EN_ZA: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0 /\x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H23H24 };
                static ES_AR: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0 /\x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0 /\x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H23H24 };
                static AR: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0 \x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H11H12 };
                static TH: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0 \x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0 \0\x0E\x19\0\x0E2\0\x0E,\0\x0E4\0\x0E\x01\0\x0E2\0\0 \x80\x80\x02\0\0 \0\x0E\x19\0\x0E2\0\x0E\x17\0\x0E5\0\0 \x80\x90\x02\0\0 \0\x0E'\0\x0E4\0\x0E\x19\0\x0E2\0\x0E\x17\0\x0E5\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x01\0\0 \0\x0E\x19\0\x0E2\0\x0E,\0\x0E4\0\x0E\x01\0\x0E2\0\0 \x80\x80\x02\0\0 \0\x0E\x19\0\x0E2\0\x0E\x17\0\x0E5\0\0 \x80\x90\x02\0\0 \0\x0E'\0\x0E4\0\x0E\x19\0\x0E2\0\x0E\x17\0\x0E5\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H23H24 };
                static UND: <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::Yokeable = icu_datetime::provider::calendar::TimeLengthsV1 { time_h11_h12: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01\0\0 \x80\xA3\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80q\x01\0\0:\x80\x80\x02\0\0 \x80`\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, time_h23_h24: icu_datetime::provider::calendar::patterns::LengthPatternsV1 { full: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x04") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, long: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02\0\0 \x80\xA0\x01") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, medium: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02\0\0:\x80\x90\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Seconds }, short: icu_datetime::pattern::runtime::Pattern { items: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80r\x02\0\0:\x80\x80\x02") }, time_granularity: icu_datetime::pattern::TimeGranularity::Minutes } }, preferred_hour_cycle: icu_datetime::pattern::CoarseHourCycle::H23H24 };
                match ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                    Ok(i) => Ok(*unsafe { [&AR, &AR, &AR, &AR, &EN, &EN, &EN_ZA, &ES, &ES_AR, &EN, &EN_ZA, &JA, &EN_ZA, &EN_ZA, &EN_ZA, &EN_ZA, &TH, &TR, &UND].get_unchecked(i) }),
                    Err(_) => Err(icu_provider::DataErrorKind::MissingLocale),
                }
            }
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_datetime::provider::calendar::TimeLengthsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_datetime::provider::calendar::TimeLengthsV1Marker>, icu_provider::DataError> {
                match Self::lookup_datetime_timelengths_v1(&req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}

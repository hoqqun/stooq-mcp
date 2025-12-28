use super::*;

#[test]
fn test_market_as_str() {
    assert_eq!(Market::Jp.as_str(), "jp");
    assert_eq!(Market::Us.as_str(), "us");
    assert_eq!(Market::Uk.as_str(), "uk");
    assert_eq!(Market::Hk.as_str(), "hk");
    assert_eq!(Market::De.as_str(), "de");
}

#[test]
fn test_build_stooq_url_latest() {
    let url = build_stooq_url("7203", &Market::Jp, None, None);
    assert_eq!(url, "https://stooq.com/q/l/?s=7203.jp&f=sd2t2ohlcv&h&e=csv");
}

#[test]
fn test_build_stooq_url_latest_us() {
    let url = build_stooq_url("AAPL", &Market::Us, None, None);
    assert_eq!(url, "https://stooq.com/q/l/?s=AAPL.us&f=sd2t2ohlcv&h&e=csv");
}

#[test]
fn test_build_stooq_url_historical() {
    let url = build_stooq_url("7203", &Market::Jp, Some("20240101"), Some("20241231"));
    assert_eq!(url, "https://stooq.com/q/d/l/?s=7203.jp&d1=20240101&d2=20241231&i=d");
}

#[test]
fn test_build_stooq_url_partial_date_uses_latest() {
    // Only start_date provided, should use latest URL
    let url = build_stooq_url("7203", &Market::Jp, Some("20240101"), None);
    assert_eq!(url, "https://stooq.com/q/l/?s=7203.jp&f=sd2t2ohlcv&h&e=csv");

    // Only end_date provided, should use latest URL
    let url = build_stooq_url("7203", &Market::Jp, None, Some("20241231"));
    assert_eq!(url, "https://stooq.com/q/l/?s=7203.jp&f=sd2t2ohlcv&h&e=csv");
}

#[test]
fn test_build_stooq_url_all_markets() {
    let markets = vec![
        (Market::Jp, "jp"),
        (Market::Us, "us"),
        (Market::Uk, "uk"),
        (Market::Hk, "hk"),
        (Market::De, "de"),
    ];

    for (market, code) in markets {
        let url = build_stooq_url("TEST", &market, None, None);
        assert!(url.contains(&format!(".{}", code)), "URL should contain .{}", code);
    }
}

#[test]
fn test_market_deserialize() {
    let jp: Market = serde_json::from_str(r#""jp""#).unwrap();
    assert_eq!(jp, Market::Jp);

    let us: Market = serde_json::from_str(r#""us""#).unwrap();
    assert_eq!(us, Market::Us);
}

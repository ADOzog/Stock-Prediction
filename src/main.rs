use yahoo_finance_api as yahoo;
use yahoo::time::Date;
use yahoo::time::Time;
use yahoo::time::UtcOffset;
use yahoo::time::OffsetDateTime;
use tokio_test;

fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = OffsetDateTime::new_utc(
    Date::from_calendar_date(2024, yahoo::time::Month::January, 1).expect("How"),
    Time::from_hms_nano(0, 0, 0, 500_000_000).expect("How"),
);
    let end =  OffsetDateTime::new_utc(
    Date::from_calendar_date(2024, yahoo::time::Month::January, 31).expect("How"),
    Time::from_hms_nano(0, 0, 0, 500_000_000).expect("How"),
);

    // returns historic quotes with daily interval
    let resp = tokio_test::block_on(provider.get_quote_history("AAPL", start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    for quote in quotes{
    println!("Apple's quotes in January: {:?}", quote);
    }
}

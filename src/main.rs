use yahoo_finance_api as yahoo;

use std::io;
//use yahoo::time::Date;
use tokio_test;

// takes the stock and the number of days ago as input then returns the yahoo_finance response
fn get_quotes(days_ago: &i64, stock: &str) -> yahoo::YResponse {
    let provider = yahoo::YahooConnector::new().unwrap();

    // get today as the end and today minus days ago as the start
    let start = time::UtcDateTime::now() - time::Duration::days(*days_ago);
    let end = time::UtcDateTime::now();

    // returns historic quotes with daily interval
    tokio_test::block_on(provider.get_quote_history(stock, start.into(), end.into())).unwrap()
}




fn main() {



    let mut first_line = String::new();

    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read line");



    let stock: &str = first_line.trim();





    
    let days_ago : i64  = 200;
    let resp = get_quotes(&days_ago, stock);
    let quotes = resp.quotes().unwrap();
    
    // could be more space efficent here
    let mut ema : Vec<f64> = vec![0.0; quotes.len() - 1];
    let mut emv : Vec<f64> = vec![0.0; quotes.len() - 1];
    
    ema[0] = (quotes[1].close / quotes[0].close) - 1.0;
    emv[0] = quotes[0].close;


    let alpha : f64 = 2.0/(days_ago as f64 + 1.0) as f64;
    for i in 1..quotes.len()-1 {
//        println!(
//            "Apple's quotes on {:?} was {:?}",
//            quote.timestamp, quote.close
//        );
    let r_t : f64 = (quotes[i + 1].close / quotes[i].close) - 1.0;
    ema[i] = alpha * r_t + (1.0 - alpha) * ema[i-1];
    emv[i] = alpha * (r_t - ema[i]).powf(2.0) + (1.0 - alpha) * emv[i-1];
    }
//    println!("The EMA is {:?}", EMA);
//    println!("The EMV is {:?}", EMV);
    let mu = ema[quotes.len() - 2];
    let sigma = emv[quotes.len() - 2].powf(0.5);
    println!("The mu is {:?}", mu);
    println!("The sigma is {:?}", sigma);
}

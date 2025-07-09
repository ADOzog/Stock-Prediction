use yahoo_finance_api as yahoo;

use std::io;
//use yahoo::time::Date;
use tokio_test;

use diffusionx::simulation::{continuous::geometric_bm::GeometricBm, prelude::*};

use plotters::prelude::*;

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

    let days_ago: i64 = 200;
    let resp = get_quotes(&days_ago, stock);
    let quotes = resp.quotes().unwrap();

    // could be more space efficent here
    let mut ema: Vec<f64> = vec![0.0; quotes.len() - 1];
    let mut emv: Vec<f64> = vec![0.0; quotes.len() - 1];

    ema[0] = (quotes[1].close / quotes[0].close).ln();
    emv[0] = (quotes[1].close / quotes[0].close).ln().powf(2.0);

    let alpha: f64 = 2.0 / (days_ago as f64 + 1.0) as f64;
    for i in 1..quotes.len() - 1 {
        //        println!(
        //            "Apple's quotes on {:?} was {:?}",
        //            quote.timestamp, quote.close
        //        );
        let r_t: f64 = (quotes[i + 1].close / quotes[i].close).ln();
        ema[i] = alpha * r_t + (1.0 - alpha) * ema[i - 1];
        emv[i] = alpha * (r_t - ema[i]).powf(2.0) + (1.0 - alpha) * emv[i - 1];
    }
    //    println!("The EMA is {:?}", ema);
    //    println!("The EMV is {:?}", emv);
    let mu = ema[quotes.len() - 2];
    let sigma = emv[quotes.len() - 2].powf(0.5);
    println!("The mu is {:?}", mu);
    println!("The sigma is {:?}", sigma);

    let start_pos: f64 = quotes[quotes.len() - 1].close;

    let gbm = GeometricBm::new(start_pos, mu, sigma).expect("This should never happen");

    let base_title: String = String::from("Geometric Brownian Motion Simulation for ");
    let full_title: String = base_title.chars().chain(stock.chars()).collect();

    let root = BitMapBackend::new("GMB.png", (1080, 640)).into_drawing_area();
    root.fill(&WHITE).expect("stop it");
    let mut chart = ChartBuilder::on(&root)
        .caption(full_title, ("sans-serif", 40).into_font())
        .margin(12)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(0_f64..200_f64, (0.0)..(start_pos * 4.0))
        .expect("Should never happen");

    // Draw the x and y axes
    chart
        .configure_mesh()
        .y_desc("Price")
        .x_desc("Days From Today")
        .draw()
        .expect("stop it");
    // The number of loops controls the number of realizations

    // I want to add an average line and a no change line
    for _ in 0..100 {
        let (x, t) = gbm.simulate(200.0, 1.0).unwrap();
        let points: Vec<(f64, f64)> = x
            .iter()
            .zip(t.iter())
            .map(|(&x_val, &y_val)| (x_val, y_val))
            .collect();

        // Could move this around then loop for more gbm realizations
        // Create a chart builder
        chart
            .draw_series(LineSeries::new(points, &BLUE))
            .expect("should never happen");
    }
    //.label("")
    //.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    //chart
    //.configure_series_labels()
    //.background_style(&WHITE.mix(0.8))
    //.border_style(&TRANSPARENT)
    //.draw()
    //.expect("should never happen");

    root.present().expect("stop it");
}

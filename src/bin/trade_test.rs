use fmp::trade::{BuyAndHold, Backtester};
use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;

    let symbols = vec!["005930.KS".to_string(), "005490.KS".to_string()];
    let from = "2018-01-01";
    let to = "2023-10-11";
    let init_balance = 1000_0000f64;
    
    let weight = vec![0.5, 0.5];
    let rebalance = 60;

    let bnh = BuyAndHold::new(&weight, rebalance);
    let mut bt = Backtester::new(&symbols, init_balance, Box::new(bnh), from, to, &api_key)?;

    let report = bt.run(120);
    let df = report.to_dataframe();
    let dg = report.to_report();

    df.print();
    dg.print();

    Ok(())
}

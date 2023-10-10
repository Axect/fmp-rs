use peroxide::fuga::*;
use fmp::strategy::*;
use fmp::api::HistoricalPriceFull;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;

    let symbol = args().nth(1).unwrap_or("005930.KS".to_string());
    let from = "2018-01-01";
    let to = "2023-10-10";

    let mut hp = HistoricalPriceFull::new(&symbol);
    hp.download_interval(&api_key, from, to)?;

    let mut df = hp.to_dataframe_simple();
    let open: Vec<f64> = df["open"].to_vec();
    let close: Vec<f64> = df["close"].to_vec();

    let init = open[0];
    let bnh = BuyAndHold::new(init, &close);
    let bnh_dr = bnh.daily_return();
    let bnh_cr = bnh.cumulative_return();
    let bnh_vol = bnh.volatility(120);
    let bnh_sr = bnh.sharpe_ratio(120);

    df.push("bnh", Series::new(bnh_dr));
    df.push("bnh_cr", Series::new(bnh_cr));
    df.push("bnh_vol", Series::new(bnh_vol));
    df.push("bnh_sr", Series::new(bnh_sr));

    df.print();

    df.write_parquet(&format!("data/{}_bnh.parquet", symbol), CompressionOptions::Uncompressed)?;

    Ok(())
}

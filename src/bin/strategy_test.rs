use peroxide::fuga::*;
use fmp::strategy::*;
use fmp::ta::*;
use fmp::api::{HistoricalPriceFull, DailyTreasury};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;

    let symbol = args().nth(1).unwrap_or("005930.KS".to_string());
    let from = "2018-01-01";
    let to = "2023-10-10";

    let mut hp = HistoricalPriceFull::new(&symbol);
    hp.download_interval(&api_key, from, to)?;

    //let mut tr = DailyTreasury::new();
    //tr.download_interval(&api_key, from, to)?;
    //let tr_df = tr.to_dataframe_simple();
    //tr_df.print();
    //let tr_y10 = tr.get_year10_vec();
    //let risk_free = sma(&tr_y10, 120);
    
    let mut tnx = HistoricalPriceFull::new("^TNX");
    tnx.download_interval(&api_key, from, to)?;
    let tnx = tnx.get_close_vec();
    let mut risk_free = tnx.fmap(|x| (1f64 + x / 100f64).powf(1f64 / 252f64) - 1f64);
    risk_free[0 .. 120].fill(0f64);

    let mut df = hp.to_dataframe_simple();
    let open: Vec<f64> = df["open"].to_vec();
    let close: Vec<f64> = df["close"].to_vec();

    let init = open[0];
    let bnh = BuyAndHold::new(init, &close);
    let bnh_cr = bnh.cumulative_return();
    let bnh_vol = bnh.roll_volatility(120);
    let bnh_sr = bnh.roll_sharpe_ratio(&risk_free, 120);
    let bnh_dd = bnh.drawdown();

    df.push("bnh_cr", Series::new(bnh_cr));
    df.push("bnh_vol", Series::new(bnh_vol));
    df.push("bnh_sr", Series::new(bnh_sr));
    df.push("bnh_dd", Series::new(bnh_dd));

    df.print();

    df.write_parquet(&format!("data/{}_bnh.parquet", symbol), CompressionOptions::Uncompressed)?;

    let mut dg = DataFrame::new(vec![]);
    dg.push("CAGR", Series::new(vec![bnh.cagr()]));
    dg.push("Volatility", Series::new(vec![bnh.volatility()]));
    dg.push("Sharpe", Series::new(vec![bnh.sharpe_ratio(&risk_free)]));
    dg.push("MDD", Series::new(vec![bnh.mdd()]));

    dg.print();

    dg.write_parquet(&format!("data/{}_bnh_summary.parquet", symbol), CompressionOptions::Uncompressed)?;

    Ok(())
}

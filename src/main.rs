use peroxide::fuga::*;
use fmp::api::HistoricalPriceFull;
use fmp::ta::{sma, ema, wma, williams_r, rsi, macd, adx_dmi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;

    let symbol = "005930.KS";
    let from = "2022-01-09"; // For cushion
    let to = "2023-10-06";

    let mut samsung_price = HistoricalPriceFull::new(symbol);
    //samsung_price.download_full(&api_key)?;
    samsung_price.download_interval(&api_key, from, to)?;
    let df = samsung_price.to_dataframe_simple();
    let date: Vec<String> = df["date"].to_vec();
    let close: Vec<f64> = df["close"].to_vec();
    let high: Vec<f64> = df["high"].to_vec();
    let low: Vec<f64> = df["low"].to_vec();
    let tp = close.iter().zip(high.iter()).zip(low.iter()).map(|((c, h), l)| (c + h + l) / 3f64).collect::<Vec<f64>>();
    let sma_ = sma(&tp, 20);
    let ema_ = ema(&tp, 20);
    let wma_ = wma(&tp, 20);
    let williams_ = williams_r(&high, &low, &close, 14);
    let rsi_ = rsi(&close, 14);
    let rsi_signal = ema(&rsi_, 9);
    let macd_ = macd(&close, 12, 26);
    let macd_signal = ema(&macd_, 9);
    let (adx_, di_plus, di_minus) = adx_dmi(&high, &low, &close, 14);

    // Result
    let date = date[240..].to_vec();
    let tp = tp[240..].to_vec();
    let sma_ = sma_[240..].to_vec();
    let ema_ = ema_[240..].to_vec();
    let wma_ = wma_[240..].to_vec();
    let williams_ = williams_[240..].to_vec();
    let rsi_ = rsi_[240..].to_vec();
    let rsi_signal = rsi_signal[240..].to_vec();
    let macd_ = macd_[240..].to_vec();
    let macd_signal = macd_signal[240..].to_vec();
    let adx_ = adx_[240..].to_vec();
    let di_plus = di_plus[240..].to_vec();
    let di_minus = di_minus[240..].to_vec();

    let mut dg = DataFrame::new(vec![]);
    dg.push("date", Series::new(date));
    dg.push("tp", Series::new(tp));
    dg.push("sma", Series::new(sma_));
    dg.push("ema", Series::new(ema_));
    dg.push("wma", Series::new(wma_));
    dg.push("williams", Series::new(williams_));
    dg.push("rsi", Series::new(rsi_));
    dg.push("rsi_signal", Series::new(rsi_signal));
    dg.push("macd", Series::new(macd_));
    dg.push("macd_signal", Series::new(macd_signal));
    dg.push("adx", Series::new(adx_));
    dg.push("di_plus", Series::new(di_plus));
    dg.push("di_minus", Series::new(di_minus));

    dg.print();

    dg.write_parquet("./data/005930.KS.parquet", CompressionOptions::Uncompressed)?;

    //let mut samsung_rsi = DailyRSI::new(symbol);
    //samsung_rsi.download_interval(&api_key, from, to)?;
    //let df = samsung_rsi.to_dataframe();
    //let date: Vec<String> = df["date"].to_vec();
    //let close: Vec<f64> = df["close"].to_vec();
    //let rsi: Vec<f64> = df["rsi"].to_vec();

    //let date = date[240..].to_vec();
    //let close = close[240..].to_vec();
    //let rsi = rsi[240..].to_vec();

    //let mut df = DataFrame::new(vec![]);
    //df.push("date", Series::new(date));
    //df.push("close", Series::new(close));
    //df.push("rsi", Series::new(rsi));
    //df.print();

    //df.write_parquet("./data/005930.KS.rsi.parquet", CompressionOptions::Uncompressed)?;

    Ok(())
}

use peroxide::fuga::*;
use fmp::api::HistoricalPriceFull;
use fmp::ta::{sma, ema, wma, dema, tema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;

    let symbol = "005930.KS";
    let from = "2021-01-01";
    let to = "2022-12-31";

    let mut samsung_price = HistoricalPriceFull::new(symbol);
    //samsung_price.download_full(&api_key)?;
    samsung_price.download_interval(&api_key, from, to)?;
    let mut df = samsung_price.to_dataframe_simple();
    let close: Vec<f64> = df["close"].to_vec();
    let high: Vec<f64> = df["high"].to_vec();
    let low: Vec<f64> = df["low"].to_vec();
    let tp = close.iter().zip(high.iter()).zip(low.iter()).map(|((c, h), l)| (c + h + l) / 3f64).collect::<Vec<f64>>();
    let sma = sma(&tp, 20);
    let ema = ema(&tp, 20);
    let wma = wma(&tp, 20);
    let dema = dema(&tp, 20);
    let tema = tema(&tp, 20);

    df.push("tp", Series::new(tp));
    df.push("sma", Series::new(sma));
    df.push("ema", Series::new(ema));
    df.push("wma", Series::new(wma));
    df.push("dema", Series::new(dema));
    df.push("tema", Series::new(tema));

    df.print();

    df.write_parquet("./data/005930.KS.parquet", CompressionOptions::Uncompressed)?;

    Ok(())
}

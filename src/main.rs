use peroxide::fuga::*;
use fmp::api::HistoricalPriceFull;
use fmp::ta::{sma, ema, wma, dema, tema, williams_r};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;

    let symbol = "005930.KS";
    let from = "2023-01-01";
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
    let sma = sma(&tp, 20);
    let ema = ema(&tp, 20);
    let wma = wma(&tp, 20);
    let dema = dema(&tp, 20);
    let tema = tema(&tp, 20);
    let williams = williams_r(&high, &low, &close, 14);

    let mut dg = DataFrame::new(vec![]);
    dg.push("date", Series::new(date));
    dg.push("tp", Series::new(tp));
    dg.push("sma", Series::new(sma));
    dg.push("ema", Series::new(ema));
    dg.push("wma", Series::new(wma));
    dg.push("dema", Series::new(dema));
    dg.push("tema", Series::new(tema));
    dg.push("williams", Series::new(williams));

    dg.print();

    dg.write_parquet("./data/005930.KS.parquet", CompressionOptions::Uncompressed)?;

    Ok(())
}

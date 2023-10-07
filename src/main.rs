use peroxide::fuga::*;
use fmp::api::HistoricalPriceFull;
use fmp::ta::sma;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;

    let symbol = "005930.KS";

    let mut samsung_price = HistoricalPriceFull::new(symbol);
    samsung_price.download_full(&api_key)?;
    let mut df = samsung_price.to_dataframe_simple();
    let close: Vec<f64> = df["close"].to_vec();
    let high: Vec<f64> = df["high"].to_vec();
    let low: Vec<f64> = df["low"].to_vec();
    let tp = close.iter().zip(high.iter()).zip(low.iter()).map(|((c, h), l)| (c + h + l) / 3f64).collect::<Vec<f64>>();
    let sma = sma(&tp, 20);

    df.push("tp", Series::new(tp));
    df.push("sma", Series::new(sma));

    df.print();

    df.write_parquet("./data/005930.KS.parquet", CompressionOptions::Uncompressed)?;

    Ok(())
}

use serde::Deserialize;
use peroxide::fuga::*;
use reqwest::header::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_dir = "./api_key.txt";
    let api_key = std::fs::read_to_string(api_key_dir)?;
    let end_point = format!("&apikey={}", api_key);

    let base_url = "https://financialmodelingprep.com/api/v3/";

    let ticker = "005930.KS";
    let indicator = "historical-price-full";
    let from = "2018-03-12";
    let to = "2023-10-06";
    let url = format!("{}{}/{}?from={}&to={}{}", base_url, indicator, ticker, from, to, end_point);

    let mut headers = HeaderMap::new();
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let resp = client.get(&url).send()?.json::<HistoricalPriceFull>()?;
    let df = resp.to_dataframe();

    df.print();

    Ok(())
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct HistoricalPrice {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adjClose: f64,
    volume: f64,
    unadjustedVolume: f64,
    change: f64,
    changePercent: f64,
    vwap: f64,
    label: String,
    changeOverTime: f64,
}

#[derive(Debug, Deserialize)]
struct HistoricalPriceFull {
    symbol: String,
    historical: Vec<HistoricalPrice>,
}

impl HistoricalPriceFull {
    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_historical(&self) -> &Vec<HistoricalPrice> {
        &self.historical
    }

    pub fn get_date_vec(&self) -> Vec<String> {
        self.historical.iter().map(|x| x.date.clone()).rev().collect()
    }

    pub fn get_open_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.open).rev().collect()
    }

    pub fn get_high_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.high).rev().collect()
    }

    pub fn get_low_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.low).rev().collect()
    }

    pub fn get_close_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.close).rev().collect()
    }

    pub fn get_adj_close_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.adjClose).rev().collect()
    }

    pub fn get_volume_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.volume).rev().collect()
    }

    pub fn get_unadjusted_volume_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.unadjustedVolume).rev().collect()
    }

    pub fn get_change_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.change).rev().collect()
    }

    pub fn get_change_percent_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.changePercent).rev().collect()
    }

    pub fn get_vwap_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.vwap).rev().collect()
    }

    pub fn get_label_vec(&self) -> Vec<String> {
        self.historical.iter().map(|x| x.label.clone()).rev().collect()
    }

    pub fn get_change_over_time_vec(&self) -> Vec<f64> {
        self.historical.iter().map(|x| x.changeOverTime).rev().collect()
    }

    pub fn to_dataframe(&self) -> DataFrame {
        let mut df = DataFrame::new(vec![]);
        df.push("date", Series::new(self.get_date_vec()));
        df.push("open", Series::new(self.get_open_vec()));
        df.push("high", Series::new(self.get_high_vec()));
        df.push("low", Series::new(self.get_low_vec()));
        df.push("close", Series::new(self.get_close_vec()));
        df.push("adjClose", Series::new(self.get_adj_close_vec()));
        df.push("volume", Series::new(self.get_volume_vec()));
        df.push("unadjustedVolume", Series::new(self.get_unadjusted_volume_vec()));
        df.push("change", Series::new(self.get_change_vec()));
        df.push("changePercent", Series::new(self.get_change_percent_vec()));
        df.push("vwap", Series::new(self.get_vwap_vec()));
        df.push("label", Series::new(self.get_label_vec()));
        df.push("changeOverTime", Series::new(self.get_change_over_time_vec()));
        df
    }
}


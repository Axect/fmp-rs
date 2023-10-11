use peroxide::fuga::*;
use std::collections::{HashMap, HashSet};
use crate::api::{download_stocks, HistoricalStockList, HistoricalPrice, download_risk_free};

#[derive(Debug, Clone)]
pub struct Portfolio {
    pub balance: f64,
    pub symbols: Vec<String>,
    pub shares: Vec<usize>,
}

impl Portfolio {
    pub fn new(balance: f64, symbols: &[String]) -> Self {
        let n = symbols.len();
        Self {
            balance,
            symbols: symbols.to_vec(),
            shares: vec![0; n],
        }
    }

    pub fn get_share(&self, symbol: &str) -> Option<usize> {
        self.symbols.iter().position(|x| x == symbol)
    }
}

pub trait Strategy {
    fn gen_order_map(&mut self, timestamp: usize, chart_map: &HashMap<String, Chart>, portfolio: &Portfolio) -> HashMap<String, Order>;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub shares: isize, // positive for buy, negative for sell
}

impl Order {
    pub fn new(symbol: &str, shares: isize) -> Self {
        Self {
            symbol: symbol.to_string(),
            shares,
        }
    }
}

pub struct Backtester {
    pub portfolio: Portfolio,
    pub strategy: Box<dyn Strategy>,
    pub market_data: MarketData,
}

pub struct BacktestReport {
    pub portfolio: Portfolio,
    pub strategy: String,
    pub market_data: MarketData,
    pub daily_return: Vec<f64>,
    pub cumulative_return: Vec<f64>,
    pub rolling_volatility: Vec<f64>,
    pub rolling_sharpe_ratio: Vec<f64>,
    pub drawdown: Vec<f64>,
    pub cagr: f64,
    pub volatility: f64,
    pub sharpe_ratio: f64,
    pub mdd: f64,
}


#[derive(Debug, Clone, Copy)]
pub struct Chart {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone)]
pub struct MarketData {
    pub from: String,
    pub to: String,
    pub date: Vec<String>,
    pub chart: Vec<HashMap<String, Chart>>,
    pub risk_free: Vec<f64>,
}

impl MarketData {
    pub fn new(symbols: &[String], from: &str, to: &str, api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let hsl = download_stocks(symbols, from, to, api_key)?;
        let mut hp_map = HashMap::new();
        let mut date_zip = vec![];
        for i in 0 .. symbols.len() {
            let hpf = hsl.get_historical_price_full(i);
            date_zip.push(hpf.get_date_vec().iter().cloned().enumerate().collect::<Vec<(usize, String)>>());
            hp_map.insert(i, hpf.get_historical().clone());
        }
        let (date_risk_free, mut risk_free) = download_risk_free(from, to, api_key);
        let risk_free_zip = date_risk_free.iter().cloned().enumerate().collect::<Vec<(usize, String)>>();

        // Check date
        // - There are some missing dates
        // - Only accept common dates
        // - To do this, we need to make dates to HashSet
        let date_vec = date_zip.iter().map(|x| x.iter().map(|y| y.1.clone()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
        let mut date_set = date_vec[0].iter().cloned().collect::<HashSet<String>>();
        for date_ in date_vec.iter().skip(1) {
            let date_set_2 = date_.iter().cloned().collect::<HashSet<String>>();
            date_set.retain(|x| date_set_2.contains(x));
        }
        let risk_free_set = date_risk_free.iter().cloned().collect::<HashSet<String>>();
        date_set.retain(|x| risk_free_set.contains(x));
        let mut date = date_set.iter().cloned().collect::<Vec<String>>();
        date.sort();

        // Remove data correspond to deleted dates in hp_map
        let idx_vec = date_zip.iter()
            .filter(|&x| date_set.contains(&x[0].1))
            .map(|x| x.iter().map(|y| y.0).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
        for i in 0 .. symbols.len() {
            let hp = hp_map.get_mut(&i).unwrap();
            let idx = idx_vec[i].clone();
            let mut hp_new = vec![];
            for j in idx {
                hp_new.push(hp[j].clone());
            }
            *hp = hp_new;
        }

        let idx_vec = risk_free_zip.iter()
            .filter(|&x| date_set.contains(&x.1))
            .map(|x| x.0)
            .collect::<Vec<usize>>();
        let mut risk_free_new = vec![];
        for i in idx_vec {
            risk_free_new.push(risk_free[i]);
        }
        risk_free = risk_free_new;

        assert_eq!(date.len(), risk_free.len());

        let mut chart = vec![HashMap::new(); risk_free.len()];
        for i in 0 .. risk_free.len() {
            for j in 0 .. symbols.len() {
                let hp = &hp_map.get(&j).as_ref().unwrap()[i];
                chart[i].insert(symbols[j].clone(), Chart {
                    open: hp.get_open(),
                    high: hp.get_high(),
                    low: hp.get_low(),
                    close: hp.get_close(),
                    volume: hp.get_volume()
                });
            }
        }

        Ok(Self {
            from: from.to_string(),
            to: to.to_string(),
            date,
            chart,
            risk_free,
        })
    }

    pub fn len(&self) -> usize {
        self.chart.len()
    }
}

pub struct BuyAndHold {
    pub weight: Vec<f64>,
    pub rebalance: usize,
    bought: bool,
}

impl BuyAndHold {
    pub fn new(weight: &[f64], rebalance: usize) -> Self {
        Self {
            weight: weight.to_vec(),
            rebalance,
            bought: false,
        }
    }
}

impl Strategy for BuyAndHold {
    fn gen_order_map(&mut self, timestamp: usize, chart_map: &HashMap<String, Chart>, portfolio: &Portfolio) -> HashMap<String, Order> {
        let weight_sum = self.weight.sum();
        let weight = self.weight.fmap(|x| x / weight_sum);

        let mut order_map = HashMap::new();
        for symbol in portfolio.symbols.iter() {
            order_map.insert(symbol.to_string(), Order::new(symbol, 0));
        }

        if timestamp % self.rebalance == 0 {
            if self.bought {
                // Closing all positions
                for symbol in portfolio.symbols.iter() {
                    let current_share = portfolio.get_share(symbol).unwrap();
                    order_map.insert(symbol.to_string(), Order::new(symbol, -(current_share as isize)));
                }
                self.bought = false;
            }
        } else if timestamp % self.rebalance == 1 {
            // Opening
            let current_balance = portfolio.balance;
            for (w, symbol) in weight.iter().zip(portfolio.symbols.iter()) {
                let current_price = chart_map.get(symbol).as_ref().unwrap().close;
                let shares = (current_balance * w / current_price) as isize;
                order_map.insert(symbol.to_string(), Order::new(symbol, shares));
            }
            self.bought = true;
        }
        order_map
    }

    fn to_string(&self) -> String {
        format!("BnH")
    }
}

impl Backtester {
    pub fn new(symbols: &[String], init_balance: f64, strategy: Box<dyn Strategy>, from: &str, to: &str, api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let market_data = MarketData::new(symbols, from, to, api_key)?;
        let portfolio = Portfolio::new(init_balance, symbols);
        Ok(Self {
            portfolio,
            strategy,
            market_data,
        })
    }

    pub fn get_symbols(&self) -> &[String] {
        &self.portfolio.symbols
    }

    pub fn get_balance(&self) -> f64 {
        self.portfolio.balance
    }

    pub fn get_shares(&self) -> &[usize] {
        &self.portfolio.shares
    }

    pub fn get_chart(&self, timestamp: usize) -> &HashMap<String, Chart> {
        &self.market_data.chart[timestamp]
    }

    pub fn get_risk_free(&self) -> &Vec<f64> {
        &self.market_data.risk_free
    }

    pub fn execute_order(&mut self, order: &HashMap<String, Order>, timestamp: usize) {
        let chart = self.get_chart(timestamp).clone();
        let shares = self.get_shares().to_vec();
        let balance = self.get_balance();
        for (i, (symbol, o)) in order.iter().enumerate() {
            let price = chart.get(symbol).as_ref().unwrap().open;
            let current_share = &shares[i];
            let order_share = o.shares;
            let cost = price * (order_share as f64);
            self.portfolio.balance = balance - cost;
            self.portfolio.shares[i] = (*current_share as isize + order_share) as usize;
        }
    }

    pub fn obtain_value(&self, timestamp: usize) -> f64 {
        let chart = self.get_chart(timestamp - 1);
        let shares = self.get_shares();
        let mut value = self.get_balance();
        for (i, symbol) in self.get_symbols().iter().enumerate() {
            let price = chart.get(symbol).as_ref().unwrap().close;
            value += price * (shares[i] as f64);
        }
        value
    }

    pub fn run(&mut self, rolling_window: usize) -> BacktestReport {
        let mut timestamp = 1usize;
        let mut daily_return = vec![0f64; self.market_data.len()];

        let mut total_value = self.obtain_value(timestamp);
        while timestamp <= self.market_data.len() {
            let idx = timestamp - 1;
            let chart_map = self.get_chart(idx).clone();
            let order_map = self.strategy.gen_order_map(idx, &chart_map, &self.portfolio);
            self.execute_order(&order_map, idx);
            let new_value = self.obtain_value(timestamp);
            daily_return[idx] = (new_value - total_value) / total_value;
            total_value = new_value;
            timestamp += 1;
        }

        let cumulative_return = daily_return.iter()
            .scan(1f64, |state, x| {
                *state *= 1f64 + x;
                Some(*state)
            })
            .collect::<Vec<f64>>();

        let mut rolling_volatility = vec![0f64; daily_return.len()];
        for i in rolling_window .. daily_return.len() {
            rolling_volatility[i] = daily_return[i - rolling_window .. i].to_vec().sd() * (252 as f64).sqrt();
        }

        let mut rolling_sharpe_ratio = vec![0f64; daily_return.len()];
        let risk_free = self.get_risk_free();
        for i in rolling_window .. daily_return.len() {
            let dr_roll = daily_return[i - rolling_window .. i].to_vec();
            let rf = risk_free[i - rolling_window .. i].to_vec();
            let excess_return = dr_roll.sub_v(&rf);
            rolling_sharpe_ratio[i] = excess_return.mean() / excess_return.sd() * (252 as f64).sqrt();
        }

        let mut drawdown = vec![0f64; daily_return.len()];
        let mut max = 0f64;
        for i in 0..daily_return.len() {
            if cumulative_return[i] > max {
                max = cumulative_return[i];
            } else {
                drawdown[i] = (max - cumulative_return[i]) / max;
            }
        }

        let cagr = cumulative_return.last().unwrap().powf(1f64 / (daily_return.len() as f64 / 252f64)) - 1f64;
        let volatility = daily_return.sd() * (daily_return.len() as f64).sqrt();
        let sharpe_ratio = {
            let excess_return = daily_return.sub_v(&risk_free);
            excess_return.mean() / excess_return.sd() * (252 as f64).sqrt()
        };
        let mdd = drawdown.max();

        BacktestReport {
            portfolio: self.portfolio.clone(),
            strategy: self.strategy.to_string(),
            market_data: self.market_data.clone(),
            daily_return,
            cumulative_return,
            rolling_volatility,
            rolling_sharpe_ratio,
            drawdown,
            cagr,
            volatility,
            sharpe_ratio,
            mdd,
        }
    }
}

impl BacktestReport {
    pub fn get_daily_return(&self) -> &Vec<f64> {
        &self.daily_return
    }

    pub fn get_cumulative_return(&self) -> &Vec<f64> {
        &self.cumulative_return
    }

    pub fn get_rolling_volatility(&self) -> &Vec<f64> {
        &self.rolling_volatility
    }

    pub fn get_rolling_sharpe_ratio(&self) -> &Vec<f64> {
        &self.rolling_sharpe_ratio
    }

    pub fn get_drawdown(&self) -> &Vec<f64> {
        &self.drawdown
    }

    pub fn get_cagr(&self) -> f64 {
        self.cagr
    }

    pub fn get_volatility(&self) -> f64 {
        self.volatility
    }

    pub fn get_sharpe_ratio(&self) -> f64 {
        self.sharpe_ratio
    }

    pub fn get_mdd(&self) -> f64 {
        self.mdd
    }

    pub fn to_dataframe(&self) -> DataFrame {
        let mut df = DataFrame::new(vec![]);
        df.push("date", Series::new(self.market_data.date.clone()));
        df.push("daily_return", Series::new(self.daily_return.clone()));
        df.push("cumulative_return", Series::new(self.cumulative_return.clone()));
        df.push("rolling_volatility", Series::new(self.rolling_volatility.clone()));
        df.push("rolling_sharpe_ratio", Series::new(self.rolling_sharpe_ratio.clone()));
        df.push("drawdown", Series::new(self.drawdown.clone()));
        df
    }

    pub fn to_report(&self) -> DataFrame {
        let mut dg = DataFrame::new(vec![]);
        dg.push("Strategy", 
            Series::new(
                vec![
                    self.strategy.clone()
                ]
            )
        );
        dg.push("CAGR", Series::new(vec![self.cagr]));
        dg.push("Volatility", Series::new(vec![self.volatility]));
        dg.push("Sharpe", Series::new(vec![self.sharpe_ratio]));
        dg.push("MDD", Series::new(vec![self.mdd]));
        dg
    }
}

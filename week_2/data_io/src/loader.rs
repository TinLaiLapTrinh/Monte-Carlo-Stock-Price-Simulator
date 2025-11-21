use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "Ticker")]
    pub ticker: String,

    #[serde(rename = "DTYYYYMMDD", deserialize_with = "parse_date")]
    pub date: NaiveDate,

    #[serde(rename = "Open")]
    pub open: f64,

    #[serde(rename = "High")]
    pub high: f64,

    #[serde(rename = "Low")]
    pub low: f64,

    #[serde(rename = "Close")]
    pub close: f64,

    #[serde(rename = "Volume")]
    pub volume: u64,
}

fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y%m%d")
        .map_err(serde::de::Error::custom)
}

pub fn load_csv_by_ticker(path: &str, ticker: &str) -> anyhow::Result<Vec<Record>> {
    let mut rdr = csv::Reader::from_path(path)?;

    let mut records: Vec<Record> = rdr.deserialize()
        .filter_map(|r| r.ok())
        .filter(|r: &Record| r.ticker == ticker)
        .collect();

    // Sắp xếp theo ngày tăng dần
    records.sort_by_key(|r| r.date);

    Ok(records)
}


pub fn load_all_tickers(path: &str) -> anyhow::Result<Vec<String>> {
    let mut rdr = csv::Reader::from_path(path)?;

    let mut tickers = std::collections::HashSet::new();

    for result in rdr.deserialize::<Record>() {
        if let Ok(rec) = result {
            tickers.insert(rec.ticker);
        }
    }

    let mut list: Vec<String> = tickers.into_iter().collect();
    list.sort();
    Ok(list)
}

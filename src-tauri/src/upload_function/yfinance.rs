use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use super::local_upload::{SymbolData, TimeframeInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct YFinanceRequest {
    pub symbol: String,
    pub save_as: String,
    pub start_date: String,
    pub end_date: String,
    pub timeframes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct YFinanceResult {
    pub success: bool,
    pub message: String,
    pub symbol: String,
    pub timeframes_downloaded: Vec<String>,
    pub total_candles: usize,
}

// Map yfinance intervals to your timeframe format
fn map_timeframe_to_yfinance(tf: &str) -> Option<String> {
    match tf {
        "M1" => Some("1m".to_string()),
        "M2" => Some("2m".to_string()),
        "M5" => Some("5m".to_string()),
        "M15" => Some("15m".to_string()),
        "M30" => Some("30m".to_string()),
        "H1" => Some("1h".to_string()),
        "H4" => Some("4h".to_string()),
        "D1" => Some("1d".to_string()),
        "W1" => Some("1wk".to_string()),
        "MN1" => Some("1mo".to_string()),
        _ => None,
    }
}

fn convert_timeframe_to_display(timeframe: &str) -> String {
    match timeframe {
        "M1" => "1 min".to_string(),
        "M2" => "2 min".to_string(),
        "M5" => "5 min".to_string(),
        "M15" => "15 min".to_string(),
        "M30" => "30 min".to_string(),
        "H1" => "1 hour".to_string(),
        "H4" => "4 hours".to_string(),
        "D1" => "1 day".to_string(),
        "W1" => "1 week".to_string(),
        "MN1" => "1 month".to_string(),
        _ => timeframe.to_string(),
    }
}

async fn fetch_yfinance_data(
    symbol: &str,
    interval: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<(i64, f64, f64, f64, f64, i64)>, String> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval={}&period1={}&period2={}",
        symbol, interval, start_date, end_date
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API returned error: {}", response.status()));
    }

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Extract data from the response
    let chart = json["chart"]["result"][0]
        .as_object()
        .ok_or("Invalid response structure")?;

    let timestamps = chart["timestamp"]
        .as_array()
        .ok_or("No timestamp data")?
        .iter()
        .filter_map(|v| v.as_i64())
        .collect::<Vec<i64>>();

    let indicators = &chart["indicators"]["quote"][0];
    let opens = indicators["open"]
        .as_array()
        .ok_or("No open data")?
        .iter()
        .map(|v| v.as_f64())
        .collect::<Vec<Option<f64>>>();
    let highs = indicators["high"]
        .as_array()
        .ok_or("No high data")?
        .iter()
        .map(|v| v.as_f64())
        .collect::<Vec<Option<f64>>>();
    let lows = indicators["low"]
        .as_array()
        .ok_or("No low data")?
        .iter()
        .map(|v| v.as_f64())
        .collect::<Vec<Option<f64>>>();
    let closes = indicators["close"]
        .as_array()
        .ok_or("No close data")?
        .iter()
        .map(|v| v.as_f64())
        .collect::<Vec<Option<f64>>>();
    let volumes = indicators["volume"]
        .as_array()
        .ok_or("No volume data")?
        .iter()
        .map(|v| v.as_i64())
        .collect::<Vec<Option<i64>>>();

    let mut candles = Vec::new();

    for i in 0..timestamps.len() {
        if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) =
            (opens[i], highs[i], lows[i], closes[i], volumes[i])
        {
            candles.push((timestamps[i], open, high, low, close, volume));
        }
    }

    if candles.is_empty() {
        return Err("No valid candle data found".to_string());
    }

    Ok(candles)
}

fn save_candles_to_csv(
    file_path: &PathBuf,
    candles: &[(i64, f64, f64, f64, f64, i64)],
) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;

    // Write header
    writeln!(file, "timestamp,open,high,low,close,volume")
        .map_err(|e| format!("Failed to write header: {}", e))?;

    // Write data
    for (timestamp, open, high, low, close, volume) in candles {
        writeln!(
            file,
            "{},{},{},{},{},{}",
            timestamp, open, high, low, close, volume
        )
        .map_err(|e| format!("Failed to write data: {}", e))?;
    }

    Ok(())
}

fn convert_date_to_timestamp(date_str: &str) -> Result<String, String> {
    use chrono::NaiveDate;

    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))?;

    let datetime = date
        .and_hms_opt(0, 0, 0)
        .ok_or("Failed to create datetime")?;
    let timestamp = datetime.and_utc().timestamp();

    Ok(timestamp.to_string())
}

#[tauri::command]
pub async fn fetch_yfinance_data_command(
    app_handle: AppHandle,
    request: YFinanceRequest,
) -> Result<YFinanceResult, String> {
    let save_as = request.save_as.to_uppercase();

    // Validate symbol name
    if save_as.is_empty() {
        return Err("Symbol name cannot be empty".to_string());
    }

    if save_as.contains(|c: char| !c.is_alphanumeric() && c != '_') {
        return Err("Symbol name can only contain letters, numbers, and underscores".to_string());
    }

    // Check if symbol already exists
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let symbols_dir = app_dir.join("symbols");
    let symbol_json = symbols_dir.join(format!("{}.json", save_as));

    if symbol_json.exists() {
        return Err(format!(
            "Symbol '{}' already exists. Please delete it first or choose a different name.",
            save_as
        ));
    }

    // Create temporary directory for CSV files
    let temp_dir = app_dir.join("temp").join(&save_as);
    fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp directory: {}", e))?;

    // Convert dates to timestamps
    let start_timestamp = convert_date_to_timestamp(&request.start_date)?;
    let end_timestamp = convert_date_to_timestamp(&request.end_date)?;

    let mut timeframe_infos = Vec::new();
    let mut total_candles = 0;
    let mut downloaded_timeframes = Vec::new();

    // Download data for each timeframe
    for tf in &request.timeframes {
        let yf_interval =
            map_timeframe_to_yfinance(tf).ok_or(format!("Unsupported timeframe: {}", tf))?;

        println!("Fetching {} data for {}...", tf, request.symbol);

        match fetch_yfinance_data(
            &request.symbol,
            &yf_interval,
            &start_timestamp,
            &end_timestamp,
        )
        .await
        {
            Ok(candles) => {
                let csv_path = temp_dir.join(format!("{}.csv", tf));
                save_candles_to_csv(&csv_path, &candles)?;

                let display_name = convert_timeframe_to_display(tf);
                let candle_count = candles.len();

                timeframe_infos.push(TimeframeInfo {
                    name: tf.clone(),
                    display_name,
                    candle_count,
                    file_path: csv_path.to_string_lossy().to_string(),
                });

                total_candles += candle_count;
                downloaded_timeframes.push(tf.clone());

                println!("✓ {} - {} candles downloaded", tf, candle_count);
            }
            Err(e) => {
                println!("✗ {} - Failed: {}", tf, e);
                // Continue with other timeframes
            }
        }
    }

    if timeframe_infos.is_empty() {
        // Clean up temp directory
        let _ = fs::remove_dir_all(&temp_dir);
        return Err("Failed to download any timeframe data".to_string());
    }

    // Sort timeframes
    timeframe_infos.sort_by(|a, b| {
        let order = [
            "M1", "M2", "M5", "M15", "M30", "H1", "H4", "D1", "W1", "MN1",
        ];
        let a_idx = order.iter().position(|&x| x == a.name).unwrap_or(999);
        let b_idx = order.iter().position(|&x| x == b.name).unwrap_or(999);
        a_idx.cmp(&b_idx)
    });

    // Save symbol data
    let symbol_data = SymbolData {
        symbol: save_as.clone(),
        timeframes: timeframe_infos,
        total_candles,
        uploaded_at: chrono::Utc::now().to_rfc3339(),
    };

    save_symbol_data(&app_handle, &symbol_data)?;

    // Clean up temp directory
    let _ = fs::remove_dir_all(&temp_dir);

    Ok(YFinanceResult {
        success: true,
        message: format!(
            "Successfully downloaded {} timeframe{} with {} total candles",
            downloaded_timeframes.len(),
            if downloaded_timeframes.len() == 1 {
                ""
            } else {
                "s"
            },
            total_candles
        ),
        symbol: save_as,
        timeframes_downloaded: downloaded_timeframes,
        total_candles,
    })
}

fn save_symbol_data(app_handle: &AppHandle, symbol_data: &SymbolData) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let symbols_dir = app_dir.join("symbols");
    fs::create_dir_all(&symbols_dir)
        .map_err(|e| format!("Failed to create symbols directory: {}", e))?;

    let json_path = symbols_dir.join(format!("{}.json", symbol_data.symbol));
    let json_content = serde_json::to_string_pretty(symbol_data)
        .map_err(|e| format!("Failed to serialize symbol data: {}", e))?;

    let mut file =
        File::create(json_path).map_err(|e| format!("Failed to create JSON file: {}", e))?;

    file.write_all(json_content.as_bytes())
        .map_err(|e| format!("Failed to write JSON file: {}", e))?;

    Ok(())
}

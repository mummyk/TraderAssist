use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeframeInfo {
    pub name: String,
    pub display_name: String,
    pub candle_count: usize,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolData {
    pub symbol: String,
    pub timeframes: Vec<TimeframeInfo>,
    pub total_candles: usize,
    pub uploaded_at: String,
}

#[derive(Debug, Serialize)]
pub struct ProcessResult {
    pub success: bool,
    pub message: String,
    pub symbol: Option<String>,
    pub timeframes_processed: Vec<String>,
    pub total_candles: usize,
}

#[tauri::command]
pub async fn process_chart_folder(
    app_handle: AppHandle,
    folder_path: String,
) -> Result<ProcessResult, String> {
    let path = PathBuf::from(&folder_path);

    if !path.exists() {
        return Err("Folder does not exist".to_string());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let symbol_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid folder name")?
        .to_uppercase();

    let mut timeframes: Vec<TimeframeInfo> = Vec::new();
    let mut total_candles = 0;

    let entries = fs::read_dir(&path).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let file_path = entry.path();

        if let Some(extension) = file_path.extension() {
            if extension == "csv" {
                let timeframe = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or("Invalid file name")?
                    .to_uppercase();

                match count_and_validate_csv(&file_path) {
                    Ok(count) => {
                        let display_name = convert_timeframe_to_display(&timeframe);

                        timeframes.push(TimeframeInfo {
                            name: timeframe.clone(),
                            display_name,
                            candle_count: count,
                            file_path: file_path.to_string_lossy().to_string(),
                        });

                        total_candles += count;
                    }
                    Err(e) => {
                        return Err(format!("Error in {}.csv: {}", timeframe, e));
                    }
                }
            }
        }
    }

    if timeframes.is_empty() {
        return Err("No valid CSV files found in the folder".to_string());
    }

    timeframes.sort_by(|a, b| {
        let order = [
            "M1", "M2", "M3", "M4", "M5", "M10", "M15", "M30", "H1", "H2", "H3", "H4", "D1", "W1",
            "MN1",
        ];
        let a_idx = order.iter().position(|&x| x == a.name).unwrap_or(999);
        let b_idx = order.iter().position(|&x| x == b.name).unwrap_or(999);
        a_idx.cmp(&b_idx)
    });

    let symbol_data = SymbolData {
        symbol: symbol_name.clone(),
        timeframes: timeframes.clone(),
        total_candles,
        uploaded_at: chrono::Utc::now().to_rfc3339(),
    };

    save_symbol_data(&app_handle, &symbol_data)?;

    let timeframes_processed: Vec<String> = timeframes.iter().map(|tf| tf.name.clone()).collect();

    Ok(ProcessResult {
        success: true,
        message: format!(
            "Successfully processed {} timeframe{} with {} total candles",
            timeframes_processed.len(),
            if timeframes_processed.len() == 1 {
                ""
            } else {
                "s"
            },
            total_candles
        ),
        symbol: Some(symbol_name),
        timeframes_processed,
        total_candles,
    })
}

fn convert_timeframe_to_display(timeframe: &str) -> String {
    match timeframe {
        "M1" => "1 min".to_string(),
        "M2" => "2 min".to_string(),
        "M3" => "3 min".to_string(),
        "M4" => "4 min".to_string(),
        "M5" => "5 min".to_string(),
        "M10" => "10 min".to_string(),
        "M15" => "15 min".to_string(),
        "M30" => "30 min".to_string(),
        "H1" => "1 hour".to_string(),
        "H2" => "2 hour".to_string(),
        "H3" => "3 hour".to_string(),
        "H4" => "4 hours".to_string(),
        "D1" => "1 day".to_string(),
        "W1" => "1 week".to_string(),
        "MN1" => "1 month".to_string(),
        _ => timeframe.to_string(),
    }
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

#[tauri::command]
pub async fn get_available_symbols(app_handle: AppHandle) -> Result<Vec<SymbolData>, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let symbols_dir = app_dir.join("symbols");

    if !symbols_dir.exists() {
        return Ok(Vec::new());
    }

    let mut symbols = Vec::new();

    let entries = fs::read_dir(&symbols_dir)
        .map_err(|e| format!("Failed to read symbols directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read symbol file: {}", e))?;

            let symbol_data: SymbolData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse symbol data: {}", e))?;

            symbols.push(symbol_data);
        }
    }

    symbols.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    Ok(symbols)
}

#[tauri::command]
pub async fn get_symbol_data(app_handle: AppHandle, symbol: String) -> Result<SymbolData, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let json_path = app_dir.join("symbols").join(format!("{}.json", symbol));

    if !json_path.exists() {
        return Err(format!("Symbol {} not found", symbol));
    }

    let content =
        fs::read_to_string(&json_path).map_err(|e| format!("Failed to read symbol file: {}", e))?;

    let symbol_data: SymbolData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse symbol data: {}", e))?;

    Ok(symbol_data)
}

fn is_valid_date_format(s: &str) -> bool {
    // Check if it matches YYYY.MM.DD or YYYY.MM.DD HH:MM:SS format
    s.contains('.') && (s.len() >= 10)
}

fn count_and_validate_csv(file_path: &PathBuf) -> Result<usize, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    if content.trim().is_empty() {
        return Err("File is empty".to_string());
    }

    // Detect delimiter - check for tabs first (most common in your data), then comma, then semicolon
    let first_line = content.lines().next().unwrap_or("");
    let delimiter = if first_line.matches('\t').count() >= 4 {
        b'\t'
    } else if first_line.matches(',').count() >= 4 {
        b','
    } else if first_line.matches(';').count() >= 4 {
        b';'
    } else if first_line.matches(' ').count() >= 7 {
        // Multiple spaces might be used as delimiter
        b' '
    } else {
        return Err(
            "Could not detect CSV delimiter (expected tab, comma, or semicolon)".to_string(),
        );
    };

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_path(file_path)
        .map_err(|e| format!("Failed to read CSV: {}", e))?;

    // FIX: Clone the headers to release the mutable borrow on 'rdr'
    let headers = rdr
        .headers()
        .map_err(|e| format!("Failed to read headers: {}", e))?
        .clone();

    if headers.len() < 5 {
        return Err(format!(
            "Invalid CSV format: Expected at least 5 columns, found {} columns in header. Your columns: {}",
            headers.len(),
            headers.iter().collect::<Vec<_>>().join(", ")
        ));
    }

    let mut count = 0;

    // Now 'rdr' is free to be borrowed again here
    for (index, result) in rdr.records().enumerate() {
        let record = result.map_err(|e| format!("Row {}: {}", index + 2, e))?;

        // Skip empty rows
        if record.iter().all(|field| field.trim().is_empty()) {
            continue;
        }

        if record.len() < 5 {
            return Err(format!(
                "Row {}: Expected at least 5 columns, found {}",
                index + 2,
                record.len()
            ));
        }

        // Validate first data row
        if count == 0 {
            // Column 0: Check if it's a date string or timestamp
            let first_col = record.get(0).ok_or("Missing first column")?;

            // Accept either date format (YYYY.MM.DD) or Unix timestamp
            if !is_valid_date_format(first_col) {
                // Try parsing as number
                first_col.parse::<i64>().map_err(|_| {
                    format!(
                        "Row {}: Invalid date/timestamp '{}'. Expected format: YYYY.MM.DD or Unix timestamp",
                        index + 2,
                        first_col
                    )
                })?;
            }

            // Columns 1-4 (or appropriate OHLC columns): Validate prices
            let price_cols = if headers.len() >= 8 {
                // Format: DATE, TIME, OPEN, HIGH, LOW, CLOSE, TICKVOL, VOL, SPREAD
                // So OPEN is at index 2
                2..=5
            } else {
                // Format: DATE, OPEN, HIGH, LOW, CLOSE
                1..=4
            };

            for col in price_cols {
                if col < record.len() {
                    let value_str = record
                        .get(col)
                        .ok_or(format!("Missing column {}", col + 1))?;
                    value_str.parse::<f64>().map_err(|_| {
                        format!(
                            "Row {}: Invalid price '{}' in column {}",
                            index + 2,
                            value_str,
                            col + 1
                        )
                    })?;
                }
            }
        }

        count += 1;
    }

    if count == 0 {
        return Err("No valid data rows found in CSV".to_string());
    }

    Ok(count)
}

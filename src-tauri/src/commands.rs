use csv::ReaderBuilder;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct ProcessResult {
    pub success: bool,
    pub message: String,
    pub symbol: Option<String>,
    pub timeframes_processed: Vec<String>,
    pub total_candles: usize,
}

#[tauri::command]
pub async fn process_chart_folder(folder_path: String) -> Result<ProcessResult, String> {
    let path = PathBuf::from(&folder_path);

    if !path.exists() {
        return Err("Folder does not exist".to_string());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    // Get symbol name from folder name
    let symbol_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid folder name")?
        .to_string();

    let mut timeframes_processed = Vec::new();
    let mut total_candles = 0;

    // Read all CSV files in the folder
    let entries = fs::read_dir(&path).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let file_path = entry.path();

        // Only process CSV files
        if let Some(extension) = file_path.extension() {
            if extension == "csv" {
                let timeframe = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or("Invalid file name")?
                    .to_string();

                // Validate and count candles in this CSV
                match count_and_validate_csv(&file_path) {
                    Ok(count) => {
                        total_candles += count;
                        timeframes_processed.push(timeframe);
                    }
                    Err(e) => {
                        return Err(format!("Error in {}.csv: {}", timeframe, e));
                    }
                }
            }
        }
    }

    if timeframes_processed.is_empty() {
        return Err("No valid CSV files found in the folder".to_string());
    }

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

fn count_and_validate_csv(file_path: &PathBuf) -> Result<usize, String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .map_err(|e| format!("Failed to read CSV: {}", e))?;

    let mut count = 0;

    // Validate first few rows to ensure proper format
    for (index, result) in rdr.records().enumerate() {
        let record = result.map_err(|e| format!("Row {}: {}", index + 2, e))?;

        // Validate CSV has at least 5 columns (timestamp, open, high, low, close)
        if record.len() < 5 {
            return Err(format!(
                "Row {}: Expected at least 5 columns, found {}",
                index + 2,
                record.len()
            ));
        }

        // Only validate format on first row to avoid performance issues
        if index == 0 {
            // Validate timestamp
            record
                .get(0)
                .and_then(|s| s.parse::<i64>().ok())
                .ok_or(format!("Row {}: Invalid timestamp format", index + 2))?;

            // Validate OHLC values
            for col in 1..=4 {
                record
                    .get(col)
                    .and_then(|s| s.parse::<f64>().ok())
                    .ok_or(format!(
                        "Row {}: Invalid number format in column {}",
                        index + 2,
                        col + 1
                    ))?;
            }
        }

        count += 1;
    }

    if count == 0 {
        return Err("CSV file is empty".to_string());
    }

    Ok(count)
}

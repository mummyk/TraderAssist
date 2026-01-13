use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use tauri::{AppHandle, Manager};

use super::local_upload::{SymbolData, TimeframeInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRequest {
    pub repo_url: String,
    pub branch: String,
    pub structure_type: String,      // "multi" or "single"
    pub symbol_name: Option<String>, // For multi-symbol repos, which symbol to download
}

#[derive(Debug, Serialize)]
pub struct GitHubResult {
    pub success: bool,
    pub message: String,
    pub symbols_processed: Vec<String>,
    pub total_timeframes: usize,
}

#[derive(Debug, Deserialize)]
struct GitHubContent {
    name: String,
    path: String,
    #[serde(rename = "type")]
    content_type: String,
    download_url: Option<String>,
}

fn parse_github_url(url: &str) -> Result<(String, String), String> {
    let url = url.trim_end_matches('/');

    // Handle both https://github.com/user/repo and github.com/user/repo
    let parts: Vec<&str> = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("github.com/")
        .split('/')
        .collect();

    if parts.len() < 2 {
        return Err(
            "Invalid GitHub URL format. Expected: https://github.com/username/repo".to_string(),
        );
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
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

async fn fetch_github_contents(
    owner: &str,
    repo: &str,
    path: &str,
    branch: &str,
) -> Result<Vec<GitHubContent>, String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
        owner, repo, path, branch
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "TraderAssist")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch GitHub contents: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "GitHub API error: {}. Make sure the repository is public and the path exists.",
            response.status()
        ));
    }

    let contents: Vec<GitHubContent> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub response: {}", e))?;

    Ok(contents)
}

async fn download_csv_file(url: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "TraderAssist")
        .send()
        .await
        .map_err(|e| format!("Failed to download file: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()));
    }

    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read file content: {}", e))?;

    Ok(content)
}

fn is_valid_timeframe(name: &str) -> bool {
    matches!(
        name.to_uppercase().as_str(),
        "M1" | "M2"
            | "M3"
            | "M4"
            | "M5"
            | "M10"
            | "M15"
            | "M30"
            | "H1"
            | "H2"
            | "H3"
            | "H4"
            | "D1"
            | "W1"
            | "MN1"
    )
}

fn count_csv_lines(content: &str) -> usize {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
        .saturating_sub(1) // Subtract header
}

async fn process_symbol_folder(
    owner: &str,
    repo: &str,
    symbol_path: &str,
    symbol_name: &str,
    branch: &str,
    app_handle: &AppHandle,
) -> Result<SymbolData, String> {
    let contents = fetch_github_contents(owner, repo, symbol_path, branch).await?;

    let csv_files: Vec<&GitHubContent> = contents
        .iter()
        .filter(|item| item.content_type == "file" && item.name.to_uppercase().ends_with(".CSV"))
        .collect();

    if csv_files.is_empty() {
        return Err(format!("No CSV files found in {}", symbol_path));
    }

    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let temp_dir = app_dir.join("temp").join(symbol_name);
    fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp directory: {}", e))?;

    let mut timeframe_infos = Vec::new();
    let mut total_candles = 0;

    for file in csv_files {
        let timeframe = file
            .name
            .trim_end_matches(".csv")
            .trim_end_matches(".CSV")
            .to_uppercase();

        if !is_valid_timeframe(&timeframe) {
            println!("Skipping invalid timeframe: {}", file.name);
            continue;
        }

        if let Some(download_url) = &file.download_url {
            println!("Downloading {} for {}...", timeframe, symbol_name);

            match download_csv_file(download_url).await {
                Ok(content) => {
                    let csv_path = temp_dir.join(format!("{}.csv", timeframe));
                    let mut f = File::create(&csv_path)
                        .map_err(|e| format!("Failed to create CSV file: {}", e))?;
                    f.write_all(content.as_bytes())
                        .map_err(|e| format!("Failed to write CSV file: {}", e))?;

                    let candle_count = count_csv_lines(&content);
                    let display_name = convert_timeframe_to_display(&timeframe);

                    timeframe_infos.push(TimeframeInfo {
                        name: timeframe.clone(),
                        display_name,
                        candle_count,
                        file_path: csv_path.to_string_lossy().to_string(),
                    });

                    total_candles += candle_count;
                    println!("✓ {} - {} candles", timeframe, candle_count);
                }
                Err(e) => {
                    println!("✗ {} - Failed: {}", timeframe, e);
                }
            }
        }
    }

    if timeframe_infos.is_empty() {
        return Err("No valid timeframe files were downloaded".to_string());
    }

    // Sort timeframes
    timeframe_infos.sort_by(|a, b| {
        let order = [
            "M1", "M2", "M3", "M4", "M5", "M10", "M15", "M30", "H1", "H2", "H3", "H4", "D1", "W1",
            "MN1",
        ];
        let a_idx = order.iter().position(|&x| x == a.name).unwrap_or(999);
        let b_idx = order.iter().position(|&x| x == b.name).unwrap_or(999);
        a_idx.cmp(&b_idx)
    });

    let symbol_data = SymbolData {
        symbol: symbol_name.to_uppercase(),
        timeframes: timeframe_infos,
        total_candles,
        uploaded_at: chrono::Utc::now().to_rfc3339(),
    };

    save_symbol_data(app_handle, &symbol_data)?;

    // Clean up temp directory
    let _ = fs::remove_dir_all(&temp_dir);

    Ok(symbol_data)
}

#[tauri::command]
pub async fn fetch_github_data_command(
    app_handle: AppHandle,
    request: GitHubRequest,
) -> Result<GitHubResult, String> {
    let (owner, repo) = parse_github_url(&request.repo_url)?;
    let branch = if request.branch.is_empty() {
        "main".to_string()
    } else {
        request.branch
    };

    let mut symbols_processed = Vec::new();
    let mut total_timeframes = 0;

    match request.structure_type.as_str() {
        "single" => {
            // Single symbol repo - files are in root
            let symbol_name = request
                .symbol_name
                .unwrap_or_else(|| repo.clone())
                .to_uppercase();

            // Check if symbol already exists
            check_symbol_exists(&app_handle, &symbol_name)?;

            let symbol_data =
                process_symbol_folder(&owner, &repo, "", &symbol_name, &branch, &app_handle)
                    .await?;

            symbols_processed.push(symbol_data.symbol);
            total_timeframes = symbol_data.timeframes.len();
        }
        "multi" => {
            if let Some(symbol_name) = request.symbol_name {
                // Download specific symbol from multi-symbol repo
                let symbol_upper = symbol_name.to_uppercase();
                check_symbol_exists(&app_handle, &symbol_upper)?;

                let symbol_data = process_symbol_folder(
                    &owner,
                    &repo,
                    &symbol_name,
                    &symbol_upper,
                    &branch,
                    &app_handle,
                )
                .await?;

                symbols_processed.push(symbol_data.symbol);
                total_timeframes = symbol_data.timeframes.len();
            } else {
                // Download all symbols from multi-symbol repo
                let contents = fetch_github_contents(&owner, &repo, "", &branch).await?;

                let symbol_folders: Vec<&GitHubContent> = contents
                    .iter()
                    .filter(|item| item.content_type == "dir")
                    .collect();

                if symbol_folders.is_empty() {
                    return Err("No symbol folders found in repository".to_string());
                }

                for folder in symbol_folders {
                    let symbol_name = folder.name.to_uppercase();

                    // Skip if already exists
                    if check_symbol_exists(&app_handle, &symbol_name).is_err() {
                        println!("Skipping {} (already exists)", symbol_name);
                        continue;
                    }

                    match process_symbol_folder(
                        &owner,
                        &repo,
                        &folder.path,
                        &symbol_name,
                        &branch,
                        &app_handle,
                    )
                    .await
                    {
                        Ok(symbol_data) => {
                            symbols_processed.push(symbol_data.symbol);
                            total_timeframes += symbol_data.timeframes.len();
                        }
                        Err(e) => {
                            println!("Failed to process {}: {}", symbol_name, e);
                        }
                    }
                }

                if symbols_processed.is_empty() {
                    return Err("No symbols were successfully downloaded".to_string());
                }
            }
        }
        _ => {
            return Err("Invalid structure type. Use 'single' or 'multi'".to_string());
        }
    }

    Ok(GitHubResult {
        success: true,
        message: format!(
            "Successfully downloaded {} symbol{} with {} total timeframe{}",
            symbols_processed.len(),
            if symbols_processed.len() == 1 {
                ""
            } else {
                "s"
            },
            total_timeframes,
            if total_timeframes == 1 { "" } else { "s" }
        ),
        symbols_processed,
        total_timeframes,
    })
}

fn check_symbol_exists(app_handle: &AppHandle, symbol: &str) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let symbol_json = app_dir.join("symbols").join(format!("{}.json", symbol));

    if symbol_json.exists() {
        return Err(format!(
            "Symbol '{}' already exists. Please delete it first.",
            symbol
        ));
    }

    Ok(())
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

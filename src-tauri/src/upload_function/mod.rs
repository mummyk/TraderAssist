// Module declarations
pub mod github;
pub mod local_upload;
pub mod yfinance;

// Re-export command functions for lib.rs
pub use local_upload::{
    delete_symbol, get_available_symbols, get_symbol_data, process_chart_folder, rename_symbol,
};

pub use github::fetch_github_data_command;

pub use yfinance::fetch_yfinance_data_command;

// If you need to use these types elsewhere, uncomment them:
// pub use local_upload::{DeleteResult, ProcessResult, RenameResult, SymbolData, TimeframeInfo};
// pub use github::{GitHubRequest, GitHubResult};
// pub use yfinance::{YFinanceRequest, YFinanceResult};

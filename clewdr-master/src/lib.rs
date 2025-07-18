use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;

pub mod api;
pub mod claude_code_state;
pub mod claude_web_body;
pub mod claude_web_state;
pub mod config;
pub mod error;
pub mod gemini_body;
pub mod gemini_state;
pub mod middleware;
pub mod router;
pub mod services;
pub mod types;
pub mod utils;

pub const IS_DEBUG: bool = cfg!(debug_assertions);
pub static IS_DEV: LazyLock<bool> = LazyLock::new(|| std::env::var("CARGO_MANIFEST_DIR").is_ok());

pub static VERSION_INFO: LazyLock<String> = LazyLock::new(|| {
    format!(
        "v{} by {}\n| profile: {}\n| mode: {}\n| no_fs: {}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        if IS_DEBUG { "debug" } else { "release" },
        if *IS_DEV { "dev" } else { "prod" },
        if cfg!(feature = "no_fs") {
            "true"
        } else {
            "false"
        }
    )
});

pub const FIG: &str = r#"
    //   ) )                                    //   ) ) 
   //        //  ___                   ___   / //___/ /  
  //        // //___) ) //  / /  / / //   ) / / ___ (    
 //        // //       //  / /  / / //   / / //   | |    
((____/ / // ((____   ((__( (__/ / ((___/ / //    | |    
"#;

/// Header for the application
pub static BANNER: LazyLock<String> = LazyLock::new(|| format!("{}\n{}", FIG, *VERSION_INFO));

/// Command line arguments for the application
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    /// Force update of the application
    pub update: bool,
    #[arg(short, long)]
    /// load cookie from file
    pub file: Option<PathBuf>,
    /// Alternative config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

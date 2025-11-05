//! CLI wrapper for GOAP-driven LLM system
//!
//! This binary provides a command-line interface for the GOAP system,
//! allowing users to process requests, manage patterns, and view metrics.

use clap::{Parser, Subcommand};
use goap_llm::GOAPSystem;
use serde_json::json;
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Parser, Debug)]
#[command(name = "goap-llm")]
#[command(about = "GOAP-driven LLM Strategic Reasoning System")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Output format (json, text)
    #[arg(long, default_value = "text")]
    output: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Process a request through the GOAP planner
    Process {
        /// The request to process
        request: String,

        /// Token budget for this request
        #[arg(long)]
        token_budget: Option<u32>,

        /// Output file for the response
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Maximum number of replans
        #[arg(long)]
        max_replans: Option<u32>,

        /// Enable reactive replanning
        #[arg(long)]
        enable_replanning: bool,
    },

    /// List cached patterns
    Patterns {
        #[command(subcommand)]
        subcommand: PatternSubcommand,
    },

    /// View system metrics
    Metrics {
        /// Time window for metrics (e.g., 1h, 24h, 7d)
        #[arg(short, long, default_value = "1h")]
        window: String,
    },

    /// Validate a request without full execution
    Validate {
        /// The request to validate
        request: String,

        /// Output file for validation results
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
enum PatternSubcommand {
    /// List all patterns
    List {
        /// Minimum confidence threshold
        #[arg(long)]
        min_confidence: Option<u8>,
    },

    /// Show details of a specific pattern
    Show {
        /// Pattern ID
        id: String,
    },

    /// Delete a pattern
    Delete {
        /// Pattern ID
        id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let cli = Cli::parse();

    // Initialize GOAP system
    let mut goap = GOAPSystem::new();

    match cli.command {
        Commands::Process {
            request,
            token_budget,
            output,
            max_replans,
            enable_replanning,
        } => {
            info!("Processing request: {}", request);

            // Configure system
            if let Some(budget) = token_budget {
                goap = goap.with_token_budget(budget);
            }
            if let Some(max_replans) = max_replans {
                goap = goap.max_replans(max_replans);
            }
            if enable_replanning {
                goap = goap.enable_replanning(true);
            }

            // Process the request
            match goap.process_request(request).await {
                Ok(response) => {
                    if let Some(output_path) = output {
                        std::fs::write(&output_path, &response)
                            .map_err(|e| anyhow::anyhow!("Failed to write output: {}", e))?;
                        info!("Response written to: {}", output_path.display());
                    } else {
                        println!("{}", response);
                    }

                    // Print metrics if requested
                    let metrics = goap.metrics();
                    if cli.output == "json" {
                        let output = json!({
                            "response": response,
                            "metrics": metrics,
                        });
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    }
                }
                Err(e) => {
                    error!("Processing failed: {}", e);
                    return Err(anyhow::anyhow!("{}", e));
                }
            }
        }

        Commands::Patterns { subcommand } => {
            match subcommand {
                PatternSubcommand::List { min_confidence } => {
                    let patterns = goap.list_patterns().await?;

                    let filtered_patterns = if let Some(min_conf) = min_confidence {
                        patterns.into_iter()
                            .filter(|p| p.confidence >= min_conf)
                            .collect()
                    } else {
                        patterns
                    };

                    if cli.output == "json" {
                        println!("{}", serde_json::to_string_pretty(&filtered_patterns)?);
                    } else {
                        for pattern in filtered_patterns {
                            println!("Pattern {}: confidence={}% used={} times",
                                pattern.id,
                                pattern.confidence,
                                pattern.usage_count
                            );
                        }
                    }
                }

                PatternSubcommand::Show { id } => {
                    match goap.get_pattern(&id).await {
                        Ok(pattern_detail) => {
                            if cli.output == "json" {
                                println!("{}", serde_json::to_string_pretty(&pattern_detail)?);
                            } else {
                                println!("Pattern: {}", pattern_detail.id);
                                println!("Confidence: {}%", pattern_detail.confidence);
                                println!("Usage count: {}", pattern_detail.usage_count);
                                println!("Success rate: {:.2}%", pattern_detail.success_rate * 100.0);
                                println!("Average tokens: {}", pattern_detail.avg_tokens);
                            }
                        }
                        Err(e) => {
                            error!("Pattern not found: {}", e);
                            return Err(anyhow::anyhow!("Pattern not found: {}", e));
                        }
                    }
                }

                PatternSubcommand::Delete { id } => {
                    match goap.delete_pattern(&id) {
                        Ok(_) => {
                            info!("Pattern deleted: {}", id);
                            println!("Pattern {} deleted successfully", id);
                        }
                        Err(e) => {
                            error!("Failed to delete pattern: {}", e);
                            return Err(anyhow::anyhow!("Failed to delete pattern: {}", e));
                        }
                    }
                }
            }
        }

        Commands::Metrics { window } => {
            let metrics = goap.metrics();

            if cli.output == "json" {
                println!("{}", serde_json::to_string_pretty(&metrics)?);
            } else {
                println!("System Metrics (Window: {})", window);
                println!("=====================================");
                println!("Success rate: {:.2}%", metrics.success_rate * 100.0);
                println!("Cache hit rate: {:.2}%", metrics.cache_hit_rate * 100.0);
                println!("Tokens saved: {}", metrics.tokens_saved);
                println!("Total requests: {}", metrics.total_requests);
                println!("Average planning time: {:.2}ms", metrics.avg_planning_time_ms);
            }
        }

        Commands::Validate { request, output } => {
            match goap.validate_request(request).await {
                Ok(validation) => {
                    if let Some(output_path) = output {
                        let json = serde_json::to_string_pretty(&validation)
                            .map_err(|e| anyhow::anyhow!("Failed to serialize: {}", e))?;
                        std::fs::write(&output_path, &json)
                            .map_err(|e| anyhow::anyhow!("Failed to write output: {}", e))?;
                        info!("Validation results written to: {}", output_path.display());
                    } else {
                        println!("Validation successful");
                        println!("Estimated tokens: {}", validation.estimated_tokens);
                        if let Some(pattern_match) = &validation.pattern_match {
                            println!("Pattern match confidence: {}%", pattern_match.confidence);
                        }
                    }
                }
                Err(e) => {
                    error!("Validation failed: {}", e);
                    return Err(anyhow::anyhow!("{}", e));
                }
            }
        }
    }

    Ok(())
}

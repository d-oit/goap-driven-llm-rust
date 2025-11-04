//! CLI Wrapper Example
//!
//! Demonstrates command-line interface usage with JSON input/output.

use goap_llm::prelude::*;
use std::io::{self, Read};

#[tokio::main]
async fn main() -> goap_llm::Result<()> {
    println!("=== GOAP CLI Wrapper Example ===\n");

    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let config = parse_cli_args(&args)?;

    // Read input from stdin or file
    let input = if let Some(input_file) = &config.input_file {
        std::fs::read_to_string(input_file)?
    } else {
        // Read from stdin
        println!("Enter your request (Ctrl+D when done):");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    };

    if input.is_empty() {
        println!("Error: No input provided");
        std::process::exit(1);
    }

    println!("Processing request: {}\n", input);

    // Process request
    let mut system = GOAPSystem::new();

    let start = std::time::Instant::now();
    let result = system.process_request(input.clone()).await?;
    let duration = start.elapsed();

    // Output results
    if config.output_format == OutputFormat::Json {
        output_json(&result, &config)?;
    } else {
        output_human_readable(&result, &config, duration)?;
    }

    println!("\n=== Complete ===");
    Ok(())
}

#[derive(Debug)]
struct CLIConfig {
    token_budget: u32,
    optimization_level: OptimizationLevel,
    output_format: OutputFormat,
    input_file: Option<String>,
    verbose: bool,
}

#[derive(Debug, Clone)]
enum OptimizationLevel {
    Speed,
    Quality,
    Balanced,
}

#[derive(Debug, Clone, PartialEq)]
enum OutputFormat {
    Human,
    Json,
}

fn parse_cli_args(args: &[String]) -> goap_llm::Result<CLIConfig> {
    let mut config = CLIConfig {
        token_budget: 5000,
        optimization_level: OptimizationLevel::Balanced,
        output_format: OutputFormat::Human,
        input_file: None,
        verbose: false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--budget" | "-b" => {
                if i + 1 < args.len() {
                    let value = args[i + 1]
                        .parse::<u32>()
                        .map_err(|e| goap_llm::Error::validation(e.to_string()))?;
                    config.token_budget = value;
                    i += 2;
                } else {
                    return Err(goap_llm::Error::validation(
                        "Missing budget value".to_string(),
                    ));
                }
            }
            "--optimize" | "-o" => {
                if i + 1 < args.len() {
                    config.optimization_level = match args[i + 1].as_str() {
                        "speed" => OptimizationLevel::Speed,
                        "quality" => OptimizationLevel::Quality,
                        "balanced" => OptimizationLevel::Balanced,
                        _ => {
                            return Err(goap_llm::Error::validation(
                                "Invalid optimization level".to_string(),
                            ));
                        }
                    };
                    i += 2;
                } else {
                    return Err(goap_llm::Error::validation(
                        "Missing optimization level".to_string(),
                    ));
                }
            }
            "--format" | "-f" => {
                if i + 1 < args.len() {
                    config.output_format = match args[i + 1].as_str() {
                        "json" => OutputFormat::Json,
                        "human" => OutputFormat::Human,
                        _ => return Err(goap_llm::Error::validation("Invalid format".to_string())),
                    };
                    i += 2;
                } else {
                    return Err(goap_llm::Error::validation(
                        "Missing format value".to_string(),
                    ));
                }
            }
            "--input" | "-i" => {
                if i + 1 < args.len() {
                    config.input_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err(goap_llm::Error::validation(
                        "Missing input file".to_string(),
                    ));
                }
            }
            "--verbose" | "-v" => {
                config.verbose = true;
                i += 1;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                if args[i].starts_with('-') {
                    return Err(goap_llm::Error::validation(format!(
                        "Unknown option: {}",
                        args[i]
                    )));
                }
                i += 1;
            }
        }
    }

    Ok(config)
}

fn build_actions_from_config(_config: &CLIConfig) -> goap_llm::Result<Vec<Action>> {
    Ok(vec![
        Action::new(ActionType::CheckPatternCache).with_cost(30),
        Action::new(ActionType::GenerateResponse).with_cost(400),
    ])
}

fn output_json(result: &str, config: &CLIConfig) -> goap_llm::Result<()> {
    println!("{{");
    println!("  \"success\": true,");
    println!("  \"response\": \"{}\",", result);
    println!("  \"config\": {{");
    println!("    \"budget\": {},", config.token_budget);
    println!("    \"optimization\": \"{:?}\"", config.optimization_level);
    println!("  }}");
    println!("}}");
    Ok(())
}

fn output_human_readable(
    result: &str,
    _config: &CLIConfig,
    duration: std::time::Duration,
) -> goap_llm::Result<()> {
    println!("=== Results ===\n");

    println!("Status: SUCCESS");
    println!("Duration: {}ms", duration.as_millis());
    println!("\nResponse:");
    println!("{}", result);

    Ok(())
}

fn print_help() {
    println!("GOAP CLI Wrapper");
    println!("\nUsage: goap-cli [OPTIONS]");
    println!("\nOptions:");
    println!("  -b, --budget <N>       Token budget (default: 5000)");
    println!("  -o, --optimize <TYPE>  Optimization: speed|quality|balanced (default: balanced)");
    println!("  -f, --format <FORMAT>  Output format: json|human (default: human)");
    println!("  -i, --input <FILE>     Input file (reads from stdin if not specified)");
    println!("  -v, --verbose          Verbose output");
    println!("  -h, --help             Show this help");
    println!("\nExamples:");
    println!("  echo 'Create a workflow' | cargo run --example cli_wrapper");
    println!("  cargo run --example cli_wrapper -- --input request.txt --format json");
    println!("  cargo run --example cli_wrapper -- --budget 3000 --optimize speed");
}

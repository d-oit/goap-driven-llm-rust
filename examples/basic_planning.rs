//! Basic Planning Example
//!
//! Demonstrates minimal GOAP planning setup with simple world state, actions, and goals.

use goap_llm::prelude::*;

#[tokio::main]
async fn main() -> goap_llm::Result<()> {
    println!("=== GOAP Basic Planning Example ===\n");

    // 1. Process request
    println!("Processing request...\n");

    let request = "Create a GitHub Actions workflow for a Node.js project".to_string();

    // 2. Create GOAP system
    let mut system = GOAPSystem::new();
    println!("GOAP System initialized\n");

    // 3. Process the request
    let result = system.process_request(request.clone()).await;

    match result {
        Ok(response) => {
            println!("✓ Planning and execution completed successfully!");
            println!("\nExecution results:");
            println!("  {}", response);
        }

        Err(e) => {
            println!("✗ Error during processing: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}

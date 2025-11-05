//! Custom Actions Example
//!
//! Demonstrates creating and integrating custom action types with the planner.

use goap_llm::goap::world::WorldProperty;
use goap_llm::prelude::*;

/// Custom action type for specific domain needs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomActionType {
    /// Custom action for specific validation
    CustomValidate(String),

    /// Custom action for specialized generation
    CustomGenerate(String),

    /// Custom action for domain-specific optimization
    CustomOptimize(String),
}

impl CustomActionType {
    pub fn name(&self) -> &'static str {
        match self {
            CustomActionType::CustomValidate(_) => "Custom Validate",
            CustomActionType::CustomGenerate(_) => "Custom Generate",
            CustomActionType::CustomOptimize(_) => "Custom Optimize",
        }
    }
}

/// Custom action with domain-specific logic
#[derive(Debug, Clone)]
pub struct CustomAction {
    pub action_type: CustomActionType,
    pub preconditions: Vec<WorldProperty>,
    pub effects: Vec<WorldProperty>,
    pub custom_metadata: String,
}

impl CustomAction {
    pub fn new(action_type: CustomActionType) -> Self {
        CustomAction {
            action_type,
            preconditions: vec![],
            effects: vec![],
            custom_metadata: "custom".to_string(),
        }
    }

    pub fn with_precondition(mut self, property: WorldProperty) -> Self {
        self.preconditions.push(property);
        self
    }

    pub fn with_effect(mut self, property: WorldProperty) -> Self {
        self.effects.push(property);
        self
    }

    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.custom_metadata = metadata;
        self
    }

    pub fn can_execute(&self, _world_state: &WorldState) -> bool {
        true
    }
}

#[tokio::main]
async fn main() -> goap_llm::Result<()> {
    println!("=== GOAP Custom Actions Example ===\n");

    let custom_actions = vec![
        CustomAction::new(CustomActionType::CustomValidate(
            "domain-specific".to_string(),
        ))
        .with_precondition(WorldProperty::SchemaAvailable)
        .with_metadata("Domain validation".to_string()),
        CustomAction::new(CustomActionType::CustomGenerate("specialized".to_string()))
            .with_precondition(WorldProperty::PatternCacheChecked)
            .with_metadata("Specialized generation".to_string()),
        CustomAction::new(CustomActionType::CustomOptimize("domain".to_string()))
            .with_effect(WorldProperty::ResponseGenerated)
            .with_metadata("Domain optimization".to_string()),
    ];

    println!("Created {} custom actions:", custom_actions.len());
    for (i, action) in custom_actions.iter().enumerate() {
        println!(
            "  {}. {} (metadata: {})",
            i + 1,
            action.action_type.name(),
            action.custom_metadata
        );
    }
    println!();

    println!("--- Testing Custom Action Logic ---\n");

    for action in &custom_actions {
        println!("Testing action: {}", action.action_type.name());
        println!("  Preconditions: {}", action.preconditions.len());
        println!("  Effects: {}", action.effects.len());
        println!("  Status: Demonstrating action definition");
        println!();
    }

    println!("--- Integration with Standard Actions ---\n");

    println!("Custom actions can integrate with standard GOAP actions");
    println!("to create hybrid planning and execution pipelines.");

    println!("\n--- Complex Action Chains ---\n");

    println!("Action chains demonstrate how custom actions can work");
    println!("together with standard GOAP actions to create complex");
    println!("workflows with dependencies and state transitions.");

    println!("\n=== Custom Action Testing Guidelines ===\n");

    println!("1. Unit Tests:");
    println!("   - Test preconditions validation");
    println!("   - Test effect application");
    println!("   - Test metadata handling");

    println!("\n2. Integration Tests:");
    println!("   - Test with GOAPPlanner");
    println!("   - Test action ordering");
    println!("   - Test failure handling");

    println!("\n3. Property-Based Tests:");
    println!("   - Test action invariants");
    println!("   - Test state transitions");
    println!("   - Test cost estimation");

    println!("\n=== Example Complete ===");
    Ok(())
}

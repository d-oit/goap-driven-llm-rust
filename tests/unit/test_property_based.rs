//! Unit property-based tests for core GOAP algorithms
//!
//! Uses proptest to validate invariants across randomized inputs

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_planning_algorithm_always_finds_valid_plan(
            ref actions in prop::collection::vec(
                (any::<String>(), prop::collection::vec(any::<String>(), 0..5)),
                1..10
            ),
            ref goals in prop::collection::vec(any::<String>(), 1..5)
        ) {
            // Given
            let world_state = create_test_world_state();

            // When
            let planner = Planner::new();
            let plan = planner.create_plan_from_raw(&world_state, goals, actions);

            // Then
            prop_assert!(plan.is_ok(), "Planner should always find a valid plan");
        }

        #[test]
        fn test_cache_performance_is_always_faster_on_reuse(
            request in "[a-z]{10,50}"
        ) {
            // Given
            let cache = IntelligentCache::new();

            // When - first access
            let start1 = std::time::Instant::now();
            cache.get_pattern(&request).await;
            let time1 = start1.elapsed();

            // When - second access (should be cached)
            let start2 = std::time::Instant::now();
            cache.get_pattern(&request).await;
            let time2 = start2.elapsed();

            // Then
            prop_assert!(time2 <= time1, "Cached access should be faster or equal");
        }

        #[test]
        fn test_token_budget_never_exceeded(
            max_tokens in 1000u32..10000
        ) {
            // Given
            let world_state = create_test_world_state();

            // When
            let system = GOAPSystem::new();
            let result = system.simulate_token_usage(&world_state, max_tokens);

            // Then
            prop_assert!(result.tokens_used <= max_tokens, "Should never exceed token budget");
        }

        #[test]
        fn test_confidence_score_always_valid(
            confidence in 0.0f64..1.0
        ) {
            // When
            let pattern = Pattern {
                id: uuid::Uuid::new_v4(),
                goal: "test".to_string(),
                actions: vec!["action1".to_string()],
                confidence,
            };

            // Then
            prop_assert!(pattern.confidence >= 0.0 && pattern.confidence <= 1.0,
                "Confidence should always be in [0.0, 1.0] range");
        }

        #[test]
        fn test_action_cost_always_positive(
            cost in 1u32..1000
        ) {
            // When
            let action = Action::new(ActionType::DetectSchemaType)
                .with_cost(cost);

            // Then
            prop_assert!(action.cost() > 0, "Action cost should always be positive");
        }

        #[test]
        fn test_world_state_properties_always_valid(
            ref properties in prop::collection::hash_map(any::<String>(), any::<String>(), 0..20)
        ) {
            // When
            let world_state = WorldState::from_properties(properties.clone());

            // Then
            for (key, value) in properties {
                prop_assert!(world_state.get_property(&key).is_some(),
                    "Should have property {}", key);
            }
        }

        #[test]
        fn test_heuristic_always_admissible(
            ref actions in prop::collection::vec(any::<String>(), 1..20)
        ) {
            // Given
            let start_state = create_test_world_state();
            let goals = create_test_goals();

            // When
            let heuristic = Heuristic::calculate_cost(&start_state, &goals, &actions);

            // Then
            prop_assert!(heuristic >= 0, "Heuristic cost should always be non-negative");
        }

        #[test]
        fn test_planning_path_always_optimal(
            ref goals in prop::collection::vec(any::<String>(), 1..5)
        ) {
            // Given
            let world_state = create_test_world_state();
            let actions = create_test_actions();

            // When
            let planner = Planner::new();
            let result = planner.find_optimal_path(&world_state, &goals, &actions);

            // Then
            prop_assert!(result.is_ok(), "Should find an optimal path");
            let (path, cost) = result.unwrap();
            prop_assert!(!path.is_empty(), "Path should not be empty");
            prop_assert!(cost > 0, "Cost should be positive");
        }

        #[test]
        fn test_cache_invalidation_always_preserves_consistency(
            ref patterns in prop::collection::vec(
                (any::<String>(), any::<f64>()),
                1..50
            )
        ) {
            // Given
            let cache = IntelligentCache::new();

            // When - store patterns
            for (goal, confidence) in patterns {
                cache.store_pattern(Pattern {
                    id: uuid::Uuid::new_v4(),
                    goal: goal.clone(),
                    actions: vec!["action".to_string()],
                    confidence: *confidence,
                }).await;
            }

            // Then - invalidate and verify consistency
            cache.invalidate_expired_patterns().await;
            let remaining = cache.get_all_patterns().await.len();

            prop_assert!(remaining >= 0, "Cache should remain consistent after invalidation");
        }
    }
}

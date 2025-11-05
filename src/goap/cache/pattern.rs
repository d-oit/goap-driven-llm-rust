//! Success pattern definitions
//!
//! Represents successful execution patterns that can be reused for efficiency.

use crate::goap::actions::ActionType;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Represents a successful execution pattern
#[derive(Debug, Clone)]
pub struct SuccessPattern {
    /// Unique identifier
    pub id: String,

    /// Pattern signature (hash of request characteristics)
    pub signature: String,

    /// Confidence score (0-100)
    pub confidence: f64,

    /// Number of times used
    pub usage_count: u32,

    /// Action sequence that led to success
    pub action_sequence: Vec<ActionType>,

    /// Average tokens consumed
    pub avg_tokens: u32,

    /// Success rate (0-1)
    pub success_rate: f64,

    /// Creation time
    pub created_at: Instant,

    /// Last used time
    pub last_used: Option<Instant>,

    /// Total duration in milliseconds
    pub total_duration_ms: u64,
}

impl SuccessPattern {
    /// Create a new success pattern
    pub fn new(
        signature: String,
        action_sequence: Vec<ActionType>,
        avg_tokens: u32,
        success_rate: f64,
    ) -> Self {
        SuccessPattern {
            id: Uuid::new_v4().to_string(),
            signature,
            confidence: 0.0,
            usage_count: 0,
            action_sequence,
            avg_tokens,
            success_rate,
            created_at: Instant::now(),
            last_used: None,
            total_duration_ms: 0,
        }
    }

    /// Calculate confidence score based on usage and success rate
    pub fn calculate_confidence(&self) -> f64 {
        let usage_factor = (self.usage_count as f64).min(10.0) / 10.0; // Cap at 10 uses
        let success_factor = self.success_rate;

        (usage_factor * 0.6 + success_factor * 0.4) * 100.0
    }

    /// Update pattern with new usage
    pub fn record_usage(&mut self, tokens_used: u32, duration_ms: u64, success: bool) {
        self.usage_count += 1;
        self.last_used = Some(Instant::now());

        // Update running averages
        let total_tokens_before = self
            .avg_tokens
            .saturating_mul(self.usage_count.saturating_sub(1));
        self.avg_tokens = (total_tokens_before + tokens_used) / self.usage_count;

        self.total_duration_ms += duration_ms;

        // Update success rate
        if success {
            // Simple moving average
            self.success_rate = (self.success_rate * 0.9) + (1.0 * 0.1);
        } else {
            self.success_rate = (self.success_rate * 0.9) + (0.0 * 0.1);
        }

        // Recalculate confidence
        self.confidence = self.calculate_confidence();
    }

    /// Check if pattern is valid for reuse
    pub fn is_valid(&self) -> bool {
        self.confidence >= 70.0 && self.success_rate >= 0.8
    }

    /// Get average duration per use
    pub fn avg_duration(&self) -> u64 {
        if self.usage_count == 0 {
            0
        } else {
            self.total_duration_ms / self.usage_count as u64
        }
    }

    /// Get pattern age
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Calculate token savings compared to baseline
    pub fn token_savings(&self, baseline_avg_tokens: u32) -> u32 {
        baseline_avg_tokens.saturating_sub(self.avg_tokens)
    }
}

/// Pattern similarity metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSimilarity {
    /// Similarity score (0-1)
    pub score: f64,

    /// Confidence in the similarity
    pub confidence: f64,
}

impl PatternSimilarity {
    /// Create a new similarity result
    pub fn new(score: f64, confidence: f64) -> Self {
        PatternSimilarity { score, confidence }
    }
}

/// Similarity algorithm types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimilarityAlgorithm {
    /// Jaccard similarity for sets
    Jaccard,

    /// Cosine similarity for vectors
    Cosine,

    /// Locality-sensitive hashing (LSH)
    LSH,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_creation() {
        let pattern = SuccessPattern::new(
            "test_signature".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            1.0,
        );
        assert_eq!(pattern.usage_count, 0);
        assert_eq!(pattern.success_rate, 1.0);
    }

    #[test]
    fn test_record_usage() {
        let mut pattern = SuccessPattern::new(
            "test".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            1.0,
        );

        pattern.record_usage(150, 500, true);

        assert_eq!(pattern.usage_count, 1);
        assert_eq!(pattern.avg_tokens, 150);
    }

    #[test]
    fn test_is_valid() {
        let mut pattern = SuccessPattern::new(
            "test".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            0.5,
        );

        // Low confidence and success rate
        assert!(!pattern.is_valid());

        // Increase usage and success rate
        for _ in 0..10 {
            pattern.record_usage(100, 500, true);
        }

        assert!(pattern.is_valid());
    }

    #[test]
    fn test_confidence_calculation() {
        let mut pattern = SuccessPattern::new(
            "test".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            0.8,
        );

        assert!(pattern.confidence >= 0.0);

        for _i in 1..=10 {
            pattern.record_usage(100, 500, true);
            assert!(pattern.confidence >= 0.0);
        }
    }
}

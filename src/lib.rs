//! # Calculator Library
//!
//! A simple calculator library with expression evaluation and operator precedence.
//! Also includes GUI state management that can be unit tested.

pub mod calculator;
pub mod display;
pub mod input;
pub mod ui;

// Re-export main types for convenience
pub use calculator::{Calculator, Operation};
pub use ui::{CalculatorUIState, MessageResult, UIMessage};

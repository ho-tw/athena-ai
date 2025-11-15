use serde::{Deserialize, Serialize};

/// Result of executing a complete plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Whether the plan executed successfully
    pub success: bool,
    /// The final response to return to the user
    pub final_response: String,
    /// Results from each step in the plan
    pub step_results: Vec<StepResult>,
}

/// Result of executing a single step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Type of step that was executed
    pub step_type: String,
    /// Output from the step execution
    pub output: String,
    /// Whether the step executed successfully
    pub success: bool,
}

impl StepResult {
    /// Create a successful step result
    pub fn success(step_type: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            step_type: step_type.into(),
            output: output.into(),
            success: true,
        }
    }

    /// Create a failed step result
    pub fn failure(step_type: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            step_type: step_type.into(),
            output: output.into(),
            success: false,
        }
    }
}

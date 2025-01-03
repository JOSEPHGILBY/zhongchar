use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LearningSession {
    pub radical_forms_covered: Vec<char>,
    
}

pub struct NLearningFrame {
}
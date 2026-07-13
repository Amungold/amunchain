/// خطأ محلي لطبقة التحقق، لتجنب الاعتماد على تعريفات دستورية خارجية.
#[derive(Debug, Clone)]
pub enum FormalError {
    StateReadError(String),
    InvariantFailed(String),
}

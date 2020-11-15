use std::collections::HashMap;
pub enum Variable {
    Number
}
pub struct Environment {
    scope_stack: HashMap<String, String>
}
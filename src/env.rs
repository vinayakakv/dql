use std::collections::HashMap;

pub struct Env {
    cte_table: HashMap<String, String>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            cte_table: HashMap::new(),
        }
    }

    pub fn insert_cte(&mut self, name: String, value: String) {
        self.cte_table.insert(name, value);
    }

    pub fn get_cte(&self, name: &str) -> Option<&String> {
        self.cte_table.get(name)
    }
}

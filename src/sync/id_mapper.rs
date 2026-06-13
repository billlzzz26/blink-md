//! ID Mapping for bidirectional sync between Notion and IR.

use std::collections::HashMap;

/// Maps Notion IDs to IR document IDs and vice versa.
#[derive(Debug, Default, Clone)]
pub struct IdMapper {
    /// Notion page/block ID -> IR ID
    notion_to_ir: HashMap<String, String>,
    /// IR ID -> Notion page/block ID  
    ir_to_notion: HashMap<String, String>,
}

impl IdMapper {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a mapping between Notion and IR IDs
    pub fn insert(&mut self, notion_id: impl Into<String>, ir_id: impl Into<String>) {
        let notion_id = notion_id.into();
        let ir_id = ir_id.into();
        self.notion_to_ir.insert(notion_id.clone(), ir_id.clone());
        self.ir_to_notion.insert(ir_id, notion_id);
    }

    /// Get IR ID for Notion ID
    pub fn get_ir(&self, notion_id: &str) -> Option<&str> {
        self.notion_to_ir.get(notion_id).map(|s| s.as_str())
    }

    /// Get Notion ID for IR ID
    pub fn get_notion(&self, ir_id: &str) -> Option<&str> {
        self.ir_to_notion.get(ir_id).map(|s| s.as_str())
    }

    /// Check if mapping exists
    pub fn contains(&self, notion_id: &str) -> bool {
        self.notion_to_ir.contains_key(notion_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_lookup() {
        let mut mapper = IdMapper::new();
        mapper.insert("notion-123", "ir-abc");

        assert_eq!(mapper.get_ir("notion-123"), Some("ir-abc"));
        assert_eq!(mapper.get_notion("ir-abc"), Some("notion-123"));
    }
}

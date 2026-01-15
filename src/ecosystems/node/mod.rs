mod parser;
mod scanner;

use crate::core::dependency::EcosystemType;
use crate::ecosystems::traits::{DependencyParser, EcosystemAdapter, ImportScanner};

pub struct NodeAdapter {
    parser: parser::NodeParser,
    scanner: scanner::NodeScanner,
}

impl Default for NodeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeAdapter {
    pub fn new() -> Self {
        Self {
            parser: parser::NodeParser,
            scanner: scanner::NodeScanner,
        }
    }
}

impl EcosystemAdapter for NodeAdapter {
    fn parser(&self) -> &dyn DependencyParser {
        &self.parser
    }

    fn scanner(&self) -> &dyn ImportScanner {
        &self.scanner
    }

    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Node
    }
}

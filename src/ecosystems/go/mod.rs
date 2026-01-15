mod parser;
mod scanner;

use crate::ecosystems::traits::{DependencyParser, EcosystemAdapter, ImportScanner};
use crate::core::dependency::EcosystemType;

pub struct GoAdapter {
    parser: parser::GoParser,
    scanner: scanner::GoScanner,
}

impl GoAdapter {
    pub fn new() -> Self {
        Self {
            parser: parser::GoParser,
            scanner: scanner::GoScanner,
        }
    }
}

impl EcosystemAdapter for GoAdapter {
    fn parser(&self) -> &dyn DependencyParser {
        &self.parser
    }

    fn scanner(&self) -> &dyn ImportScanner {
        &self.scanner
    }

    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Go
    }
}

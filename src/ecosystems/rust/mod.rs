mod parser;
mod scanner;

use crate::ecosystems::traits::{DependencyParser, EcosystemAdapter, ImportScanner};
use crate::core::dependency::EcosystemType;

pub struct RustAdapter {
    parser: parser::RustParser,
    scanner: scanner::RustScanner,
}

impl RustAdapter {
    pub fn new() -> Self {
        Self {
            parser: parser::RustParser,
            scanner: scanner::RustScanner,
        }
    }
}

impl EcosystemAdapter for RustAdapter {
    fn parser(&self) -> &dyn DependencyParser {
        &self.parser
    }

    fn scanner(&self) -> &dyn ImportScanner {
        &self.scanner
    }

    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Rust
    }
}

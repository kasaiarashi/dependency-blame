mod parser;
mod scanner;

use crate::ecosystems::traits::{DependencyParser, EcosystemAdapter, ImportScanner};
use crate::core::dependency::EcosystemType;

pub struct PythonAdapter {
    parser: parser::PythonParser,
    scanner: scanner::PythonScanner,
}

impl PythonAdapter {
    pub fn new() -> Self {
        Self {
            parser: parser::PythonParser,
            scanner: scanner::PythonScanner,
        }
    }
}

impl EcosystemAdapter for PythonAdapter {
    fn parser(&self) -> &dyn DependencyParser {
        &self.parser
    }

    fn scanner(&self) -> &dyn ImportScanner {
        &self.scanner
    }

    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Python
    }
}

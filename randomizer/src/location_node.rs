use crate::check::Check;
use crate::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct LocationNode {
    checks: Vec<Check>,
    paths: Vec<Path>,
}

impl LocationNode {
    pub fn new(_name: &'static str, checks: Vec<Check>, paths: Vec<Path>) -> Self {
        Self { checks, paths }
    }

    pub fn get_checks(self) -> Vec<Check> {
        self.checks
    }

    pub fn get_paths(self) -> Vec<Path> {
        self.paths
    }
}
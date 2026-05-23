//! Contract harness utilities.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::{config::ValidateConfig, error::ErrorKind, runtime::RuntimeState, TunexError};

use super::{ContractScope, ContractTest, TestAssertion, TestFixture};

/// Report produced by a contract harness.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HarnessReport {
    /// Harness name.
    pub name: String,
    /// Assertions collected during the run.
    pub assertions: SmallVec<[TestAssertion; 8]>,
}

impl HarnessReport {
    /// Returns whether all required assertions passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.assertions.iter().all(|assertion| assertion.passed)
    }
}

/// Reusable contract harness for module repositories.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContractHarness {
    /// Fixture used by the harness.
    pub fixture: TestFixture,
    /// Contracts to evaluate.
    pub contracts: SmallVec<[ContractTest; 8]>,
}

impl ContractHarness {
    /// Creates a contract harness.
    #[must_use]
    pub fn new(fixture: TestFixture) -> Self {
        Self {
            fixture,
            contracts: SmallVec::new(),
        }
    }

    /// Adds a contract test.
    #[must_use]
    pub fn with_contract(mut self, contract: ContractTest) -> Self {
        self.contracts.push(contract);
        self
    }

    /// Runs built-in structural assertions for the configured contracts.
    #[must_use]
    pub fn verify(&self) -> HarnessReport {
        let mut assertions = SmallVec::new();
        for contract in &self.contracts {
            assertions.push(match contract.scope {
                ContractScope::ConfigValidation => self.verify_config(&contract.name),
                ContractScope::RuntimeState => Self::verify_runtime_state(&contract.name),
                ContractScope::ErrorPropagation => Self::verify_error_propagation(&contract.name),
                _ => TestAssertion::passed(&contract.name, "contract registered"),
            });
        }
        HarnessReport {
            name: self.fixture.name.clone(),
            assertions,
        }
    }

    fn verify_config(&self, name: &str) -> TestAssertion {
        let report = self.fixture.config.validate();
        if report.is_valid() {
            TestAssertion::passed(name, "config validation passed")
        } else {
            TestAssertion::failed(name, "config validation failed")
        }
    }

    fn verify_runtime_state(name: &str) -> TestAssertion {
        if RuntimeState::Created.can_transition_to(RuntimeState::Initializing) {
            TestAssertion::passed(name, "runtime state transition contract passed")
        } else {
            TestAssertion::failed(name, "runtime state transition contract failed")
        }
    }

    fn verify_error_propagation(name: &str) -> TestAssertion {
        let error = TunexError::new(ErrorKind::Runtime, "contract error");
        if error.category() == "runtime" {
            TestAssertion::passed(name, "error propagation contract passed")
        } else {
            TestAssertion::failed(name, "error propagation contract failed")
        }
    }
}

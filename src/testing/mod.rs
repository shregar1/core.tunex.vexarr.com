//! Testing harnesses, mocks, fixtures, and contract verification utilities.

pub mod assertion;
pub mod contract;
pub mod fixture;
pub mod harness;
pub mod mock;

pub use assertion::{AssertionLevel, TestAssertion};
pub use contract::{
    assert_checkpoint_writer, assert_dataset_source, assert_metrics_sink, assert_sequence_packer,
    assert_tokenizer, assert_trainer_backend, ContractScope, ContractTest,
};
pub use fixture::TestFixture;
pub use harness::{ContractHarness, HarnessReport};
pub use mock::{
    MockCheckpointWriter, MockDatasetSource, MockSequencePacker, MockTokenizer, MockTrainerBackend,
};

//! Experimental part of configuration.

use std::num::NonZeroU32;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ExperimentalDBConfig {
    /// Block cache capacity of the state keeper RocksDB cache. The default value is 128 MB.
    #[serde(default = "ExperimentalDBConfig::default_state_keeper_db_block_cache_capacity_mb")]
    pub state_keeper_db_block_cache_capacity_mb: usize,
    /// Maximum number of files concurrently opened by state keeper cache RocksDB. Useful to fit into OS limits; can be used
    /// as a rudimentary way to control RAM usage of the cache.
    pub state_keeper_db_max_open_files: Option<NonZeroU32>,
    #[serde(default = "ExperimentalDBConfig::default_protective_reads_persistence_enabled")]
    pub reads_persistence_enabled: bool,
    #[serde(default = "ExperimentalDBConfig::default_merkle_tree_processing_delay_ms")]
    pub processing_delay_ms: u64,
    #[serde(default)]
    pub include_indices_and_filters_in_block_cache: bool,
}

impl Default for ExperimentalDBConfig {
    fn default() -> Self {
        Self {
            state_keeper_db_block_cache_capacity_mb:
                Self::default_state_keeper_db_block_cache_capacity_mb(),
            state_keeper_db_max_open_files: None,
            reads_persistence_enabled: Self::default_protective_reads_persistence_enabled(),
            processing_delay_ms: 0,
            include_indices_and_filters_in_block_cache: false,
        }
    }
}

impl ExperimentalDBConfig {
    const fn default_state_keeper_db_block_cache_capacity_mb() -> usize {
        128
    }

    pub fn state_keeper_db_block_cache_capacity(&self) -> usize {
        self.state_keeper_db_block_cache_capacity_mb * super::BYTES_IN_MEGABYTE
    }
    const fn default_protective_reads_persistence_enabled() -> bool {
        true
    }
    const fn default_merkle_tree_processing_delay_ms() -> u64 {
        100
    }
}

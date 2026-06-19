use std::error::Error;
use std::fmt;

use super::{HillslopeWritebackSurface, PerOfeDailyWaterBalanceRecord};

/// OFE-keyed daily water-balance shadow collection for staged MOFE migration.
#[derive(Debug, Clone)]
pub struct PerOfeDailyWaterBalanceCollection {
    simulation_day_index: usize,
    contributor_ofe_count: usize,
    records: Vec<PerOfeDailyWaterBalanceRecord>,
}

impl PerOfeDailyWaterBalanceCollection {
    /// Construct an empty daily per-OFE collection.
    ///
    /// # Errors
    ///
    /// Returns `PerOfeDailyWaterBalanceError` when the day index or
    /// contributor OFE count is outside the contract domain.
    pub fn new(
        simulation_day_index: usize,
        contributor_ofe_count: usize,
    ) -> Result<Self, PerOfeDailyWaterBalanceError> {
        if simulation_day_index == 0 {
            return Err(PerOfeDailyWaterBalanceError::InvalidSimulationDayIndex {
                simulation_day_index,
            });
        }
        if contributor_ofe_count == 0 {
            return Err(PerOfeDailyWaterBalanceError::InvalidContributorOfeCount {
                contributor_ofe_count,
            });
        }

        Ok(Self {
            simulation_day_index,
            contributor_ofe_count,
            records: Vec::with_capacity(contributor_ofe_count),
        })
    }

    pub fn push_record(
        &mut self,
        record: PerOfeDailyWaterBalanceRecord,
    ) -> Result<(), PerOfeDailyWaterBalanceError> {
        let expected_ofe_id = self.records.len() + 1;
        if self.records.len() >= self.contributor_ofe_count {
            return Err(PerOfeDailyWaterBalanceError::TooManyRecords {
                contributor_ofe_count: self.contributor_ofe_count,
            });
        }
        if record.ofe_id != expected_ofe_id {
            return Err(PerOfeDailyWaterBalanceError::NonSequentialOfeRecord {
                expected_ofe_id,
                observed_ofe_id: record.ofe_id,
            });
        }
        if record.upstream_transfer_input.recipient_ofe_id != record.ofe_id {
            return Err(PerOfeDailyWaterBalanceError::TransferRecipientMismatch {
                ofe_id: record.ofe_id,
                recipient_ofe_id: record.upstream_transfer_input.recipient_ofe_id,
            });
        }
        let expected_upstream_source = record.ofe_id.checked_sub(1).filter(|source| *source > 0);
        if record.upstream_transfer_input.source_ofe_id != expected_upstream_source {
            return Err(PerOfeDailyWaterBalanceError::TransferInputSourceMismatch {
                ofe_id: record.ofe_id,
                expected_source_ofe_id: expected_upstream_source,
                observed_source_ofe_id: record.upstream_transfer_input.source_ofe_id,
            });
        }
        if record.current_transfer_output.source_ofe_id != record.ofe_id {
            return Err(PerOfeDailyWaterBalanceError::TransferOutputSourceMismatch {
                ofe_id: record.ofe_id,
                source_ofe_id: record.current_transfer_output.source_ofe_id,
            });
        }
        let expected_downstream_recipient = if record.ofe_id == self.contributor_ofe_count {
            None
        } else {
            Some(record.ofe_id + 1)
        };
        if record.current_transfer_output.recipient_ofe_id != expected_downstream_recipient {
            return Err(
                PerOfeDailyWaterBalanceError::TransferOutputRecipientMismatch {
                    source_ofe_id: record.ofe_id,
                    expected_recipient_ofe_id: expected_downstream_recipient,
                    observed_recipient_ofe_id: record.current_transfer_output.recipient_ofe_id,
                },
            );
        }

        self.records.push(record);
        Ok(())
    }

    #[must_use]
    pub const fn simulation_day_index(&self) -> usize {
        self.simulation_day_index
    }

    #[must_use]
    pub const fn contributor_ofe_count(&self) -> usize {
        self.contributor_ofe_count
    }

    #[must_use]
    pub fn records(&self) -> &[PerOfeDailyWaterBalanceRecord] {
        &self.records
    }

    #[must_use]
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Return the legacy scalar surface for the N=1 compatibility adapter.
    ///
    /// # Errors
    ///
    /// Returns `PerOfeDailyWaterBalanceError` for incomplete collections or
    /// for multi-OFE collections, whose aggregate derivation remains later
    /// M-E scope.
    pub fn aggregate_for_legacy_outer_consumers(
        &self,
    ) -> Result<HillslopeWritebackSurface, PerOfeDailyWaterBalanceError> {
        if self.contributor_ofe_count != 1 {
            return Err(
                PerOfeDailyWaterBalanceError::MultiOfeAggregateNotImplemented {
                    contributor_ofe_count: self.contributor_ofe_count,
                },
            );
        }
        let Some(record) = self.records.first() else {
            return Err(PerOfeDailyWaterBalanceError::IncompleteCollection {
                contributor_ofe_count: self.contributor_ofe_count,
                record_count: self.records.len(),
            });
        };

        Ok(record.post_day_state.clone())
    }
}

/// Construction and adapter errors for M-E per-OFE shadow state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerOfeDailyWaterBalanceError {
    InvalidSimulationDayIndex {
        simulation_day_index: usize,
    },
    InvalidContributorOfeCount {
        contributor_ofe_count: usize,
    },
    InvalidRecordOfeId {
        ofe_id: usize,
    },
    InvalidTransferSourceOfeId {
        source_ofe_id: usize,
    },
    TooManyRecords {
        contributor_ofe_count: usize,
    },
    NonSequentialOfeRecord {
        expected_ofe_id: usize,
        observed_ofe_id: usize,
    },
    TransferRecipientMismatch {
        ofe_id: usize,
        recipient_ofe_id: usize,
    },
    TransferInputSourceMismatch {
        ofe_id: usize,
        expected_source_ofe_id: Option<usize>,
        observed_source_ofe_id: Option<usize>,
    },
    TransferOutputSourceMismatch {
        ofe_id: usize,
        source_ofe_id: usize,
    },
    TransferOutputRecipientMismatch {
        source_ofe_id: usize,
        expected_recipient_ofe_id: Option<usize>,
        observed_recipient_ofe_id: Option<usize>,
    },
    IncompleteCollection {
        contributor_ofe_count: usize,
        record_count: usize,
    },
    MultiOfeAggregateNotImplemented {
        contributor_ofe_count: usize,
    },
}

impl fmt::Display for PerOfeDailyWaterBalanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSimulationDayIndex {
                simulation_day_index,
            } => write!(
                f,
                "simulation_day_index must be >= 1, observed {simulation_day_index}"
            ),
            Self::InvalidContributorOfeCount {
                contributor_ofe_count,
            } => write!(
                f,
                "contributor_ofe_count must be >= 1, observed {contributor_ofe_count}"
            ),
            Self::InvalidRecordOfeId { ofe_id } => {
                write!(f, "per-OFE record id must be >= 1, observed {ofe_id}")
            }
            Self::InvalidTransferSourceOfeId { source_ofe_id } => write!(
                f,
                "transfer output source OFE id cannot be incremented, observed {source_ofe_id}"
            ),
            Self::TooManyRecords {
                contributor_ofe_count,
            } => write!(
                f,
                "cannot append more than {contributor_ofe_count} per-OFE records"
            ),
            Self::NonSequentialOfeRecord {
                expected_ofe_id,
                observed_ofe_id,
            } => write!(
                f,
                "per-OFE records must be appended in OFE order; expected {expected_ofe_id}, observed {observed_ofe_id}"
            ),
            Self::TransferRecipientMismatch {
                ofe_id,
                recipient_ofe_id,
            } => write!(
                f,
                "upstream transfer recipient {recipient_ofe_id} does not match record OFE {ofe_id}"
            ),
            Self::TransferInputSourceMismatch {
                ofe_id,
                expected_source_ofe_id,
                observed_source_ofe_id,
            } => write!(
                f,
                "upstream transfer source {observed_source_ofe_id:?} does not match expected source {expected_source_ofe_id:?} for record OFE {ofe_id}"
            ),
            Self::TransferOutputSourceMismatch {
                ofe_id,
                source_ofe_id,
            } => write!(
                f,
                "transfer output source {source_ofe_id} does not match record OFE {ofe_id}"
            ),
            Self::TransferOutputRecipientMismatch {
                source_ofe_id,
                expected_recipient_ofe_id,
                observed_recipient_ofe_id,
            } => write!(
                f,
                "transfer output from OFE {source_ofe_id} targets {observed_recipient_ofe_id:?}; expected {expected_recipient_ofe_id:?}"
            ),
            Self::IncompleteCollection {
                contributor_ofe_count,
                record_count,
            } => write!(
                f,
                "per-OFE collection has {record_count} records for {contributor_ofe_count} contributing OFEs"
            ),
            Self::MultiOfeAggregateNotImplemented {
                contributor_ofe_count,
            } => write!(
                f,
                "aggregate derivation from {contributor_ofe_count} per-OFE records is later M-E scope"
            ),
        }
    }
}

impl Error for PerOfeDailyWaterBalanceError {}

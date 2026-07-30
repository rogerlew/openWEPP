use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::amendment::V2AmendmentReceipt;
use super::amendment_support::read_regular;
use super::identity::ReviewLock;
use crate::{AssuranceError, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct V2ReceiptReportRoots {
    pub science_root: String,
    pub communication_root: String,
    pub attribution_root: String,
    pub review_governance_root: String,
    pub content_review_subject_root: String,
    pub finding_ledger_root: Option<String>,
    pub preapproval_realization_root: String,
    pub pre_steward_approval_root: Option<String>,
    pub approval_lock_root: Option<String>,
    pub realization_root: Option<String>,
    pub release_transfer_root: Option<String>,
}

pub(super) fn receipt_roots(
    root: &Path,
    reports: &[String],
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
    allow_missing: bool,
) -> Result<BTreeMap<String, V2ReceiptReportRoots>> {
    let mut roots = BTreeMap::new();
    for report_id in reports {
        let path = PathBuf::from(format!("assurance/v2/reports/{report_id}/review.lock.json"));
        let bytes = if let Some(bytes) = replacements.get(&path) {
            bytes.clone()
        } else {
            match read_regular(root, &path) {
                Ok(bytes) => bytes,
                Err(_) if allow_missing => continue,
                Err(error) => return Err(error),
            }
        };
        let lock: ReviewLock =
            serde_json::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
                path: path.clone(),
                message: error.to_string(),
            })?;
        if lock.report_id != *report_id {
            return Err(AssuranceError::Invalid(format!(
                "review lock '{}' does not match report '{report_id}'",
                path.display()
            )));
        }
        roots.insert(report_id.clone(), V2ReceiptReportRoots::from(lock));
    }
    Ok(roots)
}

impl From<ReviewLock> for V2ReceiptReportRoots {
    fn from(lock: ReviewLock) -> Self {
        Self {
            science_root: lock.science_root,
            communication_root: lock.communication_root,
            attribution_root: lock.attribution_root,
            review_governance_root: lock.review_governance_root,
            content_review_subject_root: lock.content_review_subject_root,
            finding_ledger_root: lock.finding_ledger_root,
            preapproval_realization_root: lock.preapproval_realization_root,
            pre_steward_approval_root: lock.pre_steward_approval_root,
            approval_lock_root: lock.approval_lock_root,
            realization_root: lock.realization_root,
            release_transfer_root: lock.release_transfer_root,
        }
    }
}

pub(super) fn validate_receipt_contract(receipt: &V2AmendmentReceipt) -> Result<()> {
    match receipt.schema_version {
        1 if receipt.old_roots.is_none() && receipt.new_roots.is_none() => Ok(()),
        1 => Err(AssuranceError::Invalid(
            "schema-version 1 transaction receipt cannot declare root maps".to_owned(),
        )),
        2 => {
            let old = receipt.old_roots.as_ref().ok_or_else(|| {
                AssuranceError::Invalid(
                    "schema-version 2 transaction receipt requires old_roots".to_owned(),
                )
            })?;
            let new = receipt.new_roots.as_ref().ok_or_else(|| {
                AssuranceError::Invalid(
                    "schema-version 2 transaction receipt requires new_roots".to_owned(),
                )
            })?;
            let affected = receipt
                .affected_reports
                .iter()
                .collect::<std::collections::BTreeSet<_>>();
            let old_keys = old.keys().collect::<std::collections::BTreeSet<_>>();
            let new_keys = new.keys().collect::<std::collections::BTreeSet<_>>();
            let old_is_valid = if !receipt.changed {
                old_keys == affected
            } else if receipt.operation == "admit-report" {
                old_keys.is_empty()
            } else {
                old_keys == affected
            };
            if !old_is_valid || new_keys != affected {
                return Err(AssuranceError::Invalid(
                    "transaction receipt root maps do not match affected reports".to_owned(),
                ));
            }
            Ok(())
        }
        version => Err(AssuranceError::Invalid(format!(
            "unsupported transaction receipt schema_version {version}"
        ))),
    }
}

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::Deserialize;

use super::{
    DRAFT, Finding, PRINCIPAL_SCHEMA_VERSION, RequiredNullable, Review, V2TrustDomain,
    require_absent, require_nonempty, require_present_nonempty, require_unique, validate_id,
    validate_relative,
};
use crate::{AssuranceError, Result, sha256_bytes};

pub(super) fn digest_input_set(domain: &str, inputs: &BTreeMap<PathBuf, String>) -> String {
    let mut material = format!("openwepp-assurance-v2:{domain}\n");
    for (path, digest) in inputs {
        material.push_str(path.to_string_lossy().as_ref());
        material.push(' ');
        material.push_str(digest);
        material.push('\n');
    }
    sha256_bytes(material.as_bytes())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReaderMetadata {
    pub(super) scientific_question: String,
    pub(super) assessed_process: String,
    pub(super) assessed_quantity: String,
    pub(super) realization: String,
    pub(super) related_model_narrative: PathBuf,
    pub(super) manuscript_date: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrincipalRegistry {
    schema_version: u32,
    trust_domain: V2TrustDomain,
    pub(super) principals: Vec<Principal>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Principal {
    pub(super) id: String,
    pub(super) record_version: u32,
    #[serde(default)]
    supersedes: RequiredNullable<String>,
    pub(super) display_name: String,
    pub(super) affiliations: Vec<String>,
    pub(super) kind: PrincipalKind,
    identity_authority: String,
    identity_reference: String,
    pub(super) roles: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum PrincipalKind {
    Human,
    Organization,
    Agent,
    Software,
}

pub(super) fn validate_principal_registry(
    registry: &PrincipalRegistry,
    expected_domain: V2TrustDomain,
) -> Result<()> {
    if registry.schema_version != PRINCIPAL_SCHEMA_VERSION
        || registry.trust_domain != expected_domain
        || registry.principals.is_empty()
    {
        return Err(AssuranceError::Invalid(
            "principal registry version, trust domain, or cardinality is invalid".to_owned(),
        ));
    }
    let mut records = BTreeSet::new();
    let mut latest_versions = BTreeMap::new();
    for principal in &registry.principals {
        validate_id(&principal.id, "principal")?;
        if principal.record_version == 0 {
            return Err(AssuranceError::Invalid(format!(
                "principal '{}' record_version must be positive",
                principal.id
            )));
        }
        let record_id = format!("{}@{}", principal.id, principal.record_version);
        require_unique(&mut records, &record_id, "principal record")?;
        let previous = latest_versions
            .insert(principal.id.clone(), principal.record_version)
            .unwrap_or(0);
        if previous >= principal.record_version {
            return Err(AssuranceError::Invalid(format!(
                "principal '{}' versions must be strictly increasing",
                principal.id
            )));
        }
        for (value, label) in [
            (&principal.display_name, "principal display name"),
            (
                &principal.identity_authority,
                "principal identity authority",
            ),
            (
                &principal.identity_reference,
                "principal identity reference",
            ),
        ] {
            require_nonempty(value, label)?;
        }
        if principal.roles.is_empty() {
            return Err(AssuranceError::Invalid(format!(
                "principal '{}' requires at least one role",
                principal.id
            )));
        }
        if principal
            .affiliations
            .iter()
            .any(|affiliation| affiliation.trim().is_empty())
        {
            return Err(AssuranceError::Invalid(format!(
                "principal '{}' has an empty affiliation",
                principal.id
            )));
        }
        if let Some(supersedes) = principal.supersedes.as_deref() {
            require_nonempty(supersedes, "principal supersedes")?;
        }
        let mut roles = BTreeSet::new();
        for role in &principal.roles {
            validate_id(role, "principal role")?;
            require_unique(&mut roles, role, "principal role")?;
        }
    }
    Ok(())
}

pub(super) fn validate_reader_metadata(metadata: &ReaderMetadata) -> Result<()> {
    for (value, name) in [
        (&metadata.scientific_question, "scientific question"),
        (&metadata.assessed_process, "assessed process"),
        (&metadata.assessed_quantity, "assessed quantity"),
        (&metadata.realization, "reader realization"),
    ] {
        require_nonempty(value, name)?;
    }
    validate_relative(&metadata.related_model_narrative)?;
    validate_date(&metadata.manuscript_date, "manuscript date")
}

pub(super) fn validate_review(review: &Review) -> Result<()> {
    let mut producers = BTreeSet::new();
    for producer in &review.material_producer_ids {
        validate_id(producer, "material producer")?;
        require_unique(&mut producers, producer, "material producer")?;
    }
    let mut findings = BTreeSet::new();
    for finding in &review.findings {
        validate_finding(finding, &mut findings)?;
    }
    validate_review_state(review)
}

fn validate_finding(finding: &Finding, findings: &mut BTreeSet<String>) -> Result<()> {
    validate_id(&finding.id, "review finding")?;
    require_unique(findings, &finding.id, "review finding")?;
    require_nonempty(&finding.summary, "finding summary")?;
    require_nonempty(&finding.severity, "finding severity")?;
    validate_finding_disposition(finding)
}

fn validate_finding_disposition(finding: &Finding) -> Result<()> {
    match finding.disposition.as_str() {
        "open" => {
            require_absent(&finding.resolution, "open finding resolution")?;
            require_absent(&finding.verification, "open finding verification")?;
            require_absent(&finding.verifier_id, "open finding verifier")
        }
        "resolved_and_verified" => {
            require_present_nonempty(finding.resolution.as_deref(), "finding resolution")?;
            require_present_nonempty(finding.verification.as_deref(), "finding verification")?;
            require_present_nonempty(finding.verifier_id.as_deref(), "finding verifier")
        }
        "rejected" => {
            require_present_nonempty(finding.rationale.as_deref(), "finding rationale")?;
            require_present_nonempty(finding.verifier_id.as_deref(), "finding verifier")?;
            require_absent(&finding.resolution, "rejected finding resolution")
        }
        _ => Err(AssuranceError::Invalid(format!(
            "review finding '{}' has unsupported disposition",
            finding.id
        ))),
    }
}

fn validate_review_state(review: &Review) -> Result<()> {
    match review.state.as_str() {
        DRAFT => validate_draft_review(review),
        "IN_REVIEW" => validate_in_review(review),
        "APPROVED" => validate_approved_review(review),
        "WITHDRAWN" => validate_terminal_review(review, "withdrawn"),
        "SUPERSEDED" => validate_terminal_review(review, "superseded"),
        _ => Err(AssuranceError::Invalid(
            "review state must be DRAFT, IN_REVIEW, APPROVED, WITHDRAWN, or SUPERSEDED".to_owned(),
        )),
    }
}

fn validate_draft_review(review: &Review) -> Result<()> {
    require_generated_absent(&review.subject_root, "review subject_root")?;
    require_absent(&review.charge, "review charge")?;
    require_absent(&review.build_maintainer_id, "review build maintainer")?;
    require_generated_absent(&review.finding_ledger_root, "review finding_ledger_root")?;
    require_generated_absent(&review.approval_lock_root, "review approval_lock_root")?;
    if review.decision != "not_started"
        || !review.material_producer_ids.is_empty()
        || !review.findings.is_empty()
        || !review.approvals.is_empty()
        || review.independence_assessment != "not_assessed"
    {
        return Err(AssuranceError::Invalid(
            "draft review cannot claim charge, roots, findings, approvals, producers, or independence"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_in_review(review: &Review) -> Result<()> {
    require_generated_absent(&review.subject_root, "review subject_root")?;
    require_present_nonempty(review.charge.as_deref(), "review charge")?;
    require_present_nonempty(
        review.build_maintainer_id.as_deref(),
        "review build maintainer",
    )?;
    require_generated_absent(&review.finding_ledger_root, "review finding_ledger_root")?;
    require_generated_absent(&review.approval_lock_root, "review approval_lock_root")?;
    if review.decision != "pending" || !review.approvals.is_empty() {
        return Err(AssuranceError::Invalid(
            "in-review record must remain pending without approvals".to_owned(),
        ));
    }
    require_nonempty(
        &review.independence_assessment,
        "review independence assessment",
    )
}

fn validate_approved_review(review: &Review) -> Result<()> {
    require_generated_absent(&review.subject_root, "review subject_root")?;
    require_present_nonempty(review.charge.as_deref(), "review charge")?;
    require_present_nonempty(
        review.build_maintainer_id.as_deref(),
        "review build maintainer",
    )?;
    require_generated_absent(&review.finding_ledger_root, "review finding_ledger_root")?;
    require_generated_absent(&review.approval_lock_root, "review approval_lock_root")?;
    if review.decision != "approved" || !review.findings.is_empty() || !review.approvals.is_empty()
    {
        return Err(AssuranceError::Invalid(
            "approved authored review state requires immutable event authority, not embedded findings or approvals"
                .to_owned(),
        ));
    }
    require_nonempty(
        &review.independence_assessment,
        "review independence assessment",
    )
}

fn validate_terminal_review(review: &Review, decision: &str) -> Result<()> {
    require_generated_absent(&review.subject_root, "review subject_root")?;
    require_present_nonempty(review.charge.as_deref(), "review charge")?;
    require_present_nonempty(
        review.build_maintainer_id.as_deref(),
        "review build maintainer",
    )?;
    require_generated_absent(&review.finding_ledger_root, "review finding_ledger_root")?;
    require_generated_absent(&review.approval_lock_root, "review approval_lock_root")?;
    if review.decision != decision || !review.approvals.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "terminal review must record decision '{decision}' without embedded approvals"
        )));
    }
    require_nonempty(
        &review.independence_assessment,
        "review independence assessment",
    )
}

fn require_generated_absent<T>(value: &RequiredNullable<T>, name: &str) -> Result<()> {
    match value {
        RequiredNullable::Missing | RequiredNullable::Null => Ok(()),
        RequiredNullable::Value(_) => Err(AssuranceError::Invalid(format!(
            "generated field '{name}' cannot be stored in authored report source"
        ))),
    }
}

pub(super) fn validate_date(value: &str, kind: &str) -> Result<()> {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return Err(AssuranceError::Invalid(format!(
            "{kind} must use YYYY-MM-DD"
        )));
    }
    let parse = |range: std::ops::Range<usize>| -> Option<u32> {
        std::str::from_utf8(&bytes[range]).ok()?.parse().ok()
    };
    let year = parse(0..4).unwrap_or(0);
    let month = parse(5..7).unwrap_or(0);
    let day = parse(8..10).unwrap_or(0);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };
    if year == 0 || day == 0 || day > days {
        return Err(AssuranceError::Invalid(format!(
            "{kind} must be a valid calendar date"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Finding, validate_finding_disposition};

    #[test]
    fn finding_disposition_contract_covers_every_state() {
        assert!(validate_finding_disposition(&finding("open")).is_ok());
        assert!(validate_finding_disposition(&finding("resolved_and_verified")).is_ok());
        assert!(validate_finding_disposition(&finding("rejected")).is_ok());
        assert!(validate_finding_disposition(&finding("unknown")).is_err());
    }

    fn finding(disposition: &str) -> Finding {
        let (rationale, resolution, verification) = match disposition {
            "resolved_and_verified" => (None, Some("resolution"), Some("verification")),
            "rejected" => (Some("rationale"), None, None),
            _ => (None, None, None),
        };
        serde_json::from_value(serde_json::json!({
            "id": "F-1",
            "summary": "summary",
            "severity": "blocking",
            "disposition": disposition,
            "rationale": rationale,
            "resolution": resolution,
            "verification": verification,
            "verifier_id": if disposition == "open" { None } else { Some("verifier") }
        }))
        .expect("valid finding fixture")
    }
}

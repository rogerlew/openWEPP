use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::Deserialize;

use super::{
    DRAFT, Finding, PRINCIPAL_SCHEMA_VERSION, Review, V2TrustDomain, require_absent,
    require_nonempty, require_present_nonempty, require_unique, validate_digest,
    validate_digest_present, validate_id, validate_relative,
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
    display_name: String,
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
    let mut ids = BTreeSet::new();
    for principal in &registry.principals {
        validate_id(&principal.id, "principal")?;
        require_unique(&mut ids, &principal.id, "principal")?;
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
        _ => Err(AssuranceError::Invalid(
            "review state must be DRAFT, IN_REVIEW, or APPROVED".to_owned(),
        )),
    }
}

fn validate_draft_review(review: &Review) -> Result<()> {
    for (value, label) in [
        (&review.subject_root, "review subject_root"),
        (&review.charge, "review charge"),
        (&review.build_maintainer_id, "review build maintainer"),
        (&review.finding_ledger_root, "review finding_ledger_root"),
        (&review.approval_lock_root, "review approval_lock_root"),
    ] {
        require_absent(value, label)?;
    }
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
    validate_digest_present(&review.subject_root, "review subject_root")?;
    require_present_nonempty(review.charge.as_deref(), "review charge")?;
    require_present_nonempty(
        review.build_maintainer_id.as_deref(),
        "review build maintainer",
    )?;
    validate_digest_present(&review.finding_ledger_root, "review finding_ledger_root")?;
    require_absent(&review.approval_lock_root, "review approval_lock_root")?;
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
    validate_in_review_roots(review)?;
    if review.decision != "approved"
        || review.approvals.len() != 3
        || review
            .findings
            .iter()
            .any(|finding| finding.disposition == "open")
    {
        return Err(AssuranceError::Invalid(
            "approved review requires terminal findings and exactly three approvals".to_owned(),
        ));
    }
    validate_digest_present(&review.approval_lock_root, "review approval_lock_root")?;
    let mut roles = BTreeSet::new();
    let mut principals = BTreeSet::new();
    for approval in &review.approvals {
        if approval.decision != "approved" {
            return Err(AssuranceError::Invalid(
                "review approval decision must be approved".to_owned(),
            ));
        }
        require_unique(&mut roles, &approval.role, "approval role")?;
        require_unique(
            &mut principals,
            &approval.principal_id,
            "approval principal",
        )?;
        validate_digest(&approval.finding_ledger_root, "approval finding ledger")?;
        if review.finding_ledger_root.as_deref() != Some(&approval.finding_ledger_root) {
            return Err(AssuranceError::Invalid(format!(
                "{} approval does not bind the declared finding ledger root",
                approval.role
            )));
        }
        require_nonempty(&approval.competence_basis, "approval competence basis")?;
        require_nonempty(
            &approval.independence_attestation,
            "approval independence attestation",
        )?;
        validate_date(&approval.approved_on, "approval date")?;
    }
    let expected = BTreeSet::from([
        "scientific".to_owned(),
        "reproduction_publication".to_owned(),
        "assurance_steward".to_owned(),
    ]);
    if roles != expected {
        return Err(AssuranceError::Invalid(
            "approved review requires scientific, reproduction_publication, and assurance_steward roles"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_in_review_roots(review: &Review) -> Result<()> {
    validate_digest_present(&review.subject_root, "review subject_root")?;
    require_present_nonempty(review.charge.as_deref(), "review charge")?;
    require_present_nonempty(
        review.build_maintainer_id.as_deref(),
        "review build maintainer",
    )?;
    validate_digest_present(&review.finding_ledger_root, "review finding_ledger_root")?;
    require_nonempty(
        &review.independence_assessment,
        "review independence assessment",
    )
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

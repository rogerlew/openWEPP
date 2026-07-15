use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::engine::{validate_date, validate_digest, validate_id};
use crate::error::{AssuranceError, Result};
use crate::hash::{hash_named_files, sha256_file};
use crate::model::{AnalysisAvailability, AnalysisRepository, AuthoringAnalysis, ReviewState};
use crate::path::{existing_file, validate_relative};

const OUTPUT_ROOT_DOMAIN: &str = "openwepp-assurance-agent-output-v1";

pub(crate) fn validate_authoring(root: &Path, record: &AuthoringAnalysis) -> Result<()> {
    if record.schema_version != 1 {
        return invalid(record, "has an unsupported schema version");
    }
    validate_id(&record.dossier_id, "authoring dossier ID")?;
    validate_date(&record.execution_date)?;
    for (value, label) in [
        (&record.procedure_version, "procedure version"),
        (&record.task, "bounded task"),
        (&record.agent_identity, "agent identity"),
        (&record.tool_version, "agent tool version"),
        (
            &record.nondeterministic_settings,
            "nondeterministic settings",
        ),
        (&record.input_revision, "input revision"),
    ] {
        require_text(record, value, label)?;
        crate::publication::validate_public_scalar(value, label)?;
    }
    validate_inputs(root, record)?;
    let paths = validate_outputs(root, record)?;
    if record.accepted_decisions.is_empty() {
        return invalid(record, "has no accepted analysis or extraction decisions");
    }
    for decision in &record.accepted_decisions {
        crate::publication::validate_public_scalar(decision, "agent-analysis accepted decision")?;
    }
    let observed = hash_named_files(root, &paths, OUTPUT_ROOT_DOMAIN)?;
    validate_digest(
        &record.accepted_output_root_sha256,
        "accepted agent-output root SHA-256",
    )?;
    if observed != record.accepted_output_root_sha256 {
        return invalid(
            record,
            "accepted output root does not match current output bytes",
        );
    }
    validate_authoring_review(record)
}

pub(crate) fn enforce_authoring_lock(
    record: &AuthoringAnalysis,
    lifecycle: crate::model::Lifecycle,
) -> Result<()> {
    if !lifecycle.requires_review_lock() || record.review.state == ReviewState::Approved {
        Ok(())
    } else {
        Err(AssuranceError::ReviewRequired(format!(
            "published dossier '{}' has no approved agent-assisted authoring record",
            record.dossier_id
        )))
    }
}

pub(crate) fn authoring_markdown(record: &AuthoringAnalysis) -> String {
    let review = &record.review;
    let reviewer_role = review
        .reviewer
        .as_ref()
        .map_or("not assigned", |reviewer| reviewer.role.as_str());
    let disposition = review.disposition.as_deref().unwrap_or("not yet disposed");
    let findings = if review.findings.is_empty() {
        "none recorded".to_owned()
    } else {
        review.findings.join("; ")
    };
    let decisions = record
        .accepted_decisions
        .iter()
        .map(|decision| format!("  - {decision}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "- Procedure: `{}`\n- Agent/tool: {} / {}\n- Execution date: `{}`\n- Accepted-output root: `{}`\n- Accepted decisions:\n{}\n- Independent review: `{}` ({})\n- Disposition: {}\n- Findings: {}",
        record.procedure_version,
        record.agent_identity,
        record.tool_version,
        record.execution_date,
        record.accepted_output_root_sha256,
        decisions,
        review.state.label(),
        reviewer_role,
        disposition,
        findings
    )
}

fn validate_inputs(root: &Path, record: &AuthoringAnalysis) -> Result<()> {
    if record.inputs.is_empty() {
        return invalid(record, "has no content-identified inputs");
    }
    let mut identities = BTreeSet::new();
    for input in &record.inputs {
        validate_relative(&input.path, "agent-analysis input")?;
        validate_digest(&input.sha256, "agent-analysis input SHA-256")?;
        require_text(record, &input.revision, "agent-analysis input revision")?;
        require_text(record, &input.role, "agent-analysis input role")?;
        crate::publication::validate_public_scalar(&input.role, "agent-analysis input role")?;
        let identity = format!(
            "{:?}:{}:{}",
            input.repository,
            input.revision,
            input.path.display()
        );
        if !identities.insert(identity) {
            return invalid(record, "contains a duplicate input identity");
        }
        if input.availability == AnalysisAvailability::Tracked {
            if input.repository != AnalysisRepository::Openwepp {
                return invalid(record, "marks a non-openWEPP input as locally tracked");
            }
            let path = existing_file(root, &input.path, "tracked agent-analysis input")?;
            let observed = sha256_file(&path)?;
            if observed != input.sha256 {
                return invalid(record, "tracked input digest does not match current bytes");
            }
        }
    }
    Ok(())
}

fn validate_outputs(root: &Path, record: &AuthoringAnalysis) -> Result<Vec<PathBuf>> {
    if record.accepted_outputs.is_empty() {
        return invalid(record, "has no accepted outputs");
    }
    let mut paths = BTreeSet::new();
    for output in &record.accepted_outputs {
        validate_relative(&output.path, "agent-analysis accepted output")?;
        validate_digest(&output.sha256, "agent-analysis output SHA-256")?;
        require_text(record, &output.role, "agent-analysis output role")?;
        crate::publication::validate_public_scalar(&output.role, "agent-analysis output role")?;
        let path = existing_file(root, &output.path, "agent-analysis accepted output")?;
        if sha256_file(&path)? != output.sha256 {
            return invalid(
                record,
                "accepted output digest does not match current bytes",
            );
        }
        if !paths.insert(output.path.clone()) {
            return invalid(record, "contains a duplicate accepted output");
        }
    }
    Ok(paths.into_iter().collect())
}

fn validate_authoring_review(record: &AuthoringAnalysis) -> Result<()> {
    let review = &record.review;
    for finding in &review.findings {
        crate::publication::validate_public_scalar(finding, "agent-analysis finding")?;
    }
    if let Some(disposition) = &review.disposition {
        crate::publication::validate_public_scalar(disposition, "agent-analysis disposition")?;
    }
    if let Some(date) = &review.review_date {
        validate_date(date)?;
    }
    if let Some(root) = &review.approved_output_root_sha256 {
        validate_digest(root, "agent-analysis approved output root SHA-256")?;
    }
    if let Some(reviewer) = &review.reviewer {
        for (value, label) in [
            (&reviewer.name, "agent-analysis reviewer name"),
            (&reviewer.role, "agent-analysis reviewer role"),
            (&reviewer.expertise, "agent-analysis reviewer expertise"),
            (
                &reviewer.independence_basis,
                "agent-analysis independence basis",
            ),
        ] {
            require_text(record, value, label)?;
            crate::publication::validate_public_scalar(value, label)?;
        }
    }
    if review.state != ReviewState::Approved {
        return Ok(());
    }
    let reviewer = review
        .reviewer
        .as_ref()
        .ok_or_else(|| AssuranceError::Invalid("approved agent analysis has no reviewer".into()))?;
    if !reviewer.independent_of_authors
        || review.review_date.is_none()
        || review
            .disposition
            .as_deref()
            .unwrap_or_default()
            .trim()
            .is_empty()
        || review.approved_output_root_sha256.as_deref()
            != Some(record.accepted_output_root_sha256.as_str())
    {
        return invalid(record, "has an incomplete or non-independent approval");
    }
    Ok(())
}

fn require_text(record: &AuthoringAnalysis, value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty() {
        invalid(record, &format!("has an empty {label}"))
    } else {
        Ok(())
    }
}

fn invalid<T>(record: &AuthoringAnalysis, message: &str) -> Result<T> {
    Err(AssuranceError::Invalid(format!(
        "authoring analysis for '{}' {message}",
        record.dossier_id
    )))
}

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::amendment_support::{read_regular, yaml_key};
use super::identity::ReviewLock;
use crate::{AssuranceError, Result, sha256_bytes};

pub(super) fn manifest_owned_source_drift(
    root: &Path,
    report_id: &str,
    report_path: &Path,
    sources: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>> {
    let report_directory = report_path.parent().ok_or_else(|| {
        AssuranceError::Invalid(format!("report '{report_id}' manifest lacks a parent"))
    })?;
    let mut observed = BTreeMap::new();
    for (source, expected) in sources {
        let path = PathBuf::from(source);
        let Ok(relative) = path.strip_prefix(report_directory) else {
            continue;
        };
        if relative == Path::new("review.lock.json")
            || relative
                .components()
                .next()
                .is_some_and(|component| component.as_os_str() == "review-events")
        {
            continue;
        }
        let digest = sha256_bytes(&read_regular(root, &path)?);
        if digest != *expected {
            observed.insert(source.clone(), digest);
        }
    }
    if !sources.contains_key(
        report_path
            .to_str()
            .ok_or_else(|| AssuranceError::Invalid("report path is not UTF-8".to_owned()))?,
    ) {
        return Err(AssuranceError::Invalid(format!(
            "report '{report_id}' manifest is absent from generated identity"
        )));
    }
    Ok(observed)
}

pub(super) fn requires_adoption_reset_repair(
    report: &serde_yaml::Value,
    review_lock: &ReviewLock,
    report_id: &str,
) -> Result<bool> {
    let review = report
        .get("review")
        .and_then(serde_yaml::Value::as_mapping)
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    let findings = review.get(yaml_key("findings"));
    let approvals = review.get(yaml_key("approvals"));
    if findings.is_none() && approvals.is_none() {
        return Ok(false);
    }
    let empty_sequence = |value: Option<&serde_yaml::Value>| {
        value.is_none_or(|value| value.as_sequence().is_some_and(Vec::is_empty))
    };
    let exact_defective_reset = report.get("lifecycle").and_then(serde_yaml::Value::as_str)
        == Some("DRAFT")
        && report
            .get("agent_assistance")
            .and_then(|value| value.get("review_entry_authorized"))
            .and_then(serde_yaml::Value::as_bool)
            == Some(false)
        && report
            .get("authorship")
            .and_then(|value| value.get("scientific_approver"))
            .is_some_and(serde_yaml::Value::is_null)
        && review
            .get(yaml_key("state"))
            .and_then(serde_yaml::Value::as_str)
            == Some("DRAFT")
        && review
            .get(yaml_key("decision"))
            .and_then(serde_yaml::Value::as_str)
            == Some("not_started")
        && review
            .get(yaml_key("review_charge"))
            .is_some_and(serde_yaml::Value::is_null)
        && review
            .get(yaml_key("build_maintainer_id"))
            .is_some_and(serde_yaml::Value::is_null)
        && review
            .get(yaml_key("material_producer_ids"))
            .and_then(serde_yaml::Value::as_sequence)
            .is_some_and(Vec::is_empty)
        && review
            .get(yaml_key("independence_assessment"))
            .and_then(serde_yaml::Value::as_str)
            == Some("not_assessed")
        && review_lock.event_ids.is_empty()
        && empty_sequence(findings)
        && empty_sequence(approvals);
    if !exact_defective_reset {
        return Err(AssuranceError::Invalid(format!(
            "report '{report_id}' has noncanonical review authority fields outside the repairable DRAFT reset"
        )));
    }
    Ok(true)
}

pub(super) fn require_declared_external_local_content(
    report: &serde_yaml::Value,
    report_id: &str,
    selected: &str,
) -> Result<()> {
    let dependencies = report
        .get("dependencies")
        .and_then(serde_yaml::Value::as_sequence)
        .ok_or_else(|| AssuranceError::Invalid("report dependencies are missing".to_owned()))?;
    let mut matches = dependencies.iter().filter(|dependency| {
        dependency.get("path").and_then(serde_yaml::Value::as_str) == Some(selected)
    });
    let dependency = matches.next().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "source '{selected}' is not declared by report '{report_id}'"
        ))
    })?;
    if matches.next().is_some() {
        return Err(AssuranceError::Invalid(format!(
            "source '{selected}' is declared more than once by report '{report_id}'"
        )));
    }
    if dependency.get("kind").and_then(serde_yaml::Value::as_str) != Some("local_content") {
        return Err(AssuranceError::Invalid(format!(
            "source '{selected}' must be a local_content dependency"
        )));
    }
    if selected.starts_with("assurance/") {
        return Err(AssuranceError::Invalid(format!(
            "adopted report source '{selected}' must be outside assurance"
        )));
    }
    Ok(())
}

pub(super) fn reset_report_to_draft(report: &mut serde_yaml::Value) -> Result<()> {
    let report = report
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("report is not an object".to_owned()))?;
    report.insert(
        yaml_key("lifecycle"),
        serde_yaml::Value::String("DRAFT".to_owned()),
    );
    let assistance = report
        .get_mut(yaml_key("agent_assistance"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report agent_assistance is missing".to_owned()))?;
    assistance.insert(
        yaml_key("review_entry_authorized"),
        serde_yaml::Value::Bool(false),
    );
    let authorship = report
        .get_mut(yaml_key("authorship"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report authorship is missing".to_owned()))?;
    authorship.insert(yaml_key("scientific_approver"), serde_yaml::Value::Null);
    let review = report
        .get_mut(yaml_key("review"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    for (field, value) in [
        ("state", serde_yaml::Value::String("DRAFT".to_owned())),
        (
            "decision",
            serde_yaml::Value::String("not_started".to_owned()),
        ),
        (
            "independence_assessment",
            serde_yaml::Value::String("not_assessed".to_owned()),
        ),
        ("review_charge", serde_yaml::Value::Null),
        ("build_maintainer_id", serde_yaml::Value::Null),
        (
            "material_producer_ids",
            serde_yaml::Value::Sequence(Vec::new()),
        ),
    ] {
        review.insert(yaml_key(field), value);
    }
    review.remove(yaml_key("findings"));
    review.remove(yaml_key("approvals"));
    Ok(())
}

fn apply_role_assignments(report: &mut serde_yaml::Value, request: &V2RoleRequest) -> Result<bool> {
    let mut changed = false;
    if request.assignments.report_lead {
        let authorship = report
            .get_mut("authorship")
            .and_then(serde_yaml::Value::as_mapping_mut)
            .ok_or_else(|| AssuranceError::Invalid("report authorship is missing".to_owned()))?;
        changed |= set_yaml_string(authorship, "human_report_lead", &request.principal_id);
        changed |= set_yaml_string(authorship, "accountability_state", "assigned");
    }
    let review = report
        .get_mut("review")
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    if request.assignments.build_maintainer {
        changed |= set_yaml_string(review, "build_maintainer_id", &request.principal_id);
    }
    if request.assignments.material_producer {
        let key = yaml_key("material_producer_ids");
        let producers = review
            .get_mut(&key)
            .and_then(serde_yaml::Value::as_sequence_mut)
            .ok_or_else(|| {
                AssuranceError::Invalid("material_producer_ids are missing".to_owned())
            })?;
        if !producers
            .iter()
            .any(|value| value.as_str() == Some(&request.principal_id))
        {
            producers.push(serde_yaml::Value::String(request.principal_id.clone()));
            producers.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
            changed = true;
        }
    }
    Ok(changed)
}

fn require_role_eligible_principal(
    root: &Path,
    principal_id: &str,
    assignments: &V2RoleAssignments,
) -> Result<()> {
    let path = PathBuf::from("assurance/v2/principals.yaml");
    let bytes = read_regular(root, &path)?;
    let registry: serde_yaml::Value = parse_yaml(&path, &bytes)?;
    let current = registry
        .get("principals")
        .and_then(serde_yaml::Value::as_sequence)
        .and_then(|principals| {
            principals
                .iter()
                .filter(|principal| {
                    principal.get("id").and_then(serde_yaml::Value::as_str) == Some(principal_id)
                })
                .max_by_key(|principal| {
                    principal
                        .get("record_version")
                        .and_then(serde_yaml::Value::as_u64)
                        .unwrap_or(0)
                })
        })
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown principal '{principal_id}'")))?;
    let roles = current
        .get("roles")
        .and_then(serde_yaml::Value::as_sequence)
        .ok_or_else(|| AssuranceError::Invalid("principal roles are missing".to_owned()))?;
    for (required, selected) in [
        ("report_lead", assignments.report_lead),
        ("material_producer", assignments.material_producer),
        ("build_maintainer", assignments.build_maintainer),
    ] {
        if selected && !roles.iter().any(|role| role.as_str() == Some(required)) {
            return Err(AssuranceError::Invalid(format!(
                "principal '{principal_id}' is not eligible for role '{required}'"
            )));
        }
    }
    Ok(())
}

fn no_op_receipt(
    root: &Path,
    operation: &str,
    impact_class: &str,
    affected_reports: Vec<String>,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    let lock = IdentityLock::load(root)?;
    lock.verify_files(root)?;
    require_expected_generation(&lock, if_generation)?;
    let roots = receipt_roots(root, &affected_reports, &BTreeMap::new(), false)?;
    Ok(V2AmendmentReceipt {
        schema_version: 2,
        operation: operation.to_owned(),
        impact_class: impact_class.to_owned(),
        changed: false,
        old_generation_id: Some(lock.generation_id.clone()),
        new_generation_id: lock.generation_id,
        affected_reports,
        affected_paths: Vec::new(),
        old_roots: Some(roots.clone()),
        new_roots: Some(roots),
        invalidated_authority: Vec::new(),
        gate_ids: Vec::new(),
        gate_argv: Vec::new(),
    })
}

fn require_expected_generation(lock: &IdentityLock, expected: Option<&str>) -> Result<()> {
    if let Some(expected) = expected
        && lock.generation_id != expected
    {
        return Err(AssuranceError::Drift(format!(
            "amendment compare-and-swap rejected stale generation '{expected}'"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::exact_relative_path;

    #[test]
    fn source_paths_require_exact_repository_relative_spelling() {
        assert!(exact_relative_path("assurance/evidence.json").is_ok());
        for alias in [
            "assurance//evidence.json",
            "assurance/./evidence.json",
            "assurance/evidence.json/",
            "../evidence.json",
            "/assurance/evidence.json",
        ] {
            assert!(exact_relative_path(alias).is_err(), "must reject {alias}");
        }
    }
}

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::error::{AssuranceError, Result};
use crate::hash::sha256_bytes;
use crate::model::{Lifecycle, Review, ReviewApproval, ReviewAuthor, ReviewScope, ReviewState};

pub(crate) fn validate_review(review: &Review) -> Result<()> {
    if review.conclusion_authors.is_empty() || review.approvals.is_empty() {
        return invalid(review, "requires conclusion authors and approval history");
    }
    validate_authors(review)?;
    let mut ids = BTreeSet::new();
    for approval in &review.approvals {
        validate_approval(review, approval, &mut ids)?;
    }
    validate_payload_history(review, false)?;
    for scope in [ReviewScope::Scientific, ReviewScope::Publication] {
        if current_approval(review, scope).is_none() {
            return invalid(review, &format!("has no {} review entry", scope.label()));
        }
    }
    Ok(())
}

pub(crate) fn enforce_review_lock(
    review: &Review,
    lifecycle: Lifecycle,
    scientific_root: &str,
    publication_root: &str,
) -> Result<()> {
    if !lifecycle.requires_review_lock() {
        return Ok(());
    }
    validate_payload_history(review, true).map_err(|error| {
        AssuranceError::ReviewRequired(format!(
            "dossier '{}' has an invalid approval history: {error}",
            review.dossier_id
        ))
    })?;
    if current_approval_index(review, ReviewScope::Publication)
        != review.approvals.len().checked_sub(1)
    {
        return Err(AssuranceError::ReviewRequired(format!(
            "dossier '{}' publication approval is not the terminal history entry",
            review.dossier_id
        )));
    }
    enforce_scope(review, ReviewScope::Scientific, scientific_root)?;
    enforce_scope(review, ReviewScope::Publication, publication_root)
}

pub(crate) fn review_implication(
    review: &Review,
    lifecycle: Lifecycle,
    scientific_root: &str,
    publication_root: &str,
) -> String {
    let scientific = scope_implication(review, ReviewScope::Scientific, scientific_root);
    let mut publication = scope_implication(review, ReviewScope::Publication, publication_root);
    if lifecycle.requires_review_lock()
        && (validate_payload_history(review, true).is_err()
            || current_approval_index(review, ReviewScope::Publication)
                != review.approvals.len().checked_sub(1))
    {
        publication = "history_review_required";
    }
    if lifecycle.requires_review_lock() && (scientific != "valid" || publication != "valid") {
        format!("review_required(scientific={scientific},publication={publication})")
    } else {
        format!("scientific={scientific},publication={publication}")
    }
}

pub(crate) fn approval_payloads(review: &Review) -> Result<BTreeMap<String, String>> {
    review
        .approvals
        .iter()
        .enumerate()
        .map(|(index, approval)| {
            payload_digest(review, index).map(|digest| (approval.review_id.clone(), digest))
        })
        .collect()
}

pub(crate) fn review_markdown(review: &Review) -> String {
    let mut rows = vec![
        "| Scope | Review ID | State | Reviewer role | Disposition |".to_owned(),
        "| --- | --- | --- | --- | --- |".to_owned(),
    ];
    for approval in &review.approvals {
        let roles = approval
            .reviewers
            .iter()
            .map(|reviewer| reviewer.role.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        rows.push(format!(
            "| {} | `{}` | `{}` | {} | {} |",
            approval.scope.label(),
            approval.review_id,
            approval.state.label(),
            table_text(&roles),
            table_text(&approval.disposition_summary)
        ));
        for finding in &approval.findings {
            rows.push(format!(
                "|  | finding `{}` | {} / {} | {} | {} |",
                finding.finding_id,
                finding.severity.label(),
                finding.disposition.label(),
                if finding.resolved { "resolved" } else { "open" },
                table_text(&finding.summary)
            ));
        }
    }
    rows.join("\n")
}

fn validate_authors(review: &Review) -> Result<()> {
    let mut names = BTreeSet::new();
    for author in &review.conclusion_authors {
        require_text(&author.name, "conclusion author name")?;
        require_text(&author.role, "conclusion author role")?;
        if !names.insert(author.name.trim()) {
            return invalid(review, "contains a duplicate conclusion author");
        }
    }
    Ok(())
}

fn validate_approval(
    review: &Review,
    approval: &ReviewApproval,
    ids: &mut BTreeSet<String>,
) -> Result<()> {
    validate_id(&approval.review_id, "review ID")?;
    if !ids.insert(approval.review_id.clone()) {
        return invalid(review, "contains a duplicate review ID");
    }
    require_text(&approval.disposition_summary, "review disposition")?;
    for disagreement in &approval.residual_disagreements {
        require_text(disagreement, "residual review disagreement")?;
    }
    validate_reviewers(review, approval)?;
    validate_findings(approval)?;
    if let Some(root) = &approval.reviewed_root_sha256 {
        validate_digest(root, "reviewed root SHA-256")?;
    }
    if let Some(payload) = &approval.payload_sha256 {
        validate_digest(payload, "review payload SHA-256")?;
    }
    if approval.state != ReviewState::Approved && approval.payload_sha256.is_some() {
        return invalid(review, "has a non-approved entry with an approval payload");
    }
    if let Some(date) = &approval.review_date {
        crate::engine::validate_date(date)?;
    }
    if approval.state == ReviewState::Approved {
        validate_approved(review, approval)?;
    }
    Ok(())
}

fn validate_approved(review: &Review, approval: &ReviewApproval) -> Result<()> {
    if approval.reviewers.is_empty()
        || approval.review_date.is_none()
        || approval.reviewed_root_sha256.is_none()
    {
        return invalid(
            review,
            "has an approved entry without reviewers, date, and root",
        );
    }
    for reviewer in &approval.reviewers {
        let is_author = review
            .conclusion_authors
            .iter()
            .any(|author| same_identity(author, &reviewer.name));
        if !reviewer.independent_of_authors || is_author {
            return invalid(review, "contains self-approval or undisclosed dependence");
        }
    }
    if approval
        .findings
        .iter()
        .any(|finding| finding.closure_blocking && !finding.resolved)
    {
        return invalid(review, "approves a closure-blocking unresolved finding");
    }
    Ok(())
}

fn validate_reviewers(review: &Review, approval: &ReviewApproval) -> Result<()> {
    let mut names = BTreeSet::new();
    for reviewer in &approval.reviewers {
        for (value, label) in [
            (&reviewer.name, "reviewer name"),
            (&reviewer.role, "reviewer role"),
            (&reviewer.expertise, "reviewer expertise"),
            (&reviewer.independence_basis, "reviewer independence basis"),
        ] {
            require_text(value, label)?;
        }
        if !names.insert(reviewer.name.trim()) {
            return invalid(
                review,
                &format!(
                    "review '{}' contains a duplicate reviewer",
                    approval.review_id
                ),
            );
        }
    }
    Ok(())
}

fn validate_findings(approval: &ReviewApproval) -> Result<()> {
    let mut ids = BTreeSet::new();
    for finding in &approval.findings {
        validate_id(&finding.finding_id, "review finding ID")?;
        if !ids.insert(&finding.finding_id) {
            return Err(AssuranceError::Invalid(format!(
                "review '{}' contains duplicate finding '{}'",
                approval.review_id, finding.finding_id
            )));
        }
        for (value, label) in [
            (&finding.summary, "review finding summary"),
            (&finding.rationale, "review finding rationale"),
        ] {
            require_text(value, label)?;
        }
    }
    Ok(())
}

fn enforce_scope(review: &Review, scope: ReviewScope, expected_root: &str) -> Result<()> {
    let index = current_approval_index(review, scope).ok_or_else(|| {
        AssuranceError::ReviewRequired(format!(
            "dossier '{}' has no current {} review",
            review.dossier_id,
            scope.label()
        ))
    })?;
    let approval = &review.approvals[index];
    let expected_payload = payload_digest(review, index)?;
    let matches = approval.state == ReviewState::Approved
        && approval.reviewed_root_sha256.as_deref() == Some(expected_root)
        && approval.payload_sha256.as_deref() == Some(expected_payload.as_str());
    if matches {
        Ok(())
    } else {
        Err(AssuranceError::ReviewRequired(format!(
            "dossier '{}' {} review '{}' does not bind root {}",
            review.dossier_id,
            scope.label(),
            approval.review_id,
            expected_root
        )))
    }
}

fn scope_implication(review: &Review, scope: ReviewScope, expected_root: &str) -> &'static str {
    let Some(index) = current_approval_index(review, scope) else {
        return "missing";
    };
    let approval = &review.approvals[index];
    if approval.state == ReviewState::Approved
        && approval.reviewed_root_sha256.as_deref() == Some(expected_root)
        && approval.payload_sha256.as_deref() == payload_digest(review, index).ok().as_deref()
    {
        "valid"
    } else if approval.reviewed_root_sha256.is_some() {
        "review_required"
    } else {
        "pending"
    }
}

fn current_approval(review: &Review, scope: ReviewScope) -> Option<&ReviewApproval> {
    current_approval_index(review, scope).map(|index| &review.approvals[index])
}

fn current_approval_index(review: &Review, scope: ReviewScope) -> Option<usize> {
    review
        .approvals
        .iter()
        .rposition(|approval| approval.scope == scope)
}

fn validate_payload_history(review: &Review, require_all: bool) -> Result<()> {
    for (index, approval) in review.approvals.iter().enumerate() {
        if approval.state != ReviewState::Approved {
            continue;
        }
        let expected = payload_digest(review, index)?;
        match approval.payload_sha256.as_deref() {
            Some(observed) if observed == expected => {}
            None if !require_all
                && current_approval_index(review, approval.scope) == Some(index) => {}
            Some(_) => {
                return invalid(
                    review,
                    &format!(
                        "has an approved entry '{}' whose history payload does not match",
                        approval.review_id
                    ),
                );
            }
            None => {
                return invalid(
                    review,
                    &format!(
                        "has an approved historical entry '{}' without a history payload",
                        approval.review_id
                    ),
                );
            }
        }
    }
    Ok(())
}

fn payload_digest(review: &Review, index: usize) -> Result<String> {
    let approval_history = review.approvals[..=index]
        .iter()
        .map(ApprovalPayload::from)
        .collect::<Vec<_>>();
    let payload = ReviewPayload {
        domain: "openwepp-assurance-review-history-payload-v3",
        dossier_id: &review.dossier_id,
        dossier_version: &review.dossier_version,
        conclusion_authors: &review.conclusion_authors,
        approval_history,
    };
    let bytes = serde_json::to_vec(&payload).map_err(|error| {
        AssuranceError::Invalid(format!("failed to serialize review payload: {error}"))
    })?;
    Ok(sha256_bytes(&bytes))
}

fn same_identity(author: &ReviewAuthor, reviewer: &str) -> bool {
    author.name.trim().eq_ignore_ascii_case(reviewer.trim())
}

fn require_text(value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(AssuranceError::Invalid(format!("{label} must be nonempty")))
    } else {
        crate::publication::validate_public_scalar(value, label)
    }
}

fn validate_id(value: &str, label: &str) -> Result<()> {
    crate::engine::validate_id(value, label)
}

fn validate_digest(value: &str, label: &str) -> Result<()> {
    crate::engine::validate_digest(value, label)
}

fn invalid(review: &Review, message: &str) -> Result<()> {
    Err(AssuranceError::Invalid(format!(
        "review for '{}' {message}",
        review.dossier_id
    )))
}

fn table_text(value: &str) -> String {
    if value.trim().is_empty() {
        "not assigned".to_owned()
    } else {
        value.replace('|', "\\|")
    }
}

#[derive(Serialize)]
struct ReviewPayload<'a> {
    domain: &'static str,
    dossier_id: &'a str,
    dossier_version: &'a str,
    conclusion_authors: &'a [ReviewAuthor],
    approval_history: Vec<ApprovalPayload<'a>>,
}

#[derive(Serialize)]
struct ApprovalPayload<'a> {
    review_id: &'a str,
    scope: ReviewScope,
    state: ReviewState,
    reviewers: &'a [crate::model::ReviewParticipant],
    review_date: Option<&'a str>,
    reviewed_root_sha256: Option<&'a str>,
    disposition_summary: &'a str,
    findings: &'a [crate::model::ReviewFinding],
    residual_disagreements: &'a [String],
}

impl<'a> From<&'a ReviewApproval> for ApprovalPayload<'a> {
    fn from(approval: &'a ReviewApproval) -> Self {
        Self {
            review_id: &approval.review_id,
            scope: approval.scope,
            state: approval.state,
            reviewers: &approval.reviewers,
            review_date: approval.review_date.as_deref(),
            reviewed_root_sha256: approval.reviewed_root_sha256.as_deref(),
            disposition_summary: &approval.disposition_summary,
            findings: &approval.findings,
            residual_disagreements: &approval.residual_disagreements,
        }
    }
}

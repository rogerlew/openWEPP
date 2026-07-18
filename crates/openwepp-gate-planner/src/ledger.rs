use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::canonical::{derived_id, digest, parse_strict, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};

/// Verify append-only campaign ancestry, transitions, closure, and identity.
///
/// # Errors
///
/// Returns a typed ledger error for any schema, identity, ancestry, fold, or closure mismatch.
#[allow(
    clippy::too_many_lines,
    reason = "linear audit keeps the ledger checks in authority order"
)]
pub fn verify_campaign_ledger(
    repo: &Path,
    ledger: &Value,
    predecessor: Option<&Value>,
) -> Result<()> {
    validate_document(repo, "campaign-ledger", ledger)?;
    verify_id(ledger, "ledger_id", "GATE-LEDGER-ID")?;
    verify_predecessor(ledger, predecessor)?;

    let events = array(ledger, "/events")?;
    verify_event_chain(ledger, events)?;
    verify_campaign_event_folds(ledger, events)?;
    let declared = declared_obligations(array(ledger, "/obligations")?)?;
    let folded = fold_obligations(events)?;
    verify_obligation_fold(&declared, &folded)?;
    verify_ledger_references(ledger, events)?;
    verify_campaign_closure(ledger, &declared)
}

fn verify_predecessor(ledger: &Value, predecessor: Option<&Value>) -> Result<()> {
    match predecessor {
        Some(previous) => verify_previous_ledger(ledger, previous),
        None if !ledger["predecessor_ledger_id"].is_null() => Err(ledger_error(
            "GATE-LEDGER-PREDECESSOR",
            "predecessor is required",
        )),
        None => Ok(()),
    }
}

fn verify_previous_ledger(ledger: &Value, previous: &Value) -> Result<()> {
    verify_id(previous, "ledger_id", "GATE-LEDGER-PREDECESSOR-ID")?;
    if previous["campaign"]["campaign_id"] != ledger["campaign"]["campaign_id"] {
        return Err(ledger_error(
            "GATE-LEDGER-PREDECESSOR-CAMPAIGN",
            "predecessor campaign differs",
        ));
    }
    require_ledger_cas(ledger["predecessor_ledger_id"] == previous["ledger_id"])?;
    require_ledger_cas(ledger["expected_predecessor_head"] == previous["current_head"])
}

fn require_ledger_cas(matches: bool) -> Result<()> {
    if matches {
        Ok(())
    } else {
        Err(ledger_error(
            "GATE-LEDGER-CAS",
            "predecessor ledger/head mismatch",
        ))
    }
}

fn verify_event_chain(ledger: &Value, events: &[Value]) -> Result<()> {
    let mut predecessor_event: Option<&str> = None;
    let mut event_ids = BTreeSet::new();
    let mut folded_head: Option<&str> = None;
    for event in events {
        verify_id(event, "event_id", "GATE-LEDGER-EVENT-ID")?;
        let event_id = string(event, "/event_id")?;
        if !event_ids.insert(event_id) {
            return Err(ledger_error("GATE-LEDGER-EVENT-DUPLICATE", event_id));
        }
        if event
            .pointer("/predecessor_event_id")
            .and_then(Value::as_str)
            != predecessor_event
        {
            return Err(ledger_error("GATE-LEDGER-EVENT-ANCESTRY", event_id));
        }
        let target_head = string(event, "/target_head")?;
        if event["event_type"] == "CAMPAIGN_CREATED" {
            if folded_head.is_some() {
                return Err(ledger_error("GATE-LEDGER-EVENT-TARGET", event_id));
            }
            folded_head = Some(target_head);
        } else if event["event_type"] == "HEAD_ADVANCED" {
            let current =
                folded_head.ok_or_else(|| ledger_error("GATE-LEDGER-EVENT-TARGET", event_id))?;
            folded_head = Some(fold_head_advance(event, current)?);
        } else if folded_head != Some(target_head) {
            return Err(ledger_error("GATE-LEDGER-EVENT-TARGET", event_id));
        }
        predecessor_event = Some(event_id);
    }
    if folded_head == ledger["current_head"].as_str() {
        Ok(())
    } else {
        Err(ledger_error(
            "GATE-LEDGER-EVENT-TARGET",
            "folded event head differs from ledger",
        ))
    }
}

fn verify_campaign_event_folds(ledger: &Value, events: &[Value]) -> Result<()> {
    let first = events
        .first()
        .ok_or_else(|| ledger_error("GATE-LEDGER-CAMPAIGN-CREATED", "missing creation event"))?;
    if first["event_type"] != "CAMPAIGN_CREATED"
        || first["payload"]["campaign_id"] != ledger["campaign"]["campaign_id"]
    {
        return Err(ledger_error(
            "GATE-LEDGER-CAMPAIGN-CREATED",
            "first event is not exact campaign creation",
        ));
    }
    let mut lifecycle = "ACTIVE";
    let mut head = first["target_head"]
        .as_str()
        .ok_or_else(|| ledger_error("GATE-LEDGER-HEAD-FOLD", "creation target head is missing"))?;
    for event in events.iter().skip(1) {
        if event["event_type"] == "HEAD_ADVANCED" {
            head = fold_head_advance(event, head)?;
        }
        if event["event_type"] == "LIFECYCLE_TRANSITION" {
            lifecycle = fold_lifecycle_transition(event, lifecycle)?;
        }
    }
    if ledger["current_head"] != head || ledger["lifecycle"] != lifecycle {
        Err(ledger_error(
            "GATE-LEDGER-EVENT-FOLD",
            "event fold differs from ledger head or lifecycle",
        ))
    } else {
        Ok(())
    }
}

fn fold_head_advance<'a>(event: &'a Value, current: &str) -> Result<&'a str> {
    let payload = &event["payload"];
    if payload["from_head"] != current || payload["to_head"] != event["target_head"] {
        return Err(ledger_error("GATE-LEDGER-HEAD-FOLD", current));
    }
    string(payload, "/to_head")
}

fn fold_lifecycle_transition<'a>(event: &'a Value, current: &str) -> Result<&'a str> {
    let payload = &event["payload"];
    let from = string(payload, "/from_lifecycle")?;
    let to = string(payload, "/to_lifecycle")?;
    if from == current && allowed_lifecycle_transition(from, to) {
        Ok(to)
    } else {
        Err(ledger_error(
            "GATE-LEDGER-LIFECYCLE-FOLD",
            format!("{from} -> {to}"),
        ))
    }
}

fn allowed_lifecycle_transition(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("ACTIVE", "CLOSING" | "ABORTED" | "SUPERSEDED")
            | ("CLOSING", "CERTIFIED" | "ACTIVE" | "ABORTED" | "SUPERSEDED")
    )
}

fn declared_obligations(obligations: &[Value]) -> Result<BTreeMap<&str, &str>> {
    let mut declared = BTreeMap::new();
    for obligation in obligations {
        let id = string(obligation, "/obligation_id")?;
        if declared.insert(id, string(obligation, "/state")?).is_some() {
            return Err(ledger_error("GATE-LEDGER-OBLIGATION-DUPLICATE", id));
        }
    }
    Ok(declared)
}

fn fold_obligations(events: &[Value]) -> Result<BTreeMap<&str, &str>> {
    let mut folded: BTreeMap<&str, &str> = BTreeMap::new();
    for event in events
        .iter()
        .filter(|event| event["event_type"] == "OBLIGATION_TRANSITION")
    {
        fold_transition(&mut folded, &event["payload"])?;
    }
    Ok(folded)
}

fn fold_transition<'a>(folded: &mut BTreeMap<&'a str, &'a str>, payload: &'a Value) -> Result<()> {
    let id = string(payload, "/obligation_id")?;
    let from = payload.pointer("/from_state").and_then(Value::as_str);
    let to = string(payload, "/to_state")?;
    if folded.get(id).copied().or(from) != from || !allowed_transition(from, to) {
        return Err(ledger_error(
            "GATE-LEDGER-TRANSITION",
            format!("{id}: {from:?} -> {to}"),
        ));
    }
    if to == "PASS" && payload["receipt_id"].is_null() {
        return Err(ledger_error("GATE-LEDGER-PASS-RECEIPT", id));
    }
    folded.insert(id, to);
    Ok(())
}

fn verify_obligation_fold(
    declared: &BTreeMap<&str, &str>,
    folded: &BTreeMap<&str, &str>,
) -> Result<()> {
    for (id, state) in folded {
        if declared.get(id).copied() != Some(*state) {
            return Err(ledger_error("GATE-LEDGER-FOLD", *id));
        }
    }
    for (id, state) in declared {
        if *state != "PENDING" && !folded.contains_key(id) {
            return Err(ledger_error("GATE-LEDGER-FOLD-MISSING", *id));
        }
    }
    Ok(())
}

fn verify_ledger_references(ledger: &Value, events: &[Value]) -> Result<()> {
    let receipt_refs = receipt_references(ledger)?;
    verify_execution_events(events, &receipt_refs)?;
    verify_passing_obligations(ledger, events, &receipt_refs)?;
    verify_authorizations(ledger)?;
    verify_backstop_references(ledger, &receipt_refs)?;
    verify_certification_references(ledger, &receipt_refs)
}

fn receipt_references(ledger: &Value) -> Result<BTreeMap<&str, &Value>> {
    let mut references = BTreeMap::new();
    for reference in array(ledger, "/receipts")? {
        let id = string(reference, "/receipt_id")?;
        if references.insert(id, reference).is_some() {
            return Err(ledger_error("GATE-LEDGER-RECEIPT-DUPLICATE", id));
        }
    }
    Ok(references)
}

fn verify_execution_events(events: &[Value], receipt_refs: &BTreeMap<&str, &Value>) -> Result<()> {
    for event in events
        .iter()
        .filter(|event| event["event_type"] == "EXECUTION_RECORDED")
    {
        let payload = &event["payload"];
        let receipt_id = string(payload, "/receipt_id")?;
        let reference = receipt_refs
            .get(receipt_id)
            .ok_or_else(|| ledger_error("GATE-LEDGER-EXECUTION-RECEIPT", receipt_id))?;
        if payload["envelope_id"] != reference["envelope_id"] {
            return Err(ledger_error("GATE-LEDGER-EXECUTION-ENVELOPE", receipt_id));
        }
        let obligation_id = string(payload, "/obligation_id")?;
        if !array(reference, "/obligation_ids")?.contains(&Value::String(obligation_id.to_owned()))
        {
            return Err(ledger_error(
                "GATE-LEDGER-EXECUTION-OBLIGATION",
                obligation_id,
            ));
        }
    }
    Ok(())
}

fn verify_passing_obligations(
    ledger: &Value,
    events: &[Value],
    receipt_refs: &BTreeMap<&str, &Value>,
) -> Result<()> {
    let Some(obligation) = array(ledger, "/obligations")?
        .iter()
        .find(|obligation| obligation["state"] == "PASS")
    else {
        return Ok(());
    };
    let obligation_id = string(obligation, "/obligation_id")?;
    let receipt_id = string(obligation, "/receipt_id")?;
    let reference = receipt_refs
        .get(receipt_id)
        .ok_or_else(|| ledger_error("GATE-LEDGER-PASS-RECEIPT", obligation_id))?;
    let envelope_id = &reference["envelope_id"];
    let matching_execution = events.iter().any(|event| {
        event["event_type"] == "EXECUTION_RECORDED"
            && event["payload"]["obligation_id"] == obligation_id
            && event["payload"]["receipt_id"] == receipt_id
            && event["payload"]["envelope_id"] == *envelope_id
            && matches!(
                event["payload"]["result"].as_str(),
                Some("PASS" | "PASS_WITH_RETRY")
            )
    });
    let matching_transition = events.iter().any(|event| {
        event["event_type"] == "OBLIGATION_TRANSITION"
            && event["payload"]["obligation_id"] == obligation_id
            && event["payload"]["to_state"] == "PASS"
            && event["payload"]["receipt_id"] == receipt_id
    });
    if !matching_execution || !matching_transition {
        return Err(ledger_error("GATE-LEDGER-PASS-EXECUTION", obligation_id));
    }
    Err(ledger_error(
        "GATE-LEDGER-PASS-UNAUTHENTICATED",
        "v1 requires an in-process verified receipt/envelope capability before PASS",
    ))
}

fn verify_authorizations(ledger: &Value) -> Result<()> {
    let mut ids = BTreeSet::new();
    let Some(authorization) = array(ledger, "/authorization_events")?.first() else {
        return Ok(());
    };
    verify_id(
        authorization,
        "authorization_id",
        "GATE-LEDGER-AUTHORIZATION-ID",
    )?;
    verify_authorization_binding(ledger, authorization, &mut ids)?;
    Err(ledger_error(
        "GATE-LEDGER-AUTHORIZATION-UNAUTHENTICATED",
        "self-declared authorization cannot establish authority",
    ))
}

fn verify_authorization_binding<'a>(
    ledger: &Value,
    authorization: &'a Value,
    ids: &mut BTreeSet<&'a str>,
) -> Result<()> {
    let id = string(authorization, "/authorization_id")?;
    require_authorization_binding(ids.insert(id), id)?;
    require_authorization_binding(
        authorization["campaign_id"] == ledger["campaign"]["campaign_id"],
        id,
    )?;
    require_authorization_binding(authorization["target_head"] == ledger["current_head"], id)?;
    require_authorization_binding(
        authorization["predecessor_ledger_id"] == ledger["predecessor_ledger_id"],
        id,
    )
}

fn require_authorization_binding(matches: bool, id: &str) -> Result<()> {
    if matches {
        Ok(())
    } else {
        Err(ledger_error("GATE-LEDGER-AUTHORIZATION", id))
    }
}

fn verify_backstop_references(ledger: &Value, receipt_refs: &BTreeMap<&str, &Value>) -> Result<()> {
    let backstop = &ledger["backstop"];
    if backstop["state"] != "CURRENT" {
        return Ok(());
    }
    let receipt_id = string(backstop, "/anchor_receipt_id")?;
    let reference = receipt_refs
        .get(receipt_id)
        .ok_or_else(|| ledger_error("GATE-LEDGER-BACKSTOP-RECEIPT", receipt_id))?;
    if backstop["anchor_envelope_id"] != reference["envelope_id"] {
        return Err(ledger_error("GATE-LEDGER-BACKSTOP-ENVELOPE", receipt_id));
    }
    Err(ledger_error(
        "GATE-LEDGER-BACKSTOP-UNAUTHENTICATED",
        "current backstop requires a verified protected envelope capability",
    ))
}

fn verify_certification_references(
    ledger: &Value,
    receipt_refs: &BTreeMap<&str, &Value>,
) -> Result<()> {
    let Some(certification) = ledger["certification"].as_object() else {
        return Ok(());
    };
    let receipt_id = certification["receipt_id"]
        .as_str()
        .ok_or_else(|| ledger_error("GATE-LEDGER-CERTIFICATION", "receipt"))?;
    let reference = receipt_refs
        .get(receipt_id)
        .ok_or_else(|| ledger_error("GATE-LEDGER-CERTIFICATION-RECEIPT", receipt_id))?;
    let authorization_id = certification["authorization_id"]
        .as_str()
        .ok_or_else(|| ledger_error("GATE-LEDGER-CERTIFICATION", "authorization"))?;
    let authorization = array(ledger, "/authorization_events")?
        .iter()
        .find(|authorization| authorization["authorization_id"] == authorization_id);
    if certification_binding_matches(certification, reference, authorization) {
        Err(ledger_error(
            "GATE-LEDGER-CERTIFICATION-UNAUTHENTICATED",
            "certification requires verified receipt, envelope, and role capabilities",
        ))
    } else {
        Err(ledger_error(
            "GATE-LEDGER-CERTIFICATION-REFERENCE",
            receipt_id,
        ))
    }
}

fn certification_binding_matches(
    certification: &serde_json::Map<String, Value>,
    reference: &Value,
    authorization: Option<&Value>,
) -> bool {
    if certification["envelope_id"] != reference["envelope_id"] {
        return false;
    }
    let Some(authorization) = authorization else {
        return false;
    };
    if authorization["envelope_id"] != certification["envelope_id"] {
        return false;
    }
    authorization["target_head"] == certification["certified_head"]
}

fn verify_campaign_closure(ledger: &Value, declared: &BTreeMap<&str, &str>) -> Result<()> {
    let lifecycle = string(ledger, "/lifecycle")?;
    if matches!(lifecycle, "CERTIFIED" | "CLOSING")
        && declared.values().any(|state| {
            matches!(
                *state,
                "PENDING" | "FAIL" | "BLOCKED" | "STALE" | "DEFERRED" | "LEGACY_UNVERIFIED"
            )
        })
    {
        return Err(ledger_error(
            "GATE-LEDGER-CLOSURE",
            "open obligations remain",
        ));
    }
    verify_certification(ledger, lifecycle)
}

fn verify_certification(ledger: &Value, lifecycle: &str) -> Result<()> {
    if let Some(certification) = ledger["certification"].as_object() {
        if certification.get("certified_head") != Some(&ledger["current_head"])
            || lifecycle != "CERTIFIED"
        {
            return Err(ledger_error(
                "GATE-LEDGER-CERTIFICATION",
                "certification is not exact-head CERTIFIED",
            ));
        }
    } else if lifecycle == "CERTIFIED" {
        return Err(ledger_error(
            "GATE-LEDGER-CERTIFICATION",
            "CERTIFIED ledger lacks certification",
        ));
    }
    Ok(())
}

/// Verify assurance entry identities, exact targets, aggregate fold, and currency.
///
/// # Errors
///
/// Returns a typed ledger error when the record cannot be reconstructed exactly.
pub fn verify_assurance_impact(repo: &Path, record: &Value) -> Result<()> {
    validate_document(repo, "assurance-impact", record)?;
    verify_id(record, "record_id", "GATE-ASSURANCE-RECORD-ID")?;
    let target = &record["campaign_head"];
    let raw_entries = array(record, "/entries")?;
    let fold = assurance_entries(raw_entries, target)?;
    verify_assurance_replacements(&fold.entries)?;
    verify_assurance_events(record, raw_entries)?;
    let aggregate = if assurance_invalidation_active(record, array(record, "/events")?) {
        "IMPACT_PENDING"
    } else {
        aggregate_impact(fold.object_states.values().map(|state| Ok(*state)))?
    };
    verify_assurance_axes(record, aggregate)
}

struct AssuranceFold<'a> {
    entries: BTreeMap<&'a str, &'a Value>,
    object_states: BTreeMap<String, &'a str>,
}

fn assurance_entries<'a>(raw_entries: &'a [Value], target: &Value) -> Result<AssuranceFold<'a>> {
    let mut entries = BTreeMap::new();
    let mut predecessor_event: Option<&str> = None;
    let mut object_states = BTreeMap::<String, &str>::new();
    for entry in raw_entries {
        verify_id(entry, "impact_entry_id", "GATE-ASSURANCE-ENTRY-ID")?;
        let id = string(entry, "/impact_entry_id")?;
        if entries.insert(id, entry).is_some() {
            return Err(ledger_error("GATE-ASSURANCE-ENTRY-DUPLICATE", id));
        }
        if entry["target_head"] != *target {
            return Err(ledger_error("GATE-ASSURANCE-TARGET", id));
        }
        let subject_id = string(entry, "/impact_subject_id")?;
        if subject_id != assurance_subject_id(entry)? {
            return Err(ledger_error("GATE-ASSURANCE-SUBJECT-ID", subject_id));
        }
        let previous_state = object_states.get(subject_id).copied();
        let next_state = string(entry, "/state")?;
        if entry["predecessor_event_id"].as_str() != predecessor_event
            || entry["previous_state"].as_str() != previous_state
            || !allowed_assurance_transition(
                previous_state,
                string(entry, "/event_type")?,
                next_state,
            )
        {
            return Err(ledger_error("GATE-ASSURANCE-ANCESTRY", id));
        }
        predecessor_event = Some(string(entry, "/event_id")?);
        object_states.insert(subject_id.to_owned(), next_state);
        if matches!(
            next_state,
            "NO_MATERIAL_IMPACT_AUTHORIZED" | "REFRESH_COMPLETE" | "SUPERSEDED" | "WITHDRAWN"
        ) {
            return Err(ledger_error(
                "GATE-ASSURANCE-RESOLUTION-UNAUTHENTICATED",
                "terminal resolution, supersession, or withdrawal requires verified lifecycle authority",
            ));
        }
    }
    Ok(AssuranceFold {
        entries,
        object_states,
    })
}

fn assurance_subject_id(entry: &Value) -> Result<String> {
    digest(&json!({
        "predecessor_ledger_id": entry["predecessor_ledger_id"],
        "terminal_plan_id": entry["terminal_plan_id"],
        "changed_object": entry["changed_object"],
        "matching_watch_ids": entry["matching_watch_ids"],
        "report_root": entry["report_root"],
        "target_head": entry["target_head"]
    }))
}

fn allowed_assurance_transition(previous: Option<&str>, event: &str, next: &str) -> bool {
    matches!(
        (previous, event, next),
        (
            None,
            "IMPACT_DISCOVERED",
            "OPEN_UNKNOWN" | "OPEN_ASSESSMENT"
        ) | (
            Some("OPEN_UNKNOWN" | "OPEN_ASSESSMENT" | "REFRESH_REQUIRED"),
            "ASSESSMENT_RECORDED",
            "OPEN_ASSESSMENT" | "REFRESH_REQUIRED"
        ) | (
            Some("OPEN_UNKNOWN" | "OPEN_ASSESSMENT" | "REFRESH_REQUIRED"),
            "RESOLUTION_RECORDED",
            "NO_MATERIAL_IMPACT_AUTHORIZED" | "REFRESH_COMPLETE"
        ) | (Some(_), "SUPERSESSION_RECORDED", "SUPERSEDED")
            | (Some(_), "WITHDRAWAL_RECORDED", "WITHDRAWN")
    )
}

fn verify_assurance_replacements(entries: &BTreeMap<&str, &Value>) -> Result<()> {
    for entry in entries
        .values()
        .filter(|entry| entry["state"] == "SUPERSEDED")
    {
        verify_assurance_replacement(entries, entry)?;
    }
    for id in entries.keys() {
        verify_assurance_replacement_chain(entries, id)?;
    }
    Ok(())
}

fn verify_assurance_replacement(entries: &BTreeMap<&str, &Value>, entry: &Value) -> Result<()> {
    let (replacement, replacement_entry) = assurance_replacement_entry(entries, entry)?;
    require_assurance_replacement(
        !matches!(
            replacement_entry["state"].as_str(),
            Some("SUPERSEDED" | "WITHDRAWN")
        ),
        &replacement,
    )?;
    require_assurance_replacement(
        replacement != string(entry, "/impact_entry_id")?,
        &replacement,
    )?;
    verify_assurance_replacement_bindings(replacement_entry, entry, &replacement)
}

fn verify_assurance_replacement_bindings(
    replacement_entry: &Value,
    entry: &Value,
    replacement: &str,
) -> Result<()> {
    require_assurance_replacement(
        replacement_entry["report_root"] == entry["report_root"],
        replacement,
    )?;
    require_assurance_replacement(
        replacement_entry["target_head"] == entry["target_head"],
        replacement,
    )?;
    require_assurance_replacement(
        replacement_entry["matching_watch_ids"] == entry["matching_watch_ids"],
        replacement,
    )?;
    require_assurance_replacement(
        replacement_entry["changed_object"] == entry["changed_object"],
        replacement,
    )
}

fn assurance_replacement_entry<'a>(
    entries: &BTreeMap<&str, &'a Value>,
    entry: &Value,
) -> Result<(String, &'a Value)> {
    let replacement = string(entry, "/replacement_entry_id")?.to_owned();
    let replacement_entry = entries
        .get(replacement.as_str())
        .ok_or_else(|| ledger_error("GATE-ASSURANCE-REPLACEMENT", &replacement))?;
    Ok((replacement, replacement_entry))
}

fn require_assurance_replacement(compatible: bool, replacement: &str) -> Result<()> {
    if compatible {
        Ok(())
    } else {
        Err(ledger_error("GATE-ASSURANCE-REPLACEMENT", replacement))
    }
}

fn verify_assurance_replacement_chain(entries: &BTreeMap<&str, &Value>, id: &&str) -> Result<()> {
    let mut seen = BTreeSet::new();
    let mut current = *id;
    while let Some(next) = entries
        .get(current)
        .and_then(|entry| entry["replacement_entry_id"].as_str())
    {
        if !seen.insert(current) {
            return Err(ledger_error("GATE-ASSURANCE-REPLACEMENT-CYCLE", *id));
        }
        current = next;
    }
    Ok(())
}

fn verify_assurance_events(record: &Value, entries: &[Value]) -> Result<()> {
    let mut predecessor = entries
        .iter()
        .last()
        .map(|entry| string(entry, "/event_id"))
        .transpose()?;
    for event in array(record, "/events")? {
        verify_id(event, "event_id", "GATE-ASSURANCE-TRANSFER-EVENT-ID")?;
        let event_id = string(event, "/event_id")?;
        if event["predecessor_event_id"].as_str() != predecessor {
            return Err(ledger_error("GATE-ASSURANCE-EVENT-ANCESTRY", event_id));
        }
        predecessor = Some(event_id);
    }
    verify_assurance_currency_events(record)
}

fn verify_assurance_currency_events(record: &Value) -> Result<()> {
    let events = array(record, "/events")?;
    let has_campaign_transfer = events.iter().any(|event| {
        event["event_type"] == "CAMPAIGN_TRANSFER"
            && event["target_head"] == record["campaign_head"]
    });
    let has_release_transfer = events.iter().any(|event| {
        event["event_type"] == "RELEASE_TRANSFER"
            && event["target_head"] == record["campaign_head"]
            && event["release_identity"] == record["release_identity"]
    });
    if record["axes"]["campaign_transfer_currency"] == "CURRENT" && !has_campaign_transfer {
        return Err(ledger_error(
            "GATE-ASSURANCE-CAMPAIGN-TRANSFER",
            "current campaign transfer lacks an authenticated event",
        ));
    }
    if record["axes"]["release_transfer_currency"] == "CURRENT" && !has_release_transfer {
        return Err(ledger_error(
            "GATE-ASSURANCE-RELEASE-TRANSFER",
            "current release transfer lacks an authenticated event",
        ));
    }
    if record["axes"]["campaign_transfer_currency"] == "CURRENT"
        || record["axes"]["release_transfer_currency"] == "CURRENT"
    {
        return Err(ledger_error(
            "GATE-ASSURANCE-TRANSFER-UNAUTHENTICATED",
            "CURRENT requires verified receipt, envelope, role, and revocation capabilities",
        ));
    }
    verify_assurance_invalidation_events(record, events)
}

fn verify_assurance_invalidation_events(record: &Value, events: &[Value]) -> Result<()> {
    let invalidated = assurance_invalidation_active(record, events);
    if invalidated
        && (record["aggregate_impact"] != "IMPACT_PENDING"
            || record["axes"]["campaign_transfer_currency"] != "BLOCKED"
            || record["axes"]["release_transfer_currency"] != "BLOCKED")
    {
        Err(ledger_error(
            "GATE-ASSURANCE-INVALIDATION",
            "head or authority invalidation must reopen impact and block currency",
        ))
    } else {
        Ok(())
    }
}

fn assurance_invalidation_active(record: &Value, events: &[Value]) -> bool {
    let authority = &record["resolution_authority"];
    events.iter().any(|event| {
        event["event_type"] == "TARGET_HEAD_CHANGED"
            || event["event_type"] == "PRINCIPAL_ROLE_REVOKED"
                && event["principal_id"] == authority["principal_id"]
                && event["role_record_sha256"] == authority["role_record_sha256"]
    })
}

fn aggregate_impact<'a>(states: impl Iterator<Item = Result<&'a str>>) -> Result<&'static str> {
    let states = states.collect::<Result<Vec<_>>>()?;
    Ok(
        if states
            .iter()
            .any(|state| matches!(*state, "OPEN_UNKNOWN" | "OPEN_ASSESSMENT"))
        {
            "IMPACT_PENDING"
        } else if states.contains(&"REFRESH_REQUIRED") {
            "REFRESH_REQUIRED"
        } else if states.contains(&"REFRESH_COMPLETE") {
            "REFRESH_COMPLETE"
        } else if states.contains(&"NO_MATERIAL_IMPACT_AUTHORIZED") {
            "NO_MATERIAL_IMPACT_AUTHORIZED"
        } else {
            "NO_IMPACT_DETECTED"
        },
    )
}

fn verify_assurance_axes(record: &Value, aggregate: &str) -> Result<()> {
    if record["aggregate_impact"] != aggregate
        || record["axes"]["campaign_impact_disposition"] != aggregate
    {
        return Err(ledger_error(
            "GATE-ASSURANCE-FOLD",
            "aggregate impact mismatch",
        ));
    }
    if aggregate == "IMPACT_PENDING" && record["axes"]["campaign_transfer_currency"] != "BLOCKED" {
        return Err(ledger_error(
            "GATE-ASSURANCE-CURRENCY",
            "pending impact cannot be current",
        ));
    }
    if record["axes"]["release_transfer_request"] == "NOT_REQUESTED"
        && record["axes"]["release_transfer_currency"] != "BLOCKED"
    {
        return Err(ledger_error(
            "GATE-ASSURANCE-RELEASE-CURRENCY",
            "unrequested release transfer cannot be current",
        ));
    }
    Ok(())
}

fn allowed_transition(from: Option<&str>, to: &str) -> bool {
    matches!(
        (from, to),
        (
            None | Some("DEFERRED" | "FAIL" | "BLOCKED" | "STALE" | "LEGACY_UNVERIFIED"),
            "PENDING"
        ) | (Some("PENDING" | "FAIL" | "BLOCKED" | "STALE"), "PASS")
            | (Some("PASS"), "STALE")
            | (Some("PENDING"), "FAIL" | "BLOCKED")
            | (
                Some(
                    "PENDING"
                        | "PASS"
                        | "FAIL"
                        | "BLOCKED"
                        | "STALE"
                        | "DEFERRED"
                        | "NOT_APPLICABLE"
                        | "LEGACY_UNVERIFIED"
                ),
                "SUPERSEDED"
            )
    )
}

fn verify_id(value: &Value, field: &str, code: &'static str) -> Result<()> {
    if value[field] == derived_id(value, field)? {
        Ok(())
    } else {
        Err(ledger_error(code, "derived identity mismatch"))
    }
}

fn validate_document(repo: &Path, stem: &str, value: &Value) -> Result<()> {
    let path = repo.join(format!("gate-policy/v1/schemas/{stem}.schema.json"));
    let bytes = fs::read(&path).map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-LEDGER-SCHEMA", error.to_string())
    })?;
    validate_schema(&parse_strict(&bytes)?, value, stem)
}

fn array<'a>(value: &'a Value, pointer: &str) -> Result<&'a [Value]> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| ledger_error("GATE-LEDGER-SHAPE", pointer))
}

fn string<'a>(value: &'a Value, pointer: &str) -> Result<&'a str> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| ledger_error("GATE-LEDGER-SHAPE", pointer))
}

fn ledger_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Ledger, code, message)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::{Value, json};

    use super::{
        aggregate_impact, allowed_transition, assurance_subject_id, verify_assurance_impact,
        verify_assurance_replacements, verify_authorizations, verify_campaign_ledger,
        verify_certification_references, verify_predecessor,
    };
    use crate::canonical::derived_id;

    #[test]
    fn transition_table_blocks_legacy_promotion_and_deferral_after_failure() {
        assert!(!allowed_transition(Some("LEGACY_UNVERIFIED"), "PASS"));
        assert!(!allowed_transition(Some("FAIL"), "DEFERRED"));
        assert!(allowed_transition(Some("STALE"), "PENDING"));
    }

    fn repo() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn retained_campaign_ledger_reconstructs_ancestry_and_identity() {
        let root = repo();
        let mut ledger: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/campaign-ledger.json"))
                .expect("ledger fixture"),
        )
        .expect("ledger JSON");
        ledger["events"][0]["event_id"] =
            Value::String(derived_id(&ledger["events"][0], "event_id").expect("event identity"));
        ledger["ledger_id"] =
            Value::String(derived_id(&ledger, "ledger_id").expect("ledger identity"));
        verify_campaign_ledger(&root, &ledger, None).expect("ledger verification");

        let mut drifted = ledger.clone();
        drifted["events"][0]["predecessor_event_id"] = json!("f".repeat(64));
        assert!(verify_campaign_ledger(&root, &drifted, None).is_err());

        let receipt_id = "6".repeat(64);
        let envelope_id = "7".repeat(64);
        let mut passed = ledger;
        passed["obligations"][0]["state"] = json!("PASS");
        passed["obligations"][0]["receipt_id"] = json!(receipt_id);
        passed["receipts"] = json!([{
            "receipt_id": receipt_id,
            "envelope_id": envelope_id,
            "obligation_ids": ["full-regression"]
        }]);
        let predecessor = passed["events"][0]["event_id"].clone();
        let mut transition = json!({
            "event_id": "0".repeat(64),
            "predecessor_event_id": predecessor,
            "event_type": "OBLIGATION_TRANSITION",
            "target_head": passed["current_head"],
            "authorized_by": "openwepp-maintainers",
            "recorded_at": "2026-07-17T12:01:00Z",
            "payload": {
                "kind": "OBLIGATION_TRANSITION", "obligation_id": "full-regression",
                "from_state": "PENDING", "to_state": "PASS", "reason_code": "gate-passed",
                "receipt_id": receipt_id, "replacement_obligation_id": null
            }
        });
        transition["event_id"] =
            json!(derived_id(&transition, "event_id").expect("transition identity"));
        let mut execution = json!({
            "event_id": "0".repeat(64),
            "predecessor_event_id": transition["event_id"],
            "event_type": "EXECUTION_RECORDED",
            "target_head": passed["current_head"],
            "authorized_by": "openwepp-maintainers",
            "recorded_at": "2026-07-17T12:02:00Z",
            "payload": {
                "kind": "EXECUTION_RECORDED", "obligation_id": "full-regression",
                "receipt_id": receipt_id, "envelope_id": envelope_id, "result": "PASS"
            }
        });
        execution["event_id"] =
            json!(derived_id(&execution, "event_id").expect("execution identity"));
        passed["events"]
            .as_array_mut()
            .expect("events")
            .extend([transition, execution]);
        passed["ledger_id"] = json!(derived_id(&passed, "ledger_id").expect("ledger identity"));
        assert!(
            verify_campaign_ledger(&root, &passed, None).is_err(),
            "self-declared receipt and envelope references cannot establish PASS"
        );

        let mut unproven_pass = passed;
        unproven_pass["events"]
            .as_array_mut()
            .expect("events")
            .truncate(1);
        unproven_pass["ledger_id"] =
            json!(derived_id(&unproven_pass, "ledger_id").expect("ledger identity"));
        assert!(verify_campaign_ledger(&root, &unproven_pass, None).is_err());
    }

    #[test]
    fn ledger_security_bindings_preserve_typed_fail_closed_outcomes() {
        let mut previous = json!({
            "ledger_id": "0".repeat(64),
            "campaign": {"campaign_id": "campaign"},
            "current_head": "head"
        });
        previous["ledger_id"] =
            json!(derived_id(&previous, "ledger_id").expect("predecessor identity"));
        let mut ledger = json!({
            "campaign": {"campaign_id": "campaign"},
            "predecessor_ledger_id": previous["ledger_id"],
            "expected_predecessor_head": "head"
        });
        verify_predecessor(&ledger, Some(&previous)).expect("exact predecessor binding");
        ledger["expected_predecessor_head"] = json!("different");
        let error = verify_predecessor(&ledger, Some(&previous))
            .expect_err("predecessor CAS drift must fail closed");
        assert_eq!(error.code, "GATE-LEDGER-CAS");

        let mut authorization = json!({
            "authorization_id": "0".repeat(64),
            "campaign_id": "campaign",
            "target_head": "head",
            "predecessor_ledger_id": null,
            "envelope_id": "envelope"
        });
        authorization["authorization_id"] =
            json!(derived_id(&authorization, "authorization_id").expect("authorization identity"));
        let mut authorized_ledger = json!({
            "campaign": {"campaign_id": "campaign"},
            "current_head": "head",
            "predecessor_ledger_id": null,
            "authorization_events": [authorization]
        });
        let error = verify_authorizations(&authorized_ledger)
            .expect_err("self-declared authorization remains unauthenticated");
        assert_eq!(error.code, "GATE-LEDGER-AUTHORIZATION-UNAUTHENTICATED");
        authorized_ledger["authorization_events"][0]["target_head"] = json!("different");
        authorized_ledger["authorization_events"][0]["authorization_id"] = json!(
            derived_id(
                &authorized_ledger["authorization_events"][0],
                "authorization_id"
            )
            .expect("drifted authorization identity")
        );
        let error = verify_authorizations(&authorized_ledger)
            .expect_err("authorization binding drift must fail closed");
        assert_eq!(error.code, "GATE-LEDGER-AUTHORIZATION");

        authorized_ledger["authorization_events"][0]["target_head"] = json!("head");
        authorized_ledger["authorization_events"][0]["authorization_id"] = json!(
            derived_id(
                &authorized_ledger["authorization_events"][0],
                "authorization_id"
            )
            .expect("restored authorization identity")
        );
        authorized_ledger["certification"] = json!({
            "receipt_id": "receipt",
            "envelope_id": "envelope",
            "authorization_id": authorized_ledger["authorization_events"][0]["authorization_id"],
            "certified_head": "head"
        });
        let receipt_reference = json!({"envelope_id": "envelope"});
        let receipt_refs = BTreeMap::from([("receipt", &receipt_reference)]);
        let error = verify_certification_references(&authorized_ledger, &receipt_refs)
            .expect_err("self-declared certification remains unauthenticated");
        assert_eq!(error.code, "GATE-LEDGER-CERTIFICATION-UNAUTHENTICATED");
        authorized_ledger["certification"]["envelope_id"] = json!("different");
        let error = verify_certification_references(&authorized_ledger, &receipt_refs)
            .expect_err("certification reference drift must fail closed");
        assert_eq!(error.code, "GATE-LEDGER-CERTIFICATION-REFERENCE");

        let superseded = json!({
            "impact_entry_id": "old", "state": "SUPERSEDED",
            "replacement_entry_id": "new", "report_root": "root",
            "target_head": "head", "matching_watch_ids": [], "changed_object": {}
        });
        let mut replacement = json!({
            "impact_entry_id": "new", "state": "OPEN_ASSESSMENT",
            "replacement_entry_id": null, "report_root": "root",
            "target_head": "head", "matching_watch_ids": [], "changed_object": {}
        });
        let entries = BTreeMap::from([("old", &superseded), ("new", &replacement)]);
        verify_assurance_replacements(&entries).expect("compatible acyclic replacement");
        replacement["replacement_entry_id"] = json!("old");
        let entries = BTreeMap::from([("old", &superseded), ("new", &replacement)]);
        let error = verify_assurance_replacements(&entries)
            .expect_err("replacement cycles must fail closed");
        assert_eq!(error.code, "GATE-ASSURANCE-REPLACEMENT-CYCLE");
    }

    #[test]
    fn assurance_fold_is_exact_target_and_deterministic() {
        let root = repo();
        let mut record: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/assurance-impact.json"))
                .expect("assurance fixture"),
        )
        .expect("assurance JSON");
        record["entries"][0]["impact_subject_id"] =
            Value::String(assurance_subject_id(&record["entries"][0]).expect("subject identity"));
        record["entries"][0]["impact_entry_id"] = Value::String(
            derived_id(&record["entries"][0], "impact_entry_id").expect("entry identity"),
        );
        record["record_id"] =
            Value::String(derived_id(&record, "record_id").expect("record identity"));
        verify_assurance_impact(&root, &record).expect("assurance verification");

        record["axes"]["campaign_transfer_currency"] = json!("CURRENT");
        assert!(verify_assurance_impact(&root, &record).is_err());
    }

    #[test]
    fn assurance_history_folds_by_immutable_subject_not_path() {
        let root = repo();
        let mut record: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/assurance-impact.json"))
                .expect("assurance fixture"),
        )
        .expect("assurance JSON");
        record["entries"][0]["impact_subject_id"] =
            json!(assurance_subject_id(&record["entries"][0]).expect("first subject"));
        record["entries"][0]["impact_entry_id"] = json!(
            derived_id(&record["entries"][0], "impact_entry_id").expect("first entry identity")
        );

        let mut later_change = record["entries"][0].clone();
        later_change["terminal_plan_id"] = json!("9".repeat(64));
        later_change["changed_object"]["object_sha256"] = json!("8".repeat(64));
        later_change["event_id"] = json!("7".repeat(64));
        later_change["predecessor_event_id"] = record["entries"][0]["event_id"].clone();
        later_change["impact_subject_id"] =
            json!(assurance_subject_id(&later_change).expect("later subject"));
        later_change["impact_entry_id"] =
            json!(derived_id(&later_change, "impact_entry_id").expect("later entry identity"));
        assert_ne!(
            record["entries"][0]["impact_subject_id"], later_change["impact_subject_id"],
            "a later change at the same path is a distinct immutable subject"
        );
        record["entries"]
            .as_array_mut()
            .expect("entries")
            .push(later_change);
        record["record_id"] = json!(derived_id(&record, "record_id").expect("record identity"));
        verify_assurance_impact(&root, &record).expect("both same-path impacts remain open");
    }

    #[test]
    fn assurance_historical_open_event_does_not_override_terminal_subject_state() {
        let root = repo();
        let mut record: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/assurance-impact.json"))
                .expect("assurance fixture"),
        )
        .expect("assurance JSON");
        record["entries"][0]["impact_subject_id"] =
            json!(assurance_subject_id(&record["entries"][0]).expect("subject identity"));
        record["entries"][0]["impact_entry_id"] = json!(
            derived_id(&record["entries"][0], "impact_entry_id").expect("discovery identity")
        );
        let mut assessment = record["entries"][0].clone();
        assessment["event_id"] = json!("7".repeat(64));
        assessment["predecessor_event_id"] = record["entries"][0]["event_id"].clone();
        assessment["event_type"] = json!("ASSESSMENT_RECORDED");
        assessment["previous_state"] = json!("OPEN_ASSESSMENT");
        assessment["state"] = json!("REFRESH_REQUIRED");
        assessment["impact_entry_id"] =
            json!(derived_id(&assessment, "impact_entry_id").expect("assessment identity"));
        record["entries"]
            .as_array_mut()
            .expect("entries")
            .push(assessment);
        record["aggregate_impact"] = json!("REFRESH_REQUIRED");
        record["axes"]["campaign_impact_disposition"] = json!("REFRESH_REQUIRED");
        record["record_id"] = json!(derived_id(&record, "record_id").expect("record identity"));
        verify_assurance_impact(&root, &record).expect("terminal subject state controls fold");
    }

    #[test]
    fn unauthenticated_supersession_and_withdrawal_cannot_erase_open_impact() {
        let root = repo();
        for (event_type, state) in [
            ("SUPERSESSION_RECORDED", "SUPERSEDED"),
            ("WITHDRAWAL_RECORDED", "WITHDRAWN"),
        ] {
            let mut record: Value = serde_json::from_slice(
                &std::fs::read(root.join("gate-policy/v1/fixtures/valid/assurance-impact.json"))
                    .expect("assurance fixture"),
            )
            .expect("assurance JSON");
            record["entries"][0]["impact_subject_id"] =
                json!(assurance_subject_id(&record["entries"][0]).expect("subject identity"));
            record["entries"][0]["impact_entry_id"] = json!(
                derived_id(&record["entries"][0], "impact_entry_id").expect("discovery identity")
            );
            let mut disposition = record["entries"][0].clone();
            disposition["event_id"] = json!("7".repeat(64));
            disposition["predecessor_event_id"] = record["entries"][0]["event_id"].clone();
            disposition["event_type"] = json!(event_type);
            disposition["previous_state"] = json!("OPEN_ASSESSMENT");
            disposition["state"] = json!(state);
            if state == "SUPERSEDED" {
                disposition["replacement_entry_id"] =
                    record["entries"][0]["impact_entry_id"].clone();
            }
            disposition["impact_entry_id"] =
                json!(derived_id(&disposition, "impact_entry_id").expect("disposition identity"));
            record["entries"]
                .as_array_mut()
                .expect("entries")
                .push(disposition);
            record["aggregate_impact"] = json!("NO_IMPACT_DETECTED");
            record["axes"]["campaign_impact_disposition"] = json!("NO_IMPACT_DETECTED");
            record["record_id"] = json!(derived_id(&record, "record_id").expect("record identity"));
            let error = verify_assurance_impact(&root, &record)
                .expect_err("self-declared disposition cannot erase open impact");
            assert_eq!(error.code, "GATE-ASSURANCE-RESOLUTION-UNAUTHENTICATED");
        }
    }

    #[test]
    fn assurance_multi_object_fold_is_order_independent_and_conservative() {
        for states in [
            vec![Ok("NO_MATERIAL_IMPACT_AUTHORIZED"), Ok("REFRESH_COMPLETE")],
            vec![Ok("REFRESH_COMPLETE"), Ok("NO_MATERIAL_IMPACT_AUTHORIZED")],
        ] {
            assert_eq!(
                aggregate_impact(states.into_iter()).expect("fold"),
                "REFRESH_COMPLETE"
            );
        }
        for states in [
            vec![Ok("REFRESH_REQUIRED"), Ok("OPEN_ASSESSMENT")],
            vec![Ok("OPEN_ASSESSMENT"), Ok("REFRESH_REQUIRED")],
        ] {
            assert_eq!(
                aggregate_impact(states.into_iter()).expect("fold"),
                "IMPACT_PENDING"
            );
        }
    }

    #[test]
    fn target_head_change_reopens_empty_exact_target_fold() {
        let root = repo();
        let mut record: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/assurance-impact.json"))
                .expect("assurance fixture"),
        )
        .expect("assurance JSON");
        record["entries"] = json!([]);
        let mut event = json!({
            "event_id": "0".repeat(64),
            "predecessor_event_id": null,
            "event_type": "TARGET_HEAD_CHANGED",
            "previous_head": "9".repeat(64),
            "target_head": record["campaign_head"],
            "recorded_at": "2026-07-18T12:00:00Z"
        });
        event["event_id"] = json!(derived_id(&event, "event_id").expect("head-change identity"));
        record["events"] = json!([event]);
        record["aggregate_impact"] = json!("IMPACT_PENDING");
        record["axes"]["campaign_impact_disposition"] = json!("IMPACT_PENDING");
        record["record_id"] = json!(derived_id(&record, "record_id").expect("record identity"));
        verify_assurance_impact(&root, &record).expect("head change reopens exact target");
    }
}

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::canonical::{derived_id, parse_strict, validate_schema};
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
        Some(previous) => {
            verify_id(previous, "ledger_id", "GATE-LEDGER-PREDECESSOR-ID")?;
            if previous["campaign"]["campaign_id"] != ledger["campaign"]["campaign_id"] {
                return Err(ledger_error(
                    "GATE-LEDGER-PREDECESSOR-CAMPAIGN",
                    "predecessor campaign differs",
                ));
            }
            if ledger["predecessor_ledger_id"] != previous["ledger_id"]
                || ledger["expected_predecessor_head"] != previous["current_head"]
            {
                return Err(ledger_error(
                    "GATE-LEDGER-CAS",
                    "predecessor ledger/head mismatch",
                ));
            }
        }
        None if !ledger["predecessor_ledger_id"].is_null() => {
            return Err(ledger_error(
                "GATE-LEDGER-PREDECESSOR",
                "predecessor is required",
            ));
        }
        None => {}
    }
    Ok(())
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
    if let Some(authorization) = array(ledger, "/authorization_events")?.first() {
        verify_id(
            authorization,
            "authorization_id",
            "GATE-LEDGER-AUTHORIZATION-ID",
        )?;
        let id = string(authorization, "/authorization_id")?;
        if !ids.insert(id)
            || authorization["campaign_id"] != ledger["campaign"]["campaign_id"]
            || authorization["target_head"] != ledger["current_head"]
            || authorization["predecessor_ledger_id"] != ledger["predecessor_ledger_id"]
        {
            return Err(ledger_error("GATE-LEDGER-AUTHORIZATION", id));
        }
        Err(ledger_error(
            "GATE-LEDGER-AUTHORIZATION-UNAUTHENTICATED",
            "self-declared authorization cannot establish authority",
        ))
    } else {
        Ok(())
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
    if certification["envelope_id"] == reference["envelope_id"]
        && authorization.is_some_and(|authorization| {
            authorization["envelope_id"] == certification["envelope_id"]
                && authorization["target_head"] == certification["certified_head"]
        })
    {
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
    let entries = assurance_entries(raw_entries, target)?;
    verify_assurance_replacements(&entries)?;
    verify_assurance_events(record, raw_entries)?;
    let aggregate = aggregate_impact(entries.values().map(|entry| string(entry, "/state")))?;
    verify_assurance_axes(record, aggregate)
}

fn assurance_entries<'a>(
    raw_entries: &'a [Value],
    target: &Value,
) -> Result<BTreeMap<&'a str, &'a Value>> {
    let mut entries = BTreeMap::new();
    let mut predecessor_event: Option<&str> = None;
    let mut object_states = BTreeMap::<&str, &str>::new();
    for entry in raw_entries {
        verify_id(entry, "impact_entry_id", "GATE-ASSURANCE-ENTRY-ID")?;
        let id = string(entry, "/impact_entry_id")?;
        if entries.insert(id, entry).is_some() {
            return Err(ledger_error("GATE-ASSURANCE-ENTRY-DUPLICATE", id));
        }
        if entry["target_head"] != *target {
            return Err(ledger_error("GATE-ASSURANCE-TARGET", id));
        }
        let object_path = string(entry, "/changed_object/path")?;
        let previous_state = object_states.get(object_path).copied();
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
        object_states.insert(object_path, next_state);
        if matches!(
            next_state,
            "NO_MATERIAL_IMPACT_AUTHORIZED" | "REFRESH_COMPLETE"
        ) {
            return Err(ledger_error(
                "GATE-ASSURANCE-RESOLUTION-UNAUTHENTICATED",
                "terminal assurance resolution requires verified receipt and role capabilities",
            ));
        }
    }
    Ok(entries)
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
        let replacement = string(entry, "/replacement_entry_id")?;
        let replacement_entry = entries
            .get(replacement)
            .ok_or_else(|| ledger_error("GATE-ASSURANCE-REPLACEMENT", replacement))?;
        if matches!(
            replacement_entry["state"].as_str(),
            Some("SUPERSEDED" | "WITHDRAWN")
        ) || replacement == string(entry, "/impact_entry_id")?
            || replacement_entry["report_root"] != entry["report_root"]
            || replacement_entry["target_head"] != entry["target_head"]
            || replacement_entry["matching_watch_ids"] != entry["matching_watch_ids"]
            || replacement_entry["changed_object"] != entry["changed_object"]
        {
            return Err(ledger_error("GATE-ASSURANCE-REPLACEMENT", replacement));
        }
    }
    for id in entries.keys() {
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
    let authority = &record["resolution_authority"];
    let invalidated = events.iter().any(|event| {
        event["event_type"] == "TARGET_HEAD_CHANGED"
            || event["event_type"] == "PRINCIPAL_ROLE_REVOKED"
                && event["principal_id"] == authority["principal_id"]
                && event["role_record_sha256"] == authority["role_record_sha256"]
    });
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
    use serde_json::{Value, json};

    use super::{allowed_transition, verify_assurance_impact, verify_campaign_ledger};
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
    fn assurance_fold_is_exact_target_and_deterministic() {
        let root = repo();
        let mut record: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/assurance-impact.json"))
                .expect("assurance fixture"),
        )
        .expect("assurance JSON");
        record["entries"][0]["impact_entry_id"] = Value::String(
            derived_id(&record["entries"][0], "impact_entry_id").expect("entry identity"),
        );
        record["record_id"] =
            Value::String(derived_id(&record, "record_id").expect("record identity"));
        verify_assurance_impact(&root, &record).expect("assurance verification");

        record["axes"]["campaign_transfer_currency"] = json!("CURRENT");
        assert!(verify_assurance_impact(&root, &record).is_err());
    }
}

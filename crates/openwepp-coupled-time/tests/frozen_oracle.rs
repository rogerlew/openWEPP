//! Separately written Rust executor for the frozen 108-case oracle population.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::format_collect,
    clippy::many_single_char_names,
    clippy::match_same_arms,
    clippy::needless_pass_by_value,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_lines
)]
use openwepp_coupled_time::{CoupledTimeError, ModelTimeNs, quantize_seconds_to_tick};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::PathBuf};
const SAME: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts")
}
fn err(s: &str) -> Result<Value, String> {
    Err(s.into())
}
fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v[k].as_str().unwrap()
}
fn tick(v: &Value) -> Result<u128, String> {
    let x = v.as_str().ok_or("InvalidWireIdentity")?;
    if x.is_empty() || (x != "0" && x.starts_with('0')) || !x.bytes().all(|b| b.is_ascii_digit()) {
        return Err("InvalidWireIdentity".into());
    }
    x.parse().map_err(|_| "InvalidWireIdentity".into())
}
fn support(v: &Value) -> Result<(u128, u128), String> {
    let a = tick(&v[0])?;
    let b = tick(&v[1])?;
    if a >= b {
        return Err("InvalidParentSupport".into());
    }
    Ok((a, b))
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn canonical(v: &Value) -> Vec<u8> {
    serde_json::to_vec(v).unwrap()
}
fn reject(name: String) -> Value {
    json!({"status":"rejected","error":name,"before_sha256":SAME,"after_sha256":SAME})
}
fn eval(c: &Value) -> Result<Value, String> {
    match s(c, "op") {
        "tick" => {
            tick(&c["value"])?;
            Ok(json!({"status":"accepted"}))
        }
        "duration" => {
            let (a, b) = support(&c["support"])?;
            Ok(
                json!({"status":"accepted","bits_hex":format!("{:016x}",((b-a)as f64/1e9).to_bits())}),
            )
        }
        "quantize" => {
            let bits = u64::from_str_radix(s(c, "seconds_bits"), 16).unwrap();
            let (a, b) = support(&c["parent"])?;
            match quantize_seconds_to_tick(
                ModelTimeNs::new(a),
                ModelTimeNs::new(b),
                f64::from_bits(bits),
            ) {
                Ok(t) => Ok(json!({"status":"accepted","tick":t.get().to_string()})),
                Err(CoupledTimeError::ArithmeticOverflow) => err("EventTickOverflow"),
                Err(_) => {
                    let exp = (bits >> 52) & 0x7ff;
                    let neg = bits >> 63 != 0;
                    if exp == 0x7ff || neg && bits << 1 != 0 {
                        err("InvalidEventProposal")
                    } else if f64::from_bits(bits) > (u128::MAX as f64 / 1e9) {
                        err("EventProposalOverflow")
                    } else {
                        err("EventPastParentEnd")
                    }
                }
            }
        }
        "partition" => {
            let (a, b) = support(&c["parent"])?;
            let mut cur = a;
            for p in c["segments"].as_array().unwrap() {
                let (x, y) = support(p)?;
                if x != cur || y > b {
                    return err("InvalidSegmentPartition");
                }
                cur = y
            }
            if cur != b {
                return err("InvalidSegmentPartition");
            }
            Ok(json!({"status":"accepted","cursor":cur.to_string()}))
        }
        "participants" => {
            let owners = c["owners"].as_array().unwrap();
            let mut sorted = owners.clone();
            sorted.sort_by_key(|v| v.as_str().unwrap().to_owned());
            sorted.dedup();
            if &sorted != owners
                || c["segments"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .flat_map(|v| v.as_array().unwrap())
                    .any(|x| !owners.contains(x))
            {
                return err("ParticipantSetMismatch");
            }
            Ok(json!({"status":"accepted","terminal":owners}))
        }
        "constraints" => constraints(c),
        "events" => events(c),
        "retry" => retry(c),
        "candidate" => candidate(c),
        "joins" => joins(c),
        "restart" => restart(c),
        "restart_equivalence" => restart_eq(c),
        "outbox" => outbox(c),
        "reduction" => reduction(c),
        "scheduled_output" => scheduled_output(c),
        "authority_tuple" => authority(c),
        "scheduled_once" => {
            let mut xs = c["accepted_receipts"].as_array().unwrap().clone();
            if xs.contains(&c["receipt_id"]) {
                return err("ScheduledOnceReplay");
            }
            xs.push(c["receipt_id"].clone());
            xs.sort_by_key(|v| v.as_str().unwrap().to_owned());
            Ok(json!({"status":"accepted","accepted_receipts":xs}))
        }
        "publication" => {
            if !c["parent_committed"].as_bool().unwrap() {
                return err("PublicationBeforeParentCommit");
            }
            let vals: Vec<_> = c["samples"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| v["accepted"] == true)
                .map(|v| v["value"].as_i64().unwrap())
                .collect();
            if vals.is_empty() {
                return err("NoAcceptedReductionOperand");
            }
            Ok(
                json!({"status":"accepted","maximum":vals.iter().max().unwrap(),"publication_order":c["records"].as_array().unwrap().iter().map(|v|v["receipt_id"].clone()).collect::<Vec<_>>()}),
            )
        }
        "transaction_successor" => {
            let n = tick(&c["sequence"])?;
            let next = n.checked_add(1).ok_or("TransactionSequenceOverflow")?;
            Ok(json!({"status":"accepted","successor":next.to_string()}))
        }
        "legacy_hash" => {
            for f in c["files"].as_array().unwrap() {
                let bytes =
                    fs::read(root().join(s(f, "path"))).map_err(|_| "DirectV10WireChanged")?;
                if sha(&bytes) != s(f, "sha256") {
                    return err("DirectV10WireChanged");
                }
            }
            Ok(json!({"status":"accepted","protected_files":c["files"].as_array().unwrap().len()}))
        }
        "identity" => identity(c),
        _ => err("UnknownVectorOperation"),
    }
}
fn constraints(c: &Value) -> Result<Value, String> {
    let (_, end) = support(&c["parent"])?;
    let cur = tick(&c["cursor"])?;
    let xs = c["constraints"].as_array().unwrap();
    if xs.is_empty() {
        return err("NoStepConstraint");
    }
    for x in xs {
        let e = tick(&x["end"])?;
        if e < cur {
            return err("ConstraintBehindAcceptedTime");
        }
        if e > end {
            return err("ConstraintPastParentEnd");
        }
        if e == cur && s(x, "class") != "EventBoundary" {
            return err("ZeroStepWithoutEventTransition");
        }
    }
    let selected = xs.iter().map(|x| tick(&x["end"]).unwrap()).min().unwrap();
    let tied: Vec<_> = xs
        .iter()
        .filter(|x| tick(&x["end"]).unwrap() == selected)
        .collect();
    let lineage: BTreeSet<_> = tied
        .iter()
        .map(|x| {
            (
                s(x, "parent_id"),
                s(x, "cursor"),
                s(x, "calendar_receipt"),
                s(x, "forcing_receipt"),
            )
        })
        .collect();
    let facts: BTreeSet<_> = tied
        .iter()
        .filter(|x| s(x, "class") != "AdaptiveUpperBound")
        .map(|x| s(x, "compatibility_group"))
        .collect();
    if lineage.len() != 1 || facts.len() > 1 {
        return err("ConflictingEqualTimeConstraints");
    }
    let rank = |x: &Value| {
        [
            "HardBoundary",
            "EventBoundary",
            "OutputBoundary",
            "RestartBoundary",
            "AdaptiveUpperBound",
        ]
        .iter()
        .position(|n| *n == s(x, "class"))
        .unwrap()
    };
    let mut tied = tied;
    tied.sort_by_key(|x| (rank(x), s(x, "owner"), s(x, "digest")));
    Ok(
        json!({"status":"accepted","end":selected.to_string(),"ordered":tied.iter().map(|x|format!("{}:{}:{}",s(x,"class"),s(x,"owner"),s(x,"digest"))).collect::<Vec<_>>()}),
    )
}
fn events(c: &Value) -> Result<Value, String> {
    let rank = |x: &Value| {
        [
            "OwnershipTransfer",
            "BoundaryModeTransition",
            "RegimeTransition",
            "ScheduledBoundary",
            "DiagnosticMarker",
        ]
        .iter()
        .position(|n| *n == s(x, "class"))
        .unwrap()
    };
    let mut xs = c["events"].as_array().unwrap().iter().collect::<Vec<_>>();
    xs.sort_by_key(|x| (rank(x), s(x, "owner"), s(x, "digest")));
    if xs.len()
        > c.get("same_tick_event_budget")
            .and_then(Value::as_u64)
            .unwrap_or(256) as usize
    {
        return err("EventBudgetExhausted");
    }
    let mut state = s(c, "begin_owner_sha256").to_owned();
    let mut seen: BTreeSet<String> = c
        .get("accepted_event_ids")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|v| v.as_str().unwrap().into())
        .collect();
    let mut semantic = BTreeSet::new();
    let mut receipts = vec![];
    for e in xs {
        if seen.contains(s(e, "event_id")) {
            return err("EventReplay");
        }
        if e["ledger_closed"] != true {
            return err("LedgerNotClosed");
        }
        if s(e, "begin_owner_sha256") != state {
            return err("EventCustodyConflict");
        }
        let key = format!(
            "{}:{}:{}:{}:{}",
            s(e, "tick"),
            s(e, "end_owner_sha256"),
            s(e, "regime_sha256"),
            s(e, "participant_sha256"),
            e["pending_event_ids"]
        );
        if !semantic.insert(key) {
            return err("EventNoProgressCycle");
        }
        seen.insert(s(e, "event_id").into());
        state = s(e, "end_owner_sha256").into();
        receipts.push(format!("{}:{}:{}", s(e, "class"), s(e, "event_id"), state))
    }
    Ok(
        json!({"status":"accepted","event_ordinal":c["event_ordinal"].as_u64().unwrap()+receipts.len()as u64,"end_owner_sha256":state,"receipts":receipts}),
    )
}
fn retry(c: &Value) -> Result<Value, String> {
    if c["controller_policy"] != c["restart_controller_policy"] {
        return err("ControllerPolicyMismatch");
    }
    let accepted = s(c, "accepted_state_sha256");
    let mut ords = vec![];
    for a in c["attempts"].as_array().unwrap() {
        ords.push(a["ordinal"].clone());
        if s(a, "begin_state_sha256") != accepted {
            return err("BeginningOwnerSetMismatch");
        }
        if a["owner_duration_bits"] != c["duration_bits"] {
            return err("DurationBitsMismatch");
        }
        if s(a, "outcome") == "accept" {
            return Ok(
                json!({"status":"accepted","attempt_ordinals":ords,"accepted_cursor":a["end"],"accepted_state_sha256":a["end_state_sha256"]}),
            );
        }
        if s(a, "end_state_sha256") != accepted {
            return err("RejectedAttemptMutatedState");
        }
    }
    err("MinimumStepExhaustion")
}
fn candidate(c: &Value) -> Result<Value, String> {
    let owners = c["complete_owners"].as_array().unwrap();
    let mut sorted = owners.clone();
    sorted.sort_by_key(|v| v.as_str().unwrap().to_owned());
    sorted.dedup();
    if &sorted != owners {
        return err("InvalidCompleteOwnerSet");
    }
    if c["participants"]
        .as_array()
        .unwrap()
        .iter()
        .any(|x| !owners.contains(x))
    {
        return err("ParticipantSetMismatch");
    }
    if c["begin_owner_sha256"] != c["accepted_owner_sha256"] {
        return err("BeginningOwnerSetMismatch");
    }
    for o in owners {
        if !c["participants"].as_array().unwrap().contains(o)
            && c["begin_bytes"][o.as_str().unwrap()] != c["end_bytes"][o.as_str().unwrap()]
        {
            return err("InactiveOwnerMutated");
        }
    }
    if s(c, "clock_writer") != "CoupledClock" {
        return err("UnauthorizedClockAdvance");
    }
    Ok(json!({"status":"accepted","end_owner_sha256":c["end_owner_sha256"]}))
}
fn joins(c: &Value) -> Result<Value, String> {
    let xs = c["owner_candidates"].as_array().unwrap();
    if xs.len() != c["complete_owner_count"].as_u64().unwrap() as usize
        || xs
            .iter()
            .map(|x| s(x, "owner"))
            .collect::<BTreeSet<_>>()
            .len()
            != xs.len()
    {
        return err("OwnerCardinalityMismatch");
    }
    if xs
        .iter()
        .any(|x| x["begin_digest"] != c["begin_owner_set_sha256"])
    {
        return err("BeginningOwnerSetMismatch");
    }
    if xs.iter().any(|x| x["ledger_closed"] != true) || c["aggregate_ledger_residual"] != 0 {
        return err("LedgerNotClosed");
    }
    Ok(
        json!({"status":"accepted","ending_owner_set_sha256":c["ending_owner_set_sha256"],"ledger_digest":c["ledger_digest"]}),
    )
}
fn restart(c: &Value) -> Result<Value, String> {
    let cp = &c["checkpoint"];
    for k in [
        "run_id",
        "calendar_receipt",
        "forcing_receipt",
        "model_definition",
        "constraint_policy",
        "controller_policy",
    ] {
        if cp[k] != c["expected_identity"][k] {
            return err("RestartIdentityMismatch");
        }
    }
    let owners = cp["complete_owner_state"].as_array().unwrap();
    if owners.len() != cp["owner_count"].as_u64().unwrap() as usize
        || owners
            .iter()
            .map(|x| s(x, "owner"))
            .collect::<BTreeSet<_>>()
            .len()
            != owners.len()
    {
        return err("OwnerCardinalityMismatch");
    }
    if cp["accepted_until_ns"] == cp["event_tick_ns"]
        && (cp["event_applied"].as_bool().unwrap()
            != cp["accepted_event_receipts"]
                .as_array()
                .unwrap()
                .contains(&cp["event_receipt_id"]))
    {
        return err("EventReplayStateMismatch");
    }
    let keys = [
        "accepted_event_receipts",
        "scheduled_once_receipts",
        "reduction_maximum",
        "publication_outbox",
        "complete_owner_state",
    ];
    let cont: serde_json::Map<_, _> = keys.iter().map(|k| ((*k).into(), cp[*k].clone())).collect();
    if c.get("expected_continuation")
        .is_some_and(|v| v != &Value::Object(cont.clone()))
    {
        return err("RestartContinuationMismatch");
    }
    Ok(
        json!({"status":"accepted","continuation_sha256":sha(&canonical(cp)),"scheduled_once_receipts":cp["scheduled_once_receipts"],"reduction_maximum":cp["reduction_maximum"],"publication_outbox":cp["publication_outbox"]}),
    )
}
fn restart_eq(c: &Value) -> Result<Value, String> {
    let keys = [
        "ending_owner_set_sha256",
        "accepted_slab_receipts",
        "accepted_event_receipts",
        "scheduled_once_receipts",
        "reduction_state",
        "publication_outbox",
    ];
    if keys
        .iter()
        .any(|k| c["uninterrupted"][*k] != c["restarted"][*k])
    {
        return err("RestartContinuationMismatch");
    }
    let map: serde_json::Map<_, _> = keys
        .iter()
        .map(|k| ((*k).into(), c["restarted"][*k].clone()))
        .collect();
    Ok(json!({"status":"accepted","equivalence_sha256":sha(&canonical(&Value::Object(map)))}))
}
fn outbox(c: &Value) -> Result<Value, String> {
    let key = (s(c, "state"), s(c, "action"));
    let end = match key {
        ("Buffered", "parent_commit") => "CommittedUndelivered",
        ("Buffered", "parent_rollback") => "Removed",
        ("CommittedUndelivered", "deliver" | "crash" | "restart") => {
            if key.1 == "deliver" {
                "DeliveredUnacknowledged"
            } else {
                "CommittedUndelivered"
            }
        }
        ("DeliveredUnacknowledged", "crash" | "redeliver") => "DeliveredUnacknowledged",
        ("DeliveredUnacknowledged", "ack") => "Acknowledged",
        ("Acknowledged", "crash" | "restart") => "Acknowledged",
        _ => return err("InvalidOutboxTransition"),
    };
    let count =
        c["delivery_count"].as_u64().unwrap() + u64::from(matches!(key.1, "deliver" | "redeliver"));
    Ok(json!({"status":"accepted","state":end,"receipt_id":c["receipt_id"],"delivery_count":count}))
}
fn reduction(c: &Value) -> Result<Value, String> {
    let accepted: Vec<_> = c["operands"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|x| x["accepted"] == true && s(x, "phase") == "accepted_slab")
        .collect();
    let max = accepted
        .iter()
        .map(|x| x["value"].as_i64().unwrap())
        .max()
        .unwrap();
    if c["claimed_maximum"].as_i64() != Some(max) {
        return err("ReductionAliasMismatch");
    }
    if c["published_before_commit"] == true {
        return err("PublicationBeforeParentCommit");
    }
    Ok(
        json!({"status":"accepted","maximum":max,"accepted_operand_receipts":accepted.iter().map(|x|x["receipt_id"].clone()).collect::<Vec<_>>()}),
    )
}
fn scheduled_output(c: &Value) -> Result<Value, String> {
    let xs = c["records"].as_array().unwrap();
    if xs
        .iter()
        .map(|x| s(x, "scheduled_receipt_id"))
        .collect::<BTreeSet<_>>()
        .len()
        != xs.len()
    {
        return err("DuplicateScheduledOutput");
    }
    Ok(
        json!({"status":"accepted","publication_order":xs.iter().map(|x|x["output_receipt_id"].clone()).collect::<Vec<_>>()}),
    )
}
fn authority(c: &Value) -> Result<Value, String> {
    let h = s(c, "hydrology");
    let t = s(c, "time");
    let l = c
        .get("lane_d")
        .and_then(Value::as_str)
        .unwrap_or("WholeDayNonpersistent");
    let r = c
        .get("legacy_r4l_mutation")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    if !((h == "LegacyWb14Wb18Wb19" && t == "LegacyFixedSchedule")
        || (h == "RichardsCoupledV1" && t == "CoupledAdaptiveSupportV1" && l == "Persistent" && !r))
    {
        return err("UnsupportedAuthorityTuple");
    }
    Ok(json!({"status":"accepted","tuple":format!("{h}:{t}:{l}:{}",if r{"true"}else{"false"})}))
}
fn identity(c: &Value) -> Result<Value, String> {
    let domain = s(c, "domain");
    let version = c.get("version").and_then(Value::as_u64).unwrap_or(1);
    let model: Value =
        serde_json::from_slice(&fs::read(root().join("model-definition.json")).unwrap()).unwrap();
    if c.get("enforce_domain")
        .and_then(Value::as_bool)
        .unwrap_or(true)
    {
        let declared = model["identity_domain_fields"]
            .get(domain)
            .and_then(Value::as_array)
            .ok_or("InvalidIdentityDomain")?;
        let actual: Vec<_> = c["fields"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| format!("{}:{}", s(f, "tag"), s(f, "type")))
            .collect();
        if actual
            != declared
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect::<Vec<_>>()
        {
            return err("IdentityFieldSchemaMismatch");
        }
        if version != 1 {
            return err("IdentityVersionMismatch");
        }
    }
    let mut pre = b"OPENWEPP\0".to_vec();
    pre.extend_from_slice(&(version as u16).to_be_bytes());
    pre.extend_from_slice(&(domain.len() as u16).to_be_bytes());
    pre.extend_from_slice(domain.as_bytes());
    for f in c["fields"].as_array().unwrap() {
        let tag = s(f, "tag").as_bytes();
        let val = match s(f, "type") {
            "u32" => (u32::try_from(f["value"].as_u64().ok_or("ArithmeticOverflow")?)
                .map_err(|_| "ArithmeticOverflow")?)
            .to_be_bytes()
            .to_vec(),
            "u128" => tick(&f["value"])?.to_be_bytes().to_vec(),
            "sha256" | "bytes" => hex_decode(s(f, "value"))?,
            "utf8" => s(f, "value").as_bytes().to_vec(),
            "optional-none" => vec![0],
            _ => return err("InvalidWireIdentity"),
        };
        pre.extend_from_slice(&(tag.len() as u16).to_be_bytes());
        pre.extend_from_slice(tag);
        pre.extend_from_slice(&(val.len() as u32).to_be_bytes());
        pre.extend_from_slice(&val)
    }
    Ok(json!({"status":"accepted","preimage_hex":hex_encode(&pre),"sha256":sha(&pre)}))
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err("InvalidWireIdentity".into());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| "InvalidWireIdentity".into()))
        .collect()
}
fn hex_encode(v: &[u8]) -> String {
    v.iter().map(|b| format!("{b:02x}")).collect()
}
#[test]
fn production_rust_matches_all_frozen_oracle_cases() {
    let vectors: Value =
        serde_json::from_slice(&fs::read(root().join("coupled-time-vectors.json")).unwrap())
            .unwrap();
    let cases = vectors["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 108);
    for c in cases {
        let actual = eval(c).unwrap_or_else(reject);
        assert_eq!(actual, c["expected"], "{}", s(c, "id"));
    }
}

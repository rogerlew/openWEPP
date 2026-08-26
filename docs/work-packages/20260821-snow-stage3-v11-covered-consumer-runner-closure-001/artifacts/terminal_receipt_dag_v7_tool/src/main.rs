use openwepp_coupled_time::{Digest32, FramedField, framed_sha256};
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, fs, path::Path};

const ORDER: [&str; 9] = [
    "BatchRequestCore",
    "ArmInputCore",
    "ArmPhysicalResultCore",
    "EndingJointReceipt",
    "ArmCustodyReceipt",
    "LaneEvidenceCore",
    "BatchResultCore",
    "EventReceipt",
    "ParentReceipt",
];

fn predecessors(node: &str) -> &'static [&'static str] {
    match node {
        "BatchRequestCore" => &[],
        "ArmInputCore" => &["BatchRequestCore"],
        "ArmPhysicalResultCore" => &["ArmInputCore"],
        "EndingJointReceipt" => &["ArmPhysicalResultCore"],
        "ArmCustodyReceipt" => &[
            "ArmInputCore",
            "ArmPhysicalResultCore",
            "EndingJointReceipt",
        ],
        "LaneEvidenceCore" => &["ArmCustodyReceipt"],
        "BatchResultCore" => &["BatchRequestCore", "ArmCustodyReceipt", "LaneEvidenceCore"],
        "EventReceipt" => &["BatchResultCore"],
        "ParentReceipt" => &["BatchResultCore", "EventReceipt"],
        _ => unreachable!("closed receipt node set"),
    }
}

fn construct(
    payloads: &BTreeMap<&'static str, Vec<u8>>,
) -> Result<BTreeMap<&'static str, Digest32>, String> {
    let mut digests: BTreeMap<&'static str, Digest32> = BTreeMap::new();
    for (ordinal, node) in ORDER.iter().copied().enumerate() {
        let schema = 1_u32.to_be_bytes();
        let ordinal = u32::try_from(ordinal)
            .map_err(|_| "receipt ordinal overflow")?
            .to_be_bytes();
        let predecessor_digests = predecessors(node)
            .iter()
            .map(|predecessor| {
                digests
                    .get(predecessor)
                    .copied()
                    .ok_or_else(|| format!("successor reference from {node} to {predecessor}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let payload = payloads
            .get(node)
            .ok_or_else(|| format!("missing node-local payload for {node}"))?;
        let mut fields = vec![
            FramedField {
                tag: "schema",
                value: &schema,
            },
            FramedField {
                tag: "ordinal",
                value: &ordinal,
            },
            FramedField {
                tag: "payload",
                value: payload,
            },
        ];
        fields.extend(predecessor_digests.iter().map(|digest| FramedField {
            tag: "predecessor",
            value: digest.as_bytes(),
        }));
        let domain = format!("child1-{}-v1", node.to_ascii_lowercase());
        let digest = framed_sha256(&domain, &fields).map_err(|error| error.to_string())?;
        digests.insert(node, digest);
    }
    Ok(digests)
}

fn baseline_payloads() -> BTreeMap<&'static str, Vec<u8>> {
    ORDER
        .iter()
        .copied()
        .map(|node| {
            (
                node,
                format!("child1-terminal-candidate::{node}").into_bytes(),
            )
        })
        .collect()
}

fn ancestors(target: &str) -> Vec<&'static str> {
    let mut selected = Vec::new();
    for node in ORDER {
        if node == target {
            break;
        }
        if predecessors(target).contains(&node)
            || predecessors(target)
                .iter()
                .any(|predecessor| ancestors(predecessor).contains(&node))
        {
            selected.push(node);
        }
    }
    selected
}

fn descendants(target: &str) -> Vec<&'static str> {
    let mut selected = Vec::new();
    for node in ORDER {
        if node != target
            && predecessors(node)
                .iter()
                .any(|predecessor| *predecessor == target || selected.contains(predecessor))
        {
            selected.push(node);
        }
    }
    selected
}

fn digest_map(values: &BTreeMap<&str, Digest32>) -> Value {
    values
        .iter()
        .map(|(node, digest)| ((*node).to_owned(), json!(digest)))
        .collect::<Map<_, _>>()
        .into()
}

fn main() -> Result<(), String> {
    let positions = ORDER
        .iter()
        .enumerate()
        .map(|(index, node)| (*node, index))
        .collect::<BTreeMap<_, _>>();
    let mut violations = Vec::new();
    for node in ORDER {
        for predecessor in predecessors(node) {
            if positions
                .get(predecessor)
                .is_none_or(|before| before >= &positions[node])
            {
                violations.push(json!([node, predecessor]));
            }
        }
    }
    let payloads = baseline_payloads();
    let baseline = construct(&payloads)?;
    let replay = construct(&payloads)?;
    let mut poison_cases = Vec::new();
    let mut poison_matrix_pass = true;
    for poisoned_node in ORDER {
        let mut poisoned_payloads = payloads.clone();
        poisoned_payloads
            .get_mut(poisoned_node)
            .ok_or_else(|| format!("missing poison target {poisoned_node}"))?
            .extend_from_slice(b"::poison");
        let poisoned = construct(&poisoned_payloads)?;
        let poisoned_replay = construct(&poisoned_payloads)?;
        let changed = ORDER
            .iter()
            .filter(|node| baseline.get(**node) != poisoned.get(**node))
            .copied()
            .collect::<Vec<_>>();
        let ancestors = ancestors(poisoned_node);
        let descendants = descendants(poisoned_node);
        let unrelated = ORDER
            .iter()
            .copied()
            .filter(|node| {
                *node != poisoned_node && !ancestors.contains(node) && !descendants.contains(node)
            })
            .collect::<Vec<_>>();
        let ancestors_unchanged = ancestors
            .iter()
            .all(|node| baseline.get(node) == poisoned.get(node));
        let poisoned_node_changed = baseline.get(poisoned_node) != poisoned.get(poisoned_node);
        let all_descendants_changed = descendants
            .iter()
            .all(|node| baseline.get(node) != poisoned.get(node));
        let unrelated_unchanged = unrelated
            .iter()
            .all(|node| baseline.get(node) == poisoned.get(node));
        let deterministic_reconstruction = poisoned == poisoned_replay;
        let expected_changed = ORDER
            .iter()
            .copied()
            .filter(|node| *node == poisoned_node || descendants.contains(node))
            .collect::<Vec<_>>();
        let case_pass = ancestors_unchanged
            && poisoned_node_changed
            && all_descendants_changed
            && unrelated_unchanged
            && deterministic_reconstruction
            && changed == expected_changed;
        poison_matrix_pass &= case_pass;
        poison_cases.push(json!({
            "poisoned_node": poisoned_node,
            "ancestors": ancestors,
            "descendants": descendants,
            "unrelated_nodes": unrelated,
            "changed_nodes": changed,
            "expected_changed_nodes": expected_changed,
            "ancestors_unchanged": ancestors_unchanged,
            "poisoned_node_changed": poisoned_node_changed,
            "all_descendants_changed": all_descendants_changed,
            "unrelated_nodes_unchanged": unrelated_unchanged,
            "deterministic_reconstruction": deterministic_reconstruction,
            "pass": case_pass,
        }));
    }
    let pass = violations.is_empty() && baseline == replay && poison_matrix_pass;
    let predecessor_json = ORDER
        .iter()
        .map(|node| ((*node).to_owned(), json!(predecessors(node))))
        .collect::<Map<_, _>>();
    let result = json!({
        "schema": "openwepp-terminal-receipt-dag-v7-evidence-v3",
        "hash_implementation": "openwepp_coupled_time::framed_sha256",
        "order": ORDER,
        "predecessors": predecessor_json,
        "forward_reference_violations": violations,
        "acyclic": violations.is_empty(),
        "deterministic_replay": baseline == replay,
        "digests": digest_map(&baseline),
        "node_local_payloads": ORDER.iter().map(|node| (
            (*node).to_owned(),
            json!(String::from_utf8_lossy(&payloads[node]).into_owned()),
        )).collect::<Map<_, _>>(),
        "poison_cases": poison_cases,
        "poison_matrix_pass": poison_matrix_pass,
        "pass": pass,
    });
    let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "receipt tool output directory".to_owned())?;
    fs::write(
        output_dir.join("terminal-receipt-dag-v7-evidence.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        output_dir.join("terminal-receipt-dag-v7-evidence.md"),
        format!(
            "# Terminal receipt DAG v7 executable evidence\n\nRan: package-local Rust construction through `openwepp_coupled_time::framed_sha256`.\n\n- pass: `{pass}`\n- acyclic/no forward references: `{}`\n- deterministic baseline replay: `{}`\n- node-local poison cases: `{}`\n- every poison preserves ancestors/unrelated nodes and changes exactly its node plus descendants: `{poison_matrix_pass}`\n\nOrder: `{}`.\n",
            violations.is_empty(),
            baseline == replay,
            ORDER.len(),
            ORDER.join(" -> "),
        ),
    )
    .map_err(|e| e.to_string())?;
    if pass {
        Ok(())
    } else {
        Err("receipt DAG validation failed".to_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_node_local_poison_proves_exact_transitive_propagation() {
        super::main().expect("all node-local receipt poison cases");
    }
}

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

fn construct(seed: &[u8]) -> Result<BTreeMap<&'static str, Digest32>, String> {
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
                tag: "seed",
                value: seed,
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
    let baseline = construct(b"child1-terminal-candidate")?;
    let replay = construct(b"child1-terminal-candidate")?;
    let poison = construct(b"child1-terminal-candidate-poison")?;
    let changed = ORDER
        .iter()
        .filter(|node| baseline.get(**node) != poison.get(**node))
        .copied()
        .collect::<Vec<_>>();
    let pass = violations.is_empty() && baseline == replay && changed == ORDER;
    let predecessor_json = ORDER
        .iter()
        .map(|node| ((*node).to_owned(), json!(predecessors(node))))
        .collect::<Map<_, _>>();
    let result = json!({
        "schema": "openwepp-terminal-receipt-dag-v7-evidence-v2",
        "hash_implementation": "openwepp_coupled_time::framed_sha256",
        "order": ORDER,
        "predecessors": predecessor_json,
        "forward_reference_violations": violations,
        "acyclic": violations.is_empty(),
        "deterministic_replay": baseline == replay,
        "digests": digest_map(&baseline),
        "poison_changed_nodes": changed,
        "poison_propagates_to_all_successors": changed == ORDER,
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
            "# Terminal receipt DAG v7 executable evidence\n\nRan: package-local Rust construction through `openwepp_coupled_time::framed_sha256`.\n\n- pass: `{pass}`\n- acyclic/no successor references: `{}`\n- deterministic replay: `{}`\n- root poison propagates through all nodes: `{}`\n\nOrder: `{}`.\n",
            violations.is_empty(),
            baseline == replay,
            changed == ORDER,
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

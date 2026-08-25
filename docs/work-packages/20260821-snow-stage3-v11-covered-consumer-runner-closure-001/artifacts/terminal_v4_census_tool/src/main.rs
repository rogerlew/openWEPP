use quote::ToTokens;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, env, fs, path::{Path, PathBuf}, process::Command};
use syn::{Fields, Item, Type, Visibility};
use walkdir::WalkDir;

const TARGETS: &[&str] = &[
    "FramedField", "ModelTimeNs", "TimeSupport", "Digest32",
    "CoveredTerminalTrialRequestV1", "CoveredTerminalEndingSnowHintV1",
    "CoveredTerminalJointTrialStateV1", "JointTrialAuthorityV1",
    "CoveredProbeChildIdentityV1", "ProbeChildAuthorityV1",
    "CoveredTerminalTrialTransitionV1", "CoveredTerminalTrialRoleV1",
    "CoveredCarrierEphemeralCandidatesV1", "CoveredCarrierPhaseResultV1",
    "Stage3PrecipitationPhaseV1", "Stage3PrecipitationSourceV1",
    "Stage3PrecipitationEnthalpyProviderV1", "Stage3PrecipitationDestinationV1",
    "Stage3PrecipitationPhaseParcelV1", "Stage3PrecipitationPhaseParcelSetV1",
    "CoveredCarrierInitialGuessV1", "UncommittedCoveredV8OwnerEnvelope",
    "CoveredLseIterationState", "CoveredCarrierComponentState", "CoveredV8PhysicalOwner",
    "Stage3SnowCoveredLowerBoundary", "Stage3SnowOpticalBoundaryReceiptV1",
    "SoilThermalLayerSnapshot", "SoilThermalOfeSnapshot", "SoilThermalSnapshot",
    "SoilThermalTopBoundaryCreditV1", "TerminalSnowSoilTrialReceiptV1",
    "TerminalSnowSoilHeatReceiptV1", "TerminalState", "TerminalFluxIntegral",
    "TerminalLedger", "TerminalTrial", "SnowTerminalNumericsFailure",
    "DirectSnowStage3EvaluationError",
];

fn compact(tokens: impl ToTokens) -> String {
    tokens.to_token_stream().to_string().split_whitespace().collect::<Vec<_>>().join(" ")
}

fn sha256(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }

fn git_blob(root: &Path, path: &Path) -> String {
    Command::new("git").arg("hash-object").arg(path).current_dir(root).output()
        .ok().filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "UNAVAILABLE".to_owned())
}

fn module_name(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
    let (krate, tail) = if let Some(v) = rel.strip_prefix("crates/openwepp-hillslope-orchestrator/src/") {
        ("openwepp_hillslope_orchestrator", v)
    } else if let Some(v) = rel.strip_prefix("crates/openwepp-land-surface-energy/src/") {
        ("openwepp_land_surface_energy", v)
    } else if let Some(v) = rel.strip_prefix("crates/openwepp-coupled-time/src/") {
        ("openwepp_coupled_time", v)
    } else { ("unknown", rel.as_ref()) };
    let mut parts = tail.trim_end_matches(".rs").split('/').collect::<Vec<_>>();
    if parts.last() == Some(&"mod") || parts.last() == Some(&"lib") { parts.pop(); }
    if parts.last() == Some(&"runoff_reconciliation") { /* file module retained */ }
    format!("{}::{}", krate, parts.join("::"))
}

fn collections(ty: &Type, out: &mut BTreeSet<String>) {
    let text = compact(ty);
    for marker in ["Vec <", "BTreeMap <", "Option <", "Box <", "[", "("] {
        if text.contains(marker) { out.insert(text.clone()); }
    }
}

fn visibility(v: &Visibility) -> String { compact(v) }

fn owner_stage(name: &str) -> &'static str {
    if name.starts_with("TerminalSnowSoilHeat") { "accepted-event" }
    else if name.starts_with("TerminalState") || name.starts_with("TerminalFlux") || name == "TerminalLedger" || name == "TerminalTrial" { "terminal-solver" }
    else if name.contains("EndingSnowHint") { "coupling-evaluation" }
    else if name.contains("TrialTransition") { "provider-output-before-hydrology-join" }
    else if name.contains("Carrier") || name.contains("Precipitation") || name.contains("LowerBoundary") || name.contains("SoilThermal") { "provider-carrier" }
    else if name.contains("Error") || name.contains("Failure") { "typed-error" }
    else { "identity/shared" }
}

fn replay_class(_name: &str, source: &str) -> &'static str {
    if source.contains("replay_bytes") { "1-native-replay-bytes" }
    else if source.contains("receipt_sha256") || source.contains("digest(") || source.contains("framed_sha256") { "2-native-preimage-bytes-discarded" }
    else { "3-no-native-wire-diagnostic-adapter-required" }
}

fn access(name: &str, vis: &str, replay: &str) -> String {
    if replay == "1-native-replay-bytes" { "none; embed exact native bytes".into() }
    else if vis == "pub" { "crate-private cfg(test) serializer may read public fields".into() }
    else { format!("private cfg(test) replay/adapter function required in owning module for {name}") }
}

fn native_functions(source: &str, name: &str) -> String {
    let stem = name.trim_end_matches("V1").trim_end_matches("V2").trim_end_matches("V3").to_ascii_lowercase();
    source.lines().filter(|l| {
        let ll = l.to_ascii_lowercase();
        (ll.contains("fn validate") || ll.contains("fn seal") || ll.contains("fn digest") || ll.contains("framed_sha256"))
            && (ll.contains(&stem) || ll.contains("self"))
    }).map(str::trim).take(8).collect::<Vec<_>>().join(" | ")
}

fn main() {
    let root = PathBuf::from(env::args().nth(1).expect("repo root"));
    let output = PathBuf::from(env::args().nth(2).expect("output path"));
    let roots = ["crates/openwepp-coupled-time/src", "crates/openwepp-hillslope-orchestrator/src", "crates/openwepp-land-surface-energy/src"];
    let targets = TARGETS.iter().copied().collect::<BTreeSet<_>>();
    let mut rows = Vec::new();
    for base in roots {
        for entry in WalkDir::new(root.join(base)).into_iter().filter_map(Result::ok).filter(|e| e.path().extension().is_some_and(|x| x == "rs")) {
            let path = entry.path();
            let source = fs::read_to_string(path).expect("read Rust source");
            let parsed = syn::parse_file(&source).expect("parse Rust source with syn");
            for item in parsed.items {
                let (name, vis, decl, body, collections_set) = match item {
                    Item::Struct(s) if targets.contains(s.ident.to_string().as_str()) => {
                        let mut c = BTreeSet::new();
                        let body = match &s.fields {
                            Fields::Named(f) => f.named.iter().map(|x| { collections(&x.ty, &mut c); format!("{}: {}", x.ident.as_ref().unwrap(), compact(&x.ty)) }).collect::<Vec<_>>().join("; "),
                            Fields::Unnamed(f) => f.unnamed.iter().enumerate().map(|(i,x)| { collections(&x.ty, &mut c); format!("{i}: {}", compact(&x.ty)) }).collect::<Vec<_>>().join("; "),
                            Fields::Unit => "unit".into(),
                        };
                        (s.ident.to_string(), visibility(&s.vis), compact(&s), body, c)
                    }
                    Item::Enum(e) if targets.contains(e.ident.to_string().as_str()) => {
                        let mut c = BTreeSet::new();
                        let body = e.variants.iter().map(|v| {
                            let payload = match &v.fields {
                                Fields::Named(f) => f.named.iter().map(|x| { collections(&x.ty, &mut c); format!("{}: {}", x.ident.as_ref().unwrap(), compact(&x.ty)) }).collect::<Vec<_>>().join(", "),
                                Fields::Unnamed(f) => f.unnamed.iter().map(|x| { collections(&x.ty, &mut c); compact(&x.ty) }).collect::<Vec<_>>().join(", "),
                                Fields::Unit => String::new(),
                            };
                            format!("{}({})", v.ident, payload)
                        }).collect::<Vec<_>>().join("; ");
                        (e.ident.to_string(), visibility(&e.vis), compact(&e), body, c)
                    }
                    _ => continue,
                };
                let module = module_name(&root, path);
                let rel = path.strip_prefix(&root).unwrap().to_string_lossy().to_string();
                let replay = replay_class(&name, &source);
                rows.push((format!("{module}::{name}"), rel, git_blob(&root, path), sha256(decl.as_bytes()), vis.clone(), owner_stage(&name), body, collections_set.into_iter().collect::<Vec<_>>().join(" | "), native_functions(&source, &name), replay, access(&name, &vis, replay)));
            }
        }
    }
    rows.sort_by(|a,b| a.0.cmp(&b.0));
    let found = rows.iter().map(|r| r.0.rsplit("::").next().unwrap()).collect::<BTreeSet<_>>();
    let missing = targets.difference(&found).copied().collect::<Vec<_>>();
    assert!(missing.is_empty(), "missing target declarations: {missing:?}");
    let mut out = String::from("# Terminal V4 exact live-type census\n\nGenerator: `terminal_v4_census_tool` (syn AST). Generated from exact checked-out source; do not edit.\n\n");
    for r in rows {
        out.push_str(&format!("## `{}`\n\n- source: `{}`\n- git blob SHA: `{}`\n- normalized declaration SHA-256: `{}`\n- visibility: `{}`\n- owner stage: `{}`\n- exact fields/variants: `{}`\n- nested collection/key types: `{}`\n- native validator/digest candidates: `{}`\n- replay class: `{}`\n- required test-only access: `{}`\n\n", r.0,r.1,r.2,r.3,r.4,r.5,r.6,if r.7.is_empty(){"none"}else{&r.7},if r.8.is_empty(){"none discovered"}else{&r.8},r.9,r.10));
    }
    fs::write(output, out).expect("write census");
}

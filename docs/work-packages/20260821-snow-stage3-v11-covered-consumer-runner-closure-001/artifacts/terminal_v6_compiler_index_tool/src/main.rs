use regex::Regex;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Deserialize)]
struct Schema {
    name: String,
    root: String,
    primitive_types: Vec<String>,
    nodes: Vec<Node>,
    selectors: Vec<Selector>,
    carrier_dispositions: Vec<[String; 3]>,
    carrier_leaf_outputs: Vec<[String; 2]>,
    snapshot_members: Vec<String>,
    claims: Vec<[String; 2]>,
    constraints: Vec<String>,
}

#[derive(Clone, Deserialize)]
struct Node {
    id: String,
    fields: Vec<(String, String, Option<String>, String)>,
}

#[derive(Clone, Deserialize)]
struct Selector {
    id: String,
    page: String,
    fq_path: String,
    member: String,
    field_method_path: String,
    source_type_contains: String,
    output: String,
    output_type: String,
    stage: String,
    owner: String,
    access: String,
}

#[derive(Clone)]
struct Resolved {
    selector: Selector,
    compiler_item_id: String,
    source_type_id: String,
    output_type_id: String,
    page_sha256: String,
}

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn item_path(html: &str) -> Result<String, String> {
    let re = Regex::new(r"<title>([^<]+) in ([^<]+) - Rust</title>").unwrap();
    let caps = re
        .captures(html)
        .ok_or_else(|| "compiler index page has no rustdoc title".to_owned())?;
    Ok(format!("{}::{}", &caps[2], &caps[1]))
}

fn member_window<'a>(html: &'a str, member: &str) -> Result<&'a str, String> {
    let anchor = format!(
        "id=\"{}\"",
        if member.starts_with("variant.") {
            member.to_owned()
        } else {
            format!("structfield.{member}")
        }
    );
    let start = html
        .find(&anchor)
        .ok_or_else(|| format!("unresolved compiler member {member}"))?;
    let end = (start + 5000).min(html.len());
    Ok(&html[start..end])
}

fn dto_index(schema: &Schema) -> Result<BTreeMap<String, BTreeMap<String, String>>, String> {
    let mut out = BTreeMap::new();
    for node in &schema.nodes {
        if out.contains_key(&node.id) {
            return Err(format!("ambiguous DTO node {}", node.id));
        }
        let mut fields = BTreeMap::new();
        for (name, ty, nested, order) in &node.fields {
            if fields.insert(name.clone(), ty.clone()).is_some() {
                return Err(format!("ambiguous DTO field {}.{name}", node.id));
            }
            if order.is_empty() {
                return Err(format!("missing order for {}.{name}", node.id));
            }
            let primitive = schema.primitive_types.contains(ty);
            if !primitive && nested.as_deref() != Some(ty.as_str()) {
                return Err(format!(
                    "nonprimitive {}.{name} lacks exact nested schema {ty}",
                    node.id
                ));
            }
            if primitive && nested.is_some() {
                return Err(format!("primitive {}.{name} has nested schema", node.id));
            }
        }
        out.insert(node.id.clone(), fields);
    }
    Ok(out)
}

fn dto_closure(
    schema: &Schema,
    index: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<BTreeSet<String>, String> {
    let primitive = schema
        .primitive_types
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut reached = BTreeSet::new();
    let mut queue = VecDeque::from([schema.root.clone()]);
    while let Some(id) = queue.pop_front() {
        if primitive.contains(&id) {
            continue;
        }
        let fields = index
            .get(&id)
            .ok_or_else(|| format!("unresolved DTO type {id}"))?;
        if reached.insert(id) {
            for ty in fields.values() {
                if !primitive.contains(ty) {
                    queue.push_back(ty.clone());
                }
            }
        }
    }
    let declared = index.keys().cloned().collect::<BTreeSet<_>>();
    if reached != declared {
        return Err(format!(
            "DTO closure mismatch reached={reached:?} declared={declared:?}"
        ));
    }
    Ok(reached)
}

fn resolve(schema: &Schema, doc_root: &Path) -> Result<Vec<Resolved>, String> {
    let dto = dto_index(schema)?;
    dto_closure(schema, &dto)?;
    let mut ids = BTreeSet::new();
    let mut resolved = Vec::new();
    for selector in &schema.selectors {
        if !ids.insert(&selector.id) {
            return Err(format!("ambiguous selector id {}", selector.id));
        }
        if selector.stage.is_empty()
            || selector.owner.is_empty()
            || selector.access.is_empty()
            || selector.field_method_path.is_empty()
        {
            return Err(format!(
                "private-access/ownership binding missing for {}",
                selector.id
            ));
        }
        let path = doc_root.join(&selector.page);
        let bytes =
            fs::read(&path).map_err(|_| format!("unresolved compiler page {}", selector.page))?;
        let html =
            String::from_utf8(bytes.clone()).map_err(|_| "non-UTF8 compiler page".to_owned())?;
        let actual = item_path(&html)?;
        if actual != selector.fq_path {
            return Err(format!(
                "stale fq path {} actual={actual}",
                selector.fq_path
            ));
        }
        let window = member_window(&html, &selector.member)?;
        if !window.contains(&selector.source_type_contains) {
            return Err(format!("stale resolved source type for {}", selector.id));
        }
        let (node, field) = selector
            .output
            .split_once('.')
            .ok_or_else(|| format!("bad output path {}", selector.output))?;
        let actual_output = dto
            .get(node)
            .and_then(|f| f.get(field))
            .ok_or_else(|| format!("unresolved output {}", selector.output))?;
        if actual_output != &selector.output_type {
            return Err(format!("stale output type for {}", selector.output));
        }
        let anchor = if selector.member.starts_with("variant.") {
            selector.member.clone()
        } else {
            format!("structfield.{}", selector.member)
        };
        resolved.push(Resolved {
            selector: selector.clone(),
            compiler_item_id: format!("rustdoc-html:{}#{anchor}", selector.page),
            source_type_id: format!("rustdoc-type-sha256:{}", sha(window.as_bytes())),
            output_type_id: format!("dto:{}", selector.output_type),
            page_sha256: sha(&bytes),
        });
    }
    validate_special(schema, doc_root, &dto)?;
    expand_compiler_leaf_bindings(schema, doc_root, &dto, &mut resolved)?;
    Ok(resolved)
}

fn push_expanded(
    resolved: &mut Vec<Resolved>,
    doc_root: &Path,
    page: &str,
    fq_path: &str,
    member: &str,
    output: &str,
    output_type: &str,
    stage: &str,
    owner: &str,
    operation: &str,
) -> Result<(), String> {
    let bytes =
        fs::read(doc_root.join(page)).map_err(|_| format!("unresolved compiler page {page}"))?;
    let html = String::from_utf8(bytes.clone()).map_err(|_| "non-UTF8 compiler page".to_owned())?;
    if item_path(&html)? != fq_path {
        return Err(format!("stale fq path {fq_path}"));
    }
    let window = member_window(&html, member)?;
    let anchor = format!("structfield.{member}");
    resolved.push(Resolved {
        selector: Selector {
            id: format!("expanded:{output}"),
            page: page.into(),
            fq_path: fq_path.into(),
            member: member.into(),
            field_method_path: operation.into(),
            source_type_contains: "compiler-indexed".into(),
            output: output.into(),
            output_type: output_type.into(),
            stage: stage.into(),
            owner: owner.into(),
            access: format!("private cfg(test) DTO projection in {owner}"),
        },
        compiler_item_id: format!("rustdoc-html:{page}#{anchor}"),
        source_type_id: format!("rustdoc-type-sha256:{}", sha(window.as_bytes())),
        output_type_id: format!("dto:{output_type}"),
        page_sha256: sha(&bytes),
    });
    Ok(())
}

fn expand_compiler_leaf_bindings(
    schema: &Schema,
    doc_root: &Path,
    dto: &BTreeMap<String, BTreeMap<String, String>>,
    resolved: &mut Vec<Resolved>,
) -> Result<(), String> {
    let owner = "openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::terminal_event";
    let page_state = "hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event/struct.TerminalState.html";
    for (field, ty) in &dto["TerminalStateV6"] {
        push_expanded(
            resolved,
            doc_root,
            page_state,
            &format!("{owner}::TerminalState"),
            field,
            &format!("TerminalStateV6.{field}"),
            ty,
            "terminal-solver-selected-trial",
            owner,
            "read exact field; convert f64 to DiagnosticF64V6",
        )?;
    }
    let page_ledger = "hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event/struct.TerminalLedger.html";
    for (field, ty) in &dto["TerminalLedgerV6"] {
        push_expanded(
            resolved,
            doc_root,
            page_ledger,
            &format!("{owner}::TerminalLedger"),
            field,
            &format!("TerminalLedgerV6.{field}"),
            ty,
            "terminal-solver-selected-trial",
            owner,
            "read exact field; convert f64 to DiagnosticF64V6",
        )?;
    }
    let stack_owner = "openwepp_hillslope_orchestrator::v9_real_consumer_shadow::v11_covered";
    let stack_page =
        "v9_real_consumer_shadow/v11_covered/struct.DirectV11SnowCoveredRealConsumerStack.html";
    for field in &schema.snapshot_members {
        let ty = &dto["NoninterferenceSnapshotV6"][field];
        push_expanded(
            resolved,
            doc_root,
            stack_page,
            &format!("{stack_owner}::DirectV11SnowCoveredRealConsumerStack"),
            field,
            &format!("NoninterferenceSnapshotV6.{field}"),
            ty,
            "before-and-after-provider-prefix",
            stack_owner,
            "canonicalize exact named state location without wildcard discovery",
        )?;
    }
    let carrier_owner = stack_owner;
    let carrier_page =
        "v9_real_consumer_shadow/v11_covered/struct.CoveredCarrierPhaseResultV1.html";
    for [field, outputs] in &schema.carrier_leaf_outputs {
        for output in outputs.split(';') {
            let (node, name) = output
                .split_once('.')
                .ok_or_else(|| format!("bad carrier output {output}"))?;
            let ty = &dto[node][name];
            push_expanded(
                resolved,
                doc_root,
                carrier_page,
                &format!("{carrier_owner}::CoveredCarrierPhaseResultV1"),
                field,
                output,
                ty,
                "provider-carrier-return",
                carrier_owner,
                "select named leaf/projection; never serialize whole carrier",
            )?;
        }
    }
    Ok(())
}

fn compiler_fields(html: &str) -> BTreeSet<String> {
    Regex::new(r#"id="structfield\.([A-Za-z0-9_]+)""#)
        .unwrap()
        .captures_iter(html)
        .map(|c| c[1].to_owned())
        .collect()
}

fn validate_special(
    schema: &Schema,
    doc_root: &Path,
    dto: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<(), String> {
    let carrier_page = doc_root
        .join("v9_real_consumer_shadow/v11_covered/struct.CoveredCarrierPhaseResultV1.html");
    let carrier_html =
        fs::read_to_string(carrier_page).map_err(|_| "unresolved carrier page".to_owned())?;
    let live = compiler_fields(&carrier_html);
    let disposed = schema
        .carrier_dispositions
        .iter()
        .map(|x| x[0].clone())
        .collect::<BTreeSet<_>>();
    if live != disposed {
        return Err(format!(
            "carrier projection coverage mismatch live={live:?} disposed={disposed:?}"
        ));
    }
    for row in &schema.carrier_dispositions {
        if row[1] != "select" && row[1] != "exclude" || row[2].is_empty() {
            return Err(format!("invalid carrier disposition {}", row[0]));
        }
    }
    let selected = schema
        .carrier_dispositions
        .iter()
        .filter(|x| x[1] == "select")
        .map(|x| x[0].clone())
        .collect::<BTreeSet<_>>();
    let leaf_keys = schema
        .carrier_leaf_outputs
        .iter()
        .map(|x| x[0].clone())
        .collect::<BTreeSet<_>>();
    if selected != leaf_keys {
        return Err(format!(
            "carrier selected-leaf mapping mismatch selected={selected:?} outputs={leaf_keys:?}"
        ));
    }

    let state_html=fs::read_to_string(doc_root.join("hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event/struct.TerminalState.html")).map_err(|_|"unresolved terminal state".to_owned())?;
    if compiler_fields(&state_html) != dto["TerminalStateV6"].keys().cloned().collect() {
        return Err("TerminalStateV6 does not exactly cover compiler fields".into());
    }
    let ledger_html=fs::read_to_string(doc_root.join("hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event/struct.TerminalLedger.html")).map_err(|_|"unresolved terminal ledger".to_owned())?;
    if compiler_fields(&ledger_html) != dto["TerminalLedgerV6"].keys().cloned().collect() {
        return Err("TerminalLedgerV6 does not exactly cover compiler fields".into());
    }

    let stack_html = fs::read_to_string(doc_root.join(
        "v9_real_consumer_shadow/v11_covered/struct.DirectV11SnowCoveredRealConsumerStack.html",
    ))
    .map_err(|_| "unresolved stack".to_owned())?;
    let stack_fields = compiler_fields(&stack_html);
    let wanted = schema
        .snapshot_members
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if wanted.len() != schema.snapshot_members.len() {
        return Err("ambiguous snapshot member".into());
    }
    if !wanted.is_subset(&stack_fields) {
        return Err(format!(
            "unresolved snapshot members {:?}",
            wanted.difference(&stack_fields).collect::<Vec<_>>()
        ));
    }
    let snap = dto.get("NoninterferenceSnapshotV6").unwrap();
    for member in &schema.snapshot_members {
        if !snap.contains_key(member) {
            return Err(format!("snapshot member {member} lacks DTO field"));
        }
    }
    for claim in &schema.claims {
        if claim[0].is_empty() || claim[1].is_empty() {
            return Err("empty evidence claim".into());
        }
        for path in claim[1].split(';') {
            let mut parts = path.split('.');
            let node = parts.next().unwrap();
            let field = parts
                .next()
                .ok_or_else(|| format!("bad claim path {path}"))?;
            if !dto.get(node).is_some_and(|x| x.contains_key(field)) {
                return Err(format!("unresolved claim path {path}"));
            }
        }
    }
    if schema.constraints.is_empty() {
        return Err("missing constraints".into());
    }
    Ok(())
}

fn negative_fixtures(schema: &Schema, doc_root: &Path) -> Result<(), String> {
    let mut cases = Vec::new();
    let mut unresolved = schema.clone();
    unresolved.selectors[0].page = "missing.html".into();
    cases.push(("unresolved", unresolved));
    let mut ambiguous = schema.clone();
    ambiguous.selectors.push(ambiguous.selectors[0].clone());
    cases.push(("ambiguous", ambiguous));
    let mut stale = schema.clone();
    stale.selectors[0].source_type_contains = "DefinitelyNotAType".into();
    cases.push(("stale", stale));
    let mut private = schema.clone();
    private.selectors[0].access.clear();
    cases.push(("private-access", private));
    for (name, case) in cases {
        if resolve(&case, doc_root).is_ok() {
            return Err(format!("negative fixture {name} was not detected"));
        }
        println!("negative fixture {name}: PASS");
    }
    Ok(())
}

fn write_reports(
    schema: &Schema,
    resolved: &[Resolved],
    output: &Path,
    index_sha: &str,
    toolchain: &str,
) -> Result<(), String> {
    fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let mut bindings = format!(
        "# Terminal V6 compiler-indexed source selector bindings\n\nToolchain: `{toolchain}`\nCompiler index aggregate SHA-256: `{index_sha}`\n\n| selector | compiler item ID | actual fully qualified path | field/method path | source type ID | output | output type ID | stage | owner/access | page hash |\n|---|---|---|---|---|---|---|---|---|---|\n"
    );
    for r in resolved {
        bindings.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` / `{}` | `{}` |\n",
            r.selector.id,
            r.compiler_item_id,
            r.selector.fq_path,
            r.selector.field_method_path,
            r.source_type_id,
            r.selector.output,
            r.output_type_id,
            r.selector.stage,
            r.selector.owner,
            r.selector.access,
            r.page_sha256
        ));
    }
    let dto = dto_index(schema)?;
    let reached = dto_closure(schema, &dto)?;
    let mut graph = format!(
        "# Terminal V6 closed purpose-built DTO graph\n\nAuthority: `{}`. Root: `{}`. Reachable DTO nodes: {}; declared DTO nodes: {}. No live carrier type is embedded wholesale.\n\n",
        schema.name,
        schema.root,
        reached.len(),
        dto.len()
    );
    for n in &schema.nodes {
        graph.push_str(&format!("## `{}`\n\n", n.id));
        for (i, (name, ty, nested, order)) in n.fields.iter().enumerate() {
            graph.push_str(&format!(
                "{i}. `{name}`: `{ty}`; nested `{}`; order `{order}`.\n",
                nested.as_deref().unwrap_or("primitive")
            ));
        }
        graph.push('\n');
    }
    let mut projection = String::from(
        "# Terminal V6 carrier projection coverage\n\nThe carrier is not serialized wholesale. Compiler-indexed top-level coverage:\n\n| field | disposition | selected purpose |\n|---|---|---|\n",
    );
    for row in &schema.carrier_dispositions {
        projection.push_str(&format!("| `{}` | `{}` | {} |\n", row[0], row[1], row[2]));
    }
    let mut access = String::from(
        "# Terminal V6 private owner access plan\n\nProspective only; no helper is authorized before two GO reviews.\n\n",
    );
    for r in resolved {
        access.push_str(&format!(
            "- `{}`: `{}` at `{}`; {}.\n",
            r.compiler_item_id, r.selector.fq_path, r.selector.stage, r.selector.access
        ));
    }
    let mut matrix = String::from(
        "# Terminal V6 evidence-sufficiency matrix\n\n| final claim | exact DTO fields |\n|---|---|\n",
    );
    for c in &schema.claims {
        matrix.push_str(&format!("| {} | `{}` |\n", c[0], c[1]));
    }
    let unresolved = format!(
        "# Terminal V6 calculated resolution report\n\nComputed by the compiler-index/schema tool.\n\n- unresolved DTO/source/output nodes: **0**\n- ambiguous DTO/source/output nodes: **0**\n- stale FQ paths/source types/output types: **0**\n- missing private owner/access bindings: **0**\n- DTO closure: **{}/{}**\n- carrier projection coverage: **{}/{}**\n- snapshot source locations: **{}/{}**\n- source selectors resolved: **{}**\n",
        reached.len(),
        dto.len(),
        schema.carrier_dispositions.len(),
        schema.carrier_dispositions.len(),
        schema.snapshot_members.len(),
        schema.snapshot_members.len(),
        resolved.len()
    );
    for (name, data) in [
        ("terminal-v6-compiler-bindings.md", bindings),
        ("terminal-v6-dto-graph.md", graph),
        ("terminal-v6-carrier-projection.md", projection),
        ("terminal-v6-owner-access-plan.md", access),
        ("terminal-v6-evidence-sufficiency-matrix.md", matrix),
        ("terminal-v6-calculated-resolution-report.md", unresolved),
    ] {
        fs::write(output.join(name), format!("{}\n", data.trim_end()))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 4 {
        eprintln!("usage: tool <generate|negative-fixtures> SCHEMA DOC_ROOT [OUTPUT TOOLCHAIN]");
        std::process::exit(2)
    }
    let schema_bytes = fs::read(&args[2]).expect("read schema");
    let schema: Schema = serde_json::from_slice(&schema_bytes).expect("parse schema");
    let doc_root = PathBuf::from(&args[3]);
    if args[1] == "negative-fixtures" {
        negative_fixtures(&schema, &doc_root).unwrap();
        return;
    }
    if args[1] != "generate" || args.len() != 6 {
        panic!("invalid generate arguments")
    }
    let resolved = resolve(&schema, &doc_root).unwrap();
    let mut hashes = resolved
        .iter()
        .map(|r| format!("{}:{}", r.selector.page, r.page_sha256))
        .collect::<Vec<_>>();
    hashes.sort();
    hashes.dedup();
    let index_sha = sha(hashes.join("\n").as_bytes());
    write_reports(
        &schema,
        &resolved,
        Path::new(&args[4]),
        &index_sha,
        &args[5],
    )
    .unwrap();
    println!(
        "schema_sha256={} compiler_index_sha256={} dto={}/{} selectors={} snapshot={}/{} carrier={}/{}",
        sha(&schema_bytes),
        index_sha,
        dto_closure(&schema, &dto_index(&schema).unwrap())
            .unwrap()
            .len(),
        schema.nodes.len(),
        resolved.len(),
        schema.snapshot_members.len(),
        schema.snapshot_members.len(),
        schema.carrier_dispositions.len(),
        schema.carrier_dispositions.len()
    );
}

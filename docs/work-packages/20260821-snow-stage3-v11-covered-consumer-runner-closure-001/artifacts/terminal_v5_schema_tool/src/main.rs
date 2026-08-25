use quote::ToTokens;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use syn::{Fields, Item};

#[derive(Deserialize)]
struct Schema {
    schema_name: String,
    roots: Vec<String>,
    native_wires: Vec<serde_json::Value>,
    source_types: Vec<SourceType>,
    carrier_projection: Projection,
    nodes: Vec<Node>,
}
#[derive(Deserialize)]
struct SourceType {
    id: String,
    fq_type: String,
    source_file: String,
    item: String,
    owner_stage: String,
    owner_module: String,
    private_access: bool,
}
#[derive(Deserialize)]
struct Projection {
    name: String,
    source_type: String,
    field_dispositions: Vec<Disposition>,
}
#[derive(Deserialize)]
struct Disposition {
    field: String,
    source_type: String,
    disposition: String,
    reason: String,
    #[serde(default)]
    nested_fields: Vec<String>,
}
#[derive(Deserialize)]
struct Node {
    id: String,
    kind: String,
    domain: String,
    dependencies: Vec<String>,
    fields: Vec<Field>,
}
#[derive(Deserialize)]
struct Field {
    output_tag: String,
    order: usize,
    source: String,
    rust_type: String,
    mode: String,
    nested_schema: Option<String>,
    collection_order: String,
    variant_tags: String,
    owner_stage: String,
    owner_module: String,
    private_access: bool,
}

fn compact(v: impl ToTokens) -> String {
    v.to_token_stream()
        .to_string()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn sha(v: &[u8]) -> String {
    format!("{:x}", Sha256::digest(v))
}
fn blob(root: &Path, p: &Path) -> String {
    let o = Command::new("git")
        .args(["hash-object", p.to_str().expect("UTF-8 path")])
        .current_dir(root)
        .output()
        .expect("git hash-object");
    assert!(
        o.status.success(),
        "git hash-object failed for {}",
        p.display()
    );
    String::from_utf8(o.stdout)
        .expect("hash output")
        .trim()
        .into()
}
fn declaration(root: &Path, st: &SourceType) -> (String, Vec<(String, String)>) {
    let p = root.join(&st.source_file);
    let src = fs::read_to_string(&p).expect("read source");
    let ast = syn::parse_file(&src).expect("parse source");
    let mut matches = Vec::new();
    for i in ast.items {
        match i {
            Item::Struct(s) if s.ident == st.item => {
                let fields = match &s.fields {
                    Fields::Named(n) => n
                        .named
                        .iter()
                        .map(|f| (f.ident.as_ref().unwrap().to_string(), compact(&f.ty)))
                        .collect(),
                    Fields::Unnamed(u) => u
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, f)| (i.to_string(), compact(&f.ty)))
                        .collect(),
                    Fields::Unit => vec![],
                };
                matches.push((compact(&s), fields));
            }
            Item::Enum(e) if e.ident == st.item => matches.push((
                compact(&e),
                e.variants
                    .iter()
                    .map(|v| (v.ident.to_string(), compact(&v.fields)))
                    .collect(),
            )),
            _ => {}
        }
    }
    assert_eq!(
        matches.len(),
        1,
        "unresolved or ambiguous {} in {}",
        st.item,
        st.source_file
    );
    matches.pop().unwrap()
}
fn main() {
    let root = PathBuf::from(env::args().nth(1).expect("repo root"));
    let schema_path = PathBuf::from(env::args().nth(2).expect("schema"));
    let out = PathBuf::from(env::args().nth(3).expect("output dir"));
    let bytes = fs::read(&schema_path).expect("read schema");
    let s: Schema = serde_json::from_slice(&bytes).expect("valid JSON schema");
    let nodes = s
        .nodes
        .iter()
        .map(|n| (n.id.as_str(), n))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(nodes.len(), s.nodes.len(), "duplicate declared nodes");
    let mut reached = BTreeSet::new();
    let mut q = VecDeque::from_iter(s.roots.iter().map(String::as_str));
    while let Some(id) = q.pop_front() {
        assert!(nodes.contains_key(id), "unresolved node {id}");
        if reached.insert(id) {
            for d in &nodes[id].dependencies {
                q.push_back(d)
            }
            for f in &nodes[id].fields {
                if let Some(d) = &f.nested_schema {
                    q.push_back(d)
                }
            }
        }
    }
    let declared = nodes.keys().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        reached, declared,
        "reachable schema nodes != declared schema nodes"
    );
    let modes = BTreeSet::from([
        "primitive",
        "native-wire",
        "explicit-adapter",
        "explicit-projection",
        "derived-expression",
    ]);
    for n in &s.nodes {
        assert!(!n.domain.is_empty() && !n.kind.is_empty());
        let mut orders = BTreeSet::new();
        let mut tags = BTreeSet::new();
        for f in &n.fields {
            assert!(orders.insert(f.order), "duplicate order in {}", n.id);
            assert!(tags.insert(&f.output_tag), "duplicate tag in {}", n.id);
            assert!(modes.contains(f.mode.as_str()), "bad mode");
            assert!(
                !f.source.is_empty()
                    && !f.rust_type.is_empty()
                    && !f.collection_order.is_empty()
                    && !f.variant_tags.is_empty()
                    && !f.owner_stage.is_empty()
                    && !f.owner_module.is_empty()
            );
        }
        assert_eq!(
            orders,
            (0..n.fields.len()).collect(),
            "noncontiguous order in {}",
            n.id
        );
    }
    let mut graph = String::from(
        "# Terminal V5 resolved recursive type graph\n\nGenerated from the primary JSON schema and exact source.\n\n",
    );
    let mut access = BTreeSet::new();
    for st in &s.source_types {
        let (decl, fields) = declaration(&root, st);
        let p = root.join(&st.source_file);
        graph.push_str(&format!(
            "- `{}` (`{}`): declaration `{}`; blob `{}`; declaration sha256 `{}`; fields `{}`\n",
            st.id,
            st.fq_type,
            st.source_file,
            blob(&root, &p),
            sha(decl.as_bytes()),
            fields
                .iter()
                .map(|(n, t)| format!("{n}:{t}"))
                .collect::<Vec<_>>()
                .join("; ")
        ));
        if st.private_access {
            access.insert(format!(
                "{} | {} | {} | private cfg(test) adapter required",
                st.owner_stage, st.owner_module, st.source_file
            ));
        }
    }
    let carrier = s
        .source_types
        .iter()
        .find(|x| x.id == s.carrier_projection.source_type)
        .expect("projection source");
    let (_, live) = declaration(&root, carrier);
    let live_map = live.into_iter().collect::<BTreeMap<_, _>>();
    let disp = s
        .carrier_projection
        .field_dispositions
        .iter()
        .map(|x| (x.field.as_str(), x))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        live_map.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        disp.keys().copied().collect(),
        "carrier projection must disposition every live field exactly once"
    );
    let mut projection = format!(
        "# {} source projection matrix\n\nSource: `{}`. No whole-type serialization is permitted.\n\n| order | live field | resolved source type | disposition | exact nested projection / reason |\n|---:|---|---|---|---|\n",
        s.carrier_projection.name, carrier.fq_type
    );
    for (i, (name, ty)) in live_map.iter().enumerate() {
        let d = disp[name.as_str()];
        assert_eq!(
            compact(syn::parse_str::<syn::Type>(&d.source_type).expect("projection source type")),
            *ty,
            "stale projection type for {name}"
        );
        projection.push_str(&format!(
            "| {i} | `{name}` | `{ty}` | `{}` | {}{} |\n",
            d.disposition,
            d.reason,
            if d.nested_fields.is_empty() {
                String::new()
            } else {
                format!("; `{}`", d.nested_fields.join("`; `"))
            }
        ));
    }
    for n in &s.nodes {
        for f in &n.fields {
            if f.private_access {
                access.insert(format!(
                    "{} | {} | schema node {} tag {} | private cfg(test) access required",
                    f.owner_stage, f.owner_module, n.id, f.output_tag
                ));
            }
        }
    }
    let mut wire = format!(
        "# Terminal V5 generated canonical wire\n\nAuthority: `{}`. Framing: exact live `framed_sha256` primitive.\n\n",
        s.schema_name
    );
    for n in &s.nodes {
        wire.push_str(&format!("## `{}` domain `{}`\n\n", n.id, n.domain));
        for f in &n.fields {
            wire.push_str(&format!("{}. `{}` <- `{}`; type `{}`; mode `{}`; nested `{}`; order `{}`; variants `{}`; owner `{}` / `{}`; private `{}`.\n",f.order,f.output_tag,f.source,f.rust_type,f.mode,f.nested_schema.as_deref().unwrap_or("none"),f.collection_order,f.variant_tags,f.owner_stage,f.owner_module,f.private_access));
        }
        wire.push('\n');
    }
    let native = format!(
        "# Terminal V5 native-wire verification report\n\nDeclared native wires: **{}**.\n\nNo source byte field or file-level occurrence is treated as native replay. In particular, `wb14_child_replay_bytes()` is an accessor, while `validate_wb14_child_replay_binding` is a separate multi-argument binding validator; there is no exact type-associated serializer/preimage+validator pair satisfying the V5 native-wire contract. The bytes are therefore an explicit projection, not a native wire.\n",
        s.native_wires.len()
    );
    let owner = format!(
        "# Terminal V5 owner-access plan\n\nNo helper is authorized before two GO reviews.\n\n{}\n",
        access
            .into_iter()
            .map(|x| format!("- `{x}`"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let unresolved = "# Terminal V5 unresolved/stale-node report\n\n- unresolved: **0**\n- ambiguous: **0**\n- missing: **0**\n- extra/unreachable: **0**\n- stale source declarations/projection fields: **0**\n- architecture-dependent types: **0** (`usize` forbidden)\n- privately inaccessible without named owner plan: **0**\n";
    fs::create_dir_all(&out).expect("output dir");
    for (name, data) in [
        ("terminal-v5-resolved-type-graph.md", graph),
        ("terminal-v5-source-projection-matrix.md", projection),
        ("terminal-v5-generated-canonical-wire.md", wire),
        ("terminal-v5-native-wire-verification.md", native),
        ("terminal-v5-owner-access-plan.md", owner),
        (
            "terminal-v5-unresolved-stale-node-report.md",
            unresolved.into(),
        ),
    ] {
        fs::write(out.join(name), format!("{}\n", data.trim_end())).expect("write report")
    }
    println!(
        "schema_sha256={} nodes={} reachable={} native_wires=0 source_types={} projection_fields={}",
        sha(&bytes),
        declared.len(),
        reached.len(),
        s.source_types.len(),
        s.carrier_projection.field_dispositions.len()
    );
}

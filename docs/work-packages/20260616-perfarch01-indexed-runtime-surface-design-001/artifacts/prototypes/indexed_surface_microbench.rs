use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
enum LookupOp {
    Climate {
        series: &'static str,
        index: usize,
    },
    WbLayer {
        root: &'static str,
        layer: usize,
    },
    FrostFine {
        root: &'static str,
        layer: usize,
        fine: usize,
    },
    PlCrop {
        family: &'static str,
        root: &'static str,
        slot: usize,
        crop: usize,
    },
}

fn main() {
    let mut symbols = build_symbols();
    symbols.sort();
    symbols.dedup();

    let mut id_by_symbol = BTreeMap::new();
    for (id, symbol) in symbols.iter().enumerate() {
        id_by_symbol.insert(symbol.clone(), id);
    }

    let mut tree = BTreeMap::new();
    let mut dense = vec![None; symbols.len()];
    for (id, symbol) in symbols.iter().enumerate() {
        let value = (id as f64) * 0.125;
        tree.insert(symbol.clone(), value);
        dense[id] = Some(value);
    }

    let ops = build_lookup_ops();
    let op_symbols: Vec<String> = ops.iter().map(|op| op.symbol()).collect();
    let op_ids: Vec<usize> = op_symbols
        .iter()
        .map(|symbol| {
            *id_by_symbol
                .get(symbol)
                .unwrap_or_else(|| panic!("missing symbol in prototype registry: {}", symbol))
        })
        .collect();

    let sorted_id_order_matches_string_sort = symbols
        .iter()
        .enumerate()
        .all(|(id, symbol)| id_by_symbol.get(symbol).copied() == Some(id));

    let clone_btree = bench(200, || {
        let clone = tree.clone();
        black_box(clone.len())
    });
    let clone_dense = bench(20_000, || {
        let clone = dense.clone();
        black_box(clone.len())
    });

    let lookup_btree_format = bench_lookup(2_000, &ops, |op| {
        let symbol = op.symbol();
        *tree.get(&symbol).unwrap_or(&0.0)
    });
    let lookup_dense_precomputed = bench_lookup(200_000, &op_ids, |id| dense[*id].unwrap_or(0.0));

    let update_btree = bench_update(400, || {
        let mut surface = tree.clone();
        let mut checksum = 0.0;
        for (index, symbol) in op_symbols.iter().enumerate() {
            let value = index as f64 * 0.25;
            surface.insert(symbol.clone(), value);
            checksum += value;
        }
        black_box(checksum)
    });
    let update_dense = bench_update(20_000, || {
        let mut surface = dense.clone();
        let mut checksum = 0.0;
        for (index, id) in op_ids.iter().copied().enumerate() {
            let value = index as f64 * 0.25;
            surface[id] = Some(value);
            checksum += value;
        }
        black_box(checksum)
    });

    println!("symbols={}", symbols.len());
    println!("lookup_ops={}", ops.len());
    println!("sorted_id_order_matches_string_sort={sorted_id_order_matches_string_sort}");
    print_metric("clone_btreemap_ns_per", clone_btree);
    print_metric("clone_dense_ns_per", clone_dense);
    print_ratio("clone_speedup", clone_btree, clone_dense);
    print_metric("lookup_btreemap_format_ns_per_op", lookup_btree_format);
    print_metric(
        "lookup_dense_precomputed_ns_per_op",
        lookup_dense_precomputed,
    );
    print_ratio(
        "lookup_speedup",
        lookup_btree_format,
        lookup_dense_precomputed,
    );
    print_metric("update_btreemap_clone_insert_ns_per_batch", update_btree);
    print_metric("update_dense_clone_set_ns_per_batch", update_dense);
    print_ratio("update_batch_speedup", update_btree, update_dense);
}

fn build_symbols() -> Vec<String> {
    let mut symbols = Vec::new();

    for index in 1..=220 {
        symbols.push(format!("static_symbol_{index:04}"));
    }

    for index in 1..=1_500 {
        symbols.push(format!("timem_{index:04}"));
        symbols.push(format!("intsty_{index:04}"));
    }

    for layer in 1..=24 {
        for root in [
            "wb18_perc_theta",
            "wb18_perc_fc",
            "wb18_perc_ul",
            "wb18_perc_ssc",
            "wb18_perc_frozen_depth",
            "wb19_dg",
            "wb19_thetfc",
            "wb19_thetdr",
            "wb19_por",
            "wb19_solthk",
        ] {
            symbols.push(format!("{root}_{layer:04}"));
        }
    }

    for layer in 1..=24 {
        for fine in 1..=10 {
            for root in [
                "frost_fine_depth",
                "frost_fine_theta",
                "frost_fine_temperature",
            ] {
                symbols.push(format!("{root}_{layer:04}_{fine:04}"));
            }
        }
    }

    for slot in 1..=40 {
        for root in [
            "ofe_index",
            "year_in_rotation",
            "rotation_index",
            "crop_slots",
            "slot_enabled",
        ] {
            symbols.push(format!("pl_schedule_slot_{slot:04}_{root}"));
        }
        for crop in 1..=3 {
            for root in [
                "imngmt", "jdplt", "jdharv", "jdstop", "mgtopt", "ncut", "ncycle", "frmove",
            ] {
                symbols.push(format!("pl_growth_slot_{slot:04}_crop_{crop:04}_{root}"));
                symbols.push(format!("pl_decomp_slot_{slot:04}_crop_{crop:04}_{root}"));
            }
        }
    }

    for hour in 1..=24 {
        for root in [
            "mofe_current_saturation_runoff",
            "mofe_current_lateral_runoff",
            "mofe_upstream_saturation_runoff",
            "mofe_upstream_lateral_runoff",
        ] {
            symbols.push(format!("{root}_{hour:04}"));
        }
    }

    symbols
}

fn build_lookup_ops() -> Vec<LookupOp> {
    let mut ops = Vec::new();

    for index in 1..=180 {
        ops.push(LookupOp::Climate {
            series: "timem",
            index,
        });
        ops.push(LookupOp::Climate {
            series: "intsty",
            index,
        });
    }

    for layer in 1..=24 {
        for root in [
            "wb18_perc_theta",
            "wb18_perc_fc",
            "wb18_perc_ul",
            "wb19_dg",
            "wb19_thetfc",
            "wb19_thetdr",
        ] {
            ops.push(LookupOp::WbLayer { root, layer });
        }
    }

    for layer in 1..=24 {
        for fine in 1..=10 {
            for root in [
                "frost_fine_depth",
                "frost_fine_theta",
                "frost_fine_temperature",
            ] {
                ops.push(LookupOp::FrostFine { root, layer, fine });
            }
        }
    }

    for slot in 1..=40 {
        for crop in 1..=3 {
            for (family, root) in [
                ("pl_growth", "imngmt"),
                ("pl_growth", "jdplt"),
                ("pl_growth", "jdharv"),
                ("pl_decomp", "mgtopt"),
                ("pl_decomp", "ncut"),
                ("pl_decomp", "frmove"),
            ] {
                ops.push(LookupOp::PlCrop {
                    family,
                    root,
                    slot,
                    crop,
                });
            }
        }
    }

    ops
}

impl LookupOp {
    fn symbol(self) -> String {
        match self {
            Self::Climate { series, index } => format!("{series}_{index:04}"),
            Self::WbLayer { root, layer } => format!("{root}_{layer:04}"),
            Self::FrostFine { root, layer, fine } => {
                format!("{root}_{layer:04}_{fine:04}")
            }
            Self::PlCrop {
                family,
                root,
                slot,
                crop,
            } => format!("{family}_slot_{slot:04}_crop_{crop:04}_{root}"),
        }
    }
}

fn bench<F>(repeats: usize, mut work: F) -> f64
where
    F: FnMut() -> usize,
{
    let start = Instant::now();
    let mut sink = 0usize;
    for _ in 0..repeats {
        sink ^= work();
    }
    black_box(sink);
    ns_per(start.elapsed(), repeats)
}

fn bench_lookup<T, F>(repeats: usize, ops: &[T], mut lookup: F) -> f64
where
    F: FnMut(&T) -> f64,
{
    let start = Instant::now();
    let mut sink = 0.0;
    for _ in 0..repeats {
        for op in ops {
            sink += lookup(op);
        }
    }
    black_box(sink);
    ns_per(start.elapsed(), repeats * ops.len())
}

fn bench_update<F>(repeats: usize, mut work: F) -> f64
where
    F: FnMut() -> f64,
{
    let start = Instant::now();
    let mut sink = 0.0;
    for _ in 0..repeats {
        sink += work();
    }
    black_box(sink);
    ns_per(start.elapsed(), repeats)
}

fn ns_per(duration: Duration, repeats: usize) -> f64 {
    duration.as_nanos() as f64 / repeats as f64
}

fn print_metric(name: &str, value: f64) {
    println!("{name}={value:.2}");
}

fn print_ratio(name: &str, numerator: f64, denominator: f64) {
    println!("{name}={:.2}x", numerator / denominator);
}

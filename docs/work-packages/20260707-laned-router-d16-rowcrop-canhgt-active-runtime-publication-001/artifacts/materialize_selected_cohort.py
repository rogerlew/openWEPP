#!/usr/bin/env python3
"""Materialize D16 selected active-suite run directories."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import shutil
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import pyarrow.parquet as pq

OPENWEPP_ROOT = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
RUNS_DIR = ARTIFACTS / "selected-cohort-runs"
WEPPPY_ROOT = Path("/home/workdir/wepppy")

sys.path.insert(0, str(WEPPPY_ROOT))

from wepppy.wepp.management import read_management  # noqa: E402


def load_route_coefficients_module() -> Any:
    path = WEPPPY_ROOT / "wepppy/nodb/mods/disturbed/route_coefficients.py"
    spec = importlib.util.spec_from_file_location("disturbed_route_coefficients", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


ROUTE_COEFFICIENTS = load_route_coefficients_module()


@dataclass(frozen=True)
class ExternalMember:
    member_id: str
    root: Path
    target_class: str


EXTERNAL_MEMBERS = (
    ExternalMember(
        member_id="mn_corn_h4",
        root=Path("/wc1/runs/al/algebraic-radium"),
        target_class="agriculture crops",
    ),
    ExternalMember(
        member_id="n_idaho_forest_h1",
        root=Path("/wc1/runs/un/unpalatable-rind"),
        target_class="forest",
    ),
    ExternalMember(
        member_id="wa_cascades_forest_h1",
        root=Path("/wc1/runs/ar/arboreal-dendrite"),
        target_class="forest",
    ),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as fp:
        for chunk in iter(lambda: fp.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def copy_file(src: Path, dst: Path) -> None:
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)


def read_landuse_rows(root: Path) -> list[dict[str, Any]]:
    table = pq.read_table(root / "landuse/landuse.parquet")
    return table.to_pylist()


def select_landuse_row(member: ExternalMember) -> dict[str, Any]:
    candidates = [
        row
        for row in read_landuse_rows(member.root)
        if row.get("_map") == "disturbed"
        and row.get("disturbed_class") == member.target_class
    ]
    if not candidates:
        raise RuntimeError(
            f"{member.root} has no disturbed landuse row for {member.target_class!r}"
        )
    return min(candidates, key=lambda row: int(row["wepp_id"]))


def write_runfile(
    run_dir: Path,
    run_name: str,
    stem: str,
    include_pmetpara: bool,
    wepp_ui: bool,
    output_subdir: str,
    runfile_name: str,
) -> None:
    lines = [
        'schema = "openwepp-hillslope-runfile-v1"',
        f'run_name = "{run_name}"',
        'unit_system = "metric"',
        "",
        "[inputs]",
        f'soil = "{stem}.sol"',
        f'management = "{stem}.man"',
        f'slope = "{stem}.slp"',
        f'climate = "{stem}.cli"',
        f"wepp_ui = {str(wepp_ui).lower()}",
    ]
    if include_pmetpara:
        lines.append('pmetpara = "pmetpara.txt"')
    lines.extend(
        [
            "",
            "[outputs]",
            f'pass = "{output_subdir}/H{stem[1:]}.hbp"',
            f'loss = "{output_subdir}/H{stem[1:]}.loss.json"',
            f'pass_parquet = "{output_subdir}/H{stem[1:]}.pass.parquet"',
            f'wat = "{output_subdir}/H{stem[1:]}.wat.parquet"',
            "",
        ]
    )
    (run_dir / runfile_name).write_text("\n".join(lines))


def write_mode_runfiles(
    run_dir: Path,
    run_name: str,
    stem: str,
    include_pmetpara: bool,
    wepp_ui: bool,
) -> dict[str, str]:
    plain = f"{stem}.plain.run.toml"
    hybrid = f"{stem}.hybrid.run.toml"
    write_runfile(
        run_dir,
        run_name,
        stem,
        include_pmetpara,
        wepp_ui,
        "output-plain",
        plain,
    )
    write_runfile(
        run_dir,
        run_name,
        stem,
        include_pmetpara,
        wepp_ui,
        "output-hybrid",
        hybrid,
    )
    return {"plain_run_file": plain, "hybrid_run_file": hybrid}


def materialize_h2637(manifest: list[dict[str, Any]]) -> None:
    src = (
        OPENWEPP_ROOT
        / "docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/h2637-prechange-active-plain"
    )
    dst = RUNS_DIR / "h2637"
    dst.mkdir(parents=True, exist_ok=True)
    for name in [
        "p2637.cli",
        "p2637.man",
        "p2637.slp",
        "p2637.sol",
        "pmetpara.txt",
        "snow.txt",
        "wepp_ui.txt",
    ]:
        copy_file(src / name, dst / name)
    mode_runfiles = write_mode_runfiles(
        run_dir=dst,
        run_name="h2637-laned-seam-active-suite",
        stem="p2637",
        include_pmetpara=True,
        wepp_ui=True,
    )
    manifest.append(
        {
            "member_id": "h2637",
            "source": str(src),
            "wepp_id": 2637,
            "disturbed_class": None,
            "route_coefficients": "pre-existing native H2637 active fixture",
            "run_dir": str(dst),
            **mode_runfiles,
            "input_sha256": {
                path.name: sha256(path)
                for path in sorted(dst.iterdir())
                if path.is_file()
            },
        }
    )


def materialize_external(member: ExternalMember, manifest: list[dict[str, Any]]) -> None:
    row = select_landuse_row(member)
    wepp_id = int(row["wepp_id"])
    source_runs = member.root / "wepp/runs"
    source_stem = f"p{wepp_id}"
    run_dir = RUNS_DIR / member.member_id
    run_dir.mkdir(parents=True, exist_ok=True)

    for suffix in ["cli", "slp", "sol"]:
        copy_file(source_runs / f"{source_stem}.{suffix}", run_dir / f"{source_stem}.{suffix}")

    include_pmetpara = (source_runs / "pmetpara.txt").is_file()
    include_snow = (source_runs / "snow.txt").is_file()
    if include_pmetpara:
        copy_file(source_runs / "pmetpara.txt", run_dir / "pmetpara.txt")
    if include_snow:
        copy_file(source_runs / "snow.txt", run_dir / "snow.txt")
    for optional in ["gwcoeff.txt", "wepp_ui.txt"]:
        if (source_runs / optional).is_file():
            copy_file(source_runs / optional, run_dir / optional)

    source_management = read_management(str(source_runs / f"{source_stem}.man"))
    coeff_row = ROUTE_COEFFICIENTS.route_coefficient_defaults_for_class(
        row["disturbed_class"]
    )
    coeff_row.update(
        {
            "disturbed_class": row["disturbed_class"],
            "luse": row["disturbed_class"],
            "stext": "texture_invariant",
        }
    )
    ROUTE_COEFFICIENTS.validate_route_coefficient_row(coeff_row)
    route_values = ROUTE_COEFFICIENTS.routing_coefficients_from_row(coeff_row)
    native_management = source_management.as_openwepp_native_cropland(route_values)
    (run_dir / f"{source_stem}.man").write_text(str(native_management))

    mode_runfiles = write_mode_runfiles(
        run_dir=run_dir,
        run_name=member.member_id,
        stem=source_stem,
        include_pmetpara=include_pmetpara,
        wepp_ui=(source_runs / "wepp_ui.txt").is_file(),
    )

    manifest.append(
        {
            "member_id": member.member_id,
            "source_root": str(member.root),
            "source_runs": str(source_runs),
            "wepp_id": wepp_id,
            "topaz_id": int(row["topaz_id"]),
            "disturbed_class": row["disturbed_class"],
            "landuse_key": int(row["key"]),
            "source_man_fn": row["man_fn"],
            "source_map": row["_map"],
            "route_coefficients": dict(
                zip(ROUTE_COEFFICIENTS.ROUTE_COEFFICIENT_COLUMNS, route_values)
            ),
            "route_coeff_source_ref": coeff_row["route_coeff_source_ref"],
            "route_coeff_authority_class": coeff_row["route_coeff_authority_class"],
            "route_coeff_confidence": coeff_row["route_coeff_confidence"],
            "route_coeff_notes": coeff_row["route_coeff_notes"],
            "run_dir": str(run_dir),
            **mode_runfiles,
            "input_sha256": {
                path.name: sha256(path)
                for path in sorted(run_dir.iterdir())
                if path.is_file()
            },
        }
    )


def main() -> None:
    if RUNS_DIR.exists():
        shutil.rmtree(RUNS_DIR)
    RUNS_DIR.mkdir(parents=True)

    manifest: list[dict[str, Any]] = []
    materialize_h2637(manifest)
    for member in EXTERNAL_MEMBERS:
        materialize_external(member, manifest)

    manifest_path = ARTIFACTS / "selected-cohort-materialization.json"
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"manifest": str(manifest_path), "members": len(manifest)}))


if __name__ == "__main__":
    main()

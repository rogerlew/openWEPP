#!/usr/bin/env python3
"""Independent binary64 oracle for the consumed-replacement witness.

This mirrors only the prescribed diagonal Stage 1 lambda-ball arithmetic.  It
does not call the Rust implementation or implement a production solver.
"""

import hashlib
import json
import math
import struct
from pathlib import Path

N = 21
SOURCE_ROOT = Path(
    "/home/roger/openwepp-experiments/cold-canopy-m1-controls17-review-20260924"
)
STAGE1 = SOURCE_ROOT / "crates/openwepp-land-surface-energy/src/m1_trust_region_stage1.rs"
CONTROL = SOURCE_ROOT / "crates/openwepp-land-surface-energy/src/m1_trust_region_stage2_controls.rs"
OUTPUT = Path(__file__).with_suffix(".json")


def bits(value: float) -> str:
    return f"0x{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def lambda_step(lam: float) -> list[float]:
    # Canonical diagonal fixture: sigma=0.25; U=I; y[0]=-2^-25,
    # y[3]=-0.5; all other y are +0.  Coordinates 5 and 11 are fixed at
    # their lower bounds, but their zero coefficients do not affect this
    # result.  Preserve the Rust operation grouping gain=σ/(σ²+λ), then
    # coefficient=-(gain*Uty).
    sigma = 0.25
    denom = sigma * sigma + lam
    gain = sigma / denom
    y = [0.0] * N
    y[0] = -2.0**-25
    y[3] = -0.5
    return [-(gain * value) for value in y]


def ordered_norm(step: list[float]) -> float:
    total = 0.0
    for value in step:
        total += value * value
    return math.sqrt(total)


def solve(radius: float) -> dict:
    zero = lambda_step(0.0)
    if ordered_norm(zero) <= radius:
        lo = hi = 0.0
        selected = zero
        expansions = 0
    else:
        lo = 0.0
        hi = 1.0
        selected = lambda_step(hi)
        expansions = 0
        while expansions < 48 and ordered_norm(selected) > radius:
            lo = hi
            hi *= 4.0
            selected = lambda_step(hi)
            expansions += 1
        if ordered_norm(selected) > radius:
            raise RuntimeError("canonical 48-expansion bracket was not found")
        for _ in range(48):
            mid = (lo + hi) * 0.5
            trial = lambda_step(mid)
            if ordered_norm(trial) <= radius:
                hi = mid
                selected = trial
            else:
                lo = mid

    scale = [1.0] * N
    scale[3] = 0.01
    scale[9] = 0.01
    scale[4] = 1000.0
    scale[10] = 1000.0
    scale[5] = 1.0e-5
    scale[11] = 1.0e-5
    scale[13] = 0.001
    physical = [scale[i] * selected[i] for i in range(N)]
    base = [300.0] * N
    base[3] = 0.01
    base[9] = 0.01
    base[4] = 1000.0
    base[10] = 1000.0
    base[5] = 0.0
    base[11] = 0.0
    base[13] = 0.01
    candidate = [base[i] + physical[i] for i in range(N)]
    return {
        "radius": radius.hex(),
        "radius_bits": bits(radius),
        "bracket_expansions": expansions,
        "final_lambda_interval": [lo.hex(), hi.hex()],
        "selected_upper_lambda_bits": bits(hi),
        "selected_norm": ordered_norm(selected).hex(),
        "selected_norm_bits": bits(ordered_norm(selected)),
        "scaled_step_hex": [value.hex() for value in selected],
        "scaled_step_bits": [bits(value) for value in selected],
        "physical_direction_s_times_p_hex": [value.hex() for value in physical],
        "physical_direction_s_times_p_bits": [bits(value) for value in physical],
        "candidate_coordinates_hex": [value.hex() for value in candidate],
        "candidate_coordinates_bits": [bits(value) for value in candidate],
    }


def main() -> None:
    records = [solve(4.0**-index) for index in range(11)]
    document = {
        "evidence_class": "independent static binary64 reconstruction",
        "frozen_tree_sha256": "ba8726d86c95bf56f044569a53224d5e7a94d2ad116aced84cac8e55ac0600ba",
        "source_provenance": {
            "stage1_path": str(STAGE1),
            "stage1_sha256": sha256(STAGE1),
            "stage1_operations": [
                "lambda_step: sigma/(sigma*sigma+lambda), then -(gain*uty)",
                "lambda_norm: ascending-coordinate square accumulation, then sqrt",
                "bracket: hi starts at 1 and is multiplied by 4 while norm>radius",
                "bisection: 48 midpoints; feasible midpoint replaces hi and selected step",
                "endpoint: final feasible upper endpoint",
            ],
            "fixture_path": str(CONTROL),
            "fixture_sha256": sha256(CONTROL),
            "fixture_model": "N=21; weighted A diagonal 0.25; weighted F[0]=-2^-25, F[3]=-0.5; other F=0; lower-active coordinates 5 and 11 have zero coefficients",
        },
        "records": records,
    }
    OUTPUT.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()

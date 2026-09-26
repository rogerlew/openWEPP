#!/usr/bin/env python3
"""Non-result-bearing primitive controls for the frozen guarded Dot2 path."""
from __future__ import annotations

import importlib.util
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("dot2_primitive_controls_target", HERE / "dot2-primitive.py")
assert spec is not None and spec.loader is not None
primitive = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = primitive
spec.loader.exec_module(primitive)


def limitation(label: str, x: list[float], y: list[float], expected: str) -> dict[str, str]:
    try:
        primitive.dot2(x, y)
    except primitive.PrototypeLimitation as error:
        message = str(error)
        assert expected in message, (label, message)
        return {"control": label, "message": message}
    raise AssertionError(f"{label}: expected PrototypeLimitation")


def main() -> None:
    positive = primitive.dot2([1.5, -2.0, 0.25], [2.0, 3.0, -4.0])
    primitive.verify_identities(positive)
    assert positive.counters.arithmetic == 25 * 3 - 7
    signed_zero = primitive.dot2([-0.0], [1.0])
    primitive.verify_identities(signed_zero)
    assert primitive.bits(signed_zero.trace.two_products[0][2]) == "8000000000000000"
    assert signed_zero.counters.signed_zero_values > 0
    controls = [
        limitation("nan_input", [math.nan], [1.0], "nonfinite or nonzero subnormal"),
        limitation("positive_infinity_input", [math.inf], [1.0], "nonfinite or nonzero subnormal"),
        limitation("negative_infinity_input", [-math.inf], [1.0], "nonfinite or nonzero subnormal"),
        limitation("nonzero_subnormal_input", [float.fromhex("0x0.0000000000001p-1022")], [1.0], "nonfinite or nonzero subnormal"),
        limitation("splitter_overflow", [sys.float_info.max], [1.0], "splitter overflow"),
        limitation("nonzero_product_underflow", [sys.float_info.min], [sys.float_info.min], "nonzero-product underflow"),
    ]
    output = {
        "evidence_class": "Ran: synthetic primitive admission/identity controls only; no retained capture loaded or candidate case evaluated",
        "positive_admitted_identity_controls": {
            "arithmetic": positive.counters.arithmetic,
            "guards": positive.counters.guards,
            "branches": positive.counters.branches,
            "exceptional_branches": positive.counters.exceptional_branches,
            "identity_checks": positive.counters.identity_checks,
        },
        "signed_zero": {
            "two_product_leading_bits": primitive.bits(signed_zero.trace.two_products[0][2]),
            "dot2_result_bits": primitive.bits(signed_zero.value),
            "recorded_signed_zero_values": signed_zero.counters.signed_zero_values,
            "interpretation": "The primitive retains raw binary64 signed-zero bits; it performs no normalization. The final Algorithm-5.3 addition's computed sign is separately recorded.",
        },
        "negative_controls": controls,
    }
    (HERE / "dot2-primitive-controls.json").write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()

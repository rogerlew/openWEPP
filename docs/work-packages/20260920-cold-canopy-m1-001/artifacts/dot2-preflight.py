#!/usr/bin/env python3
"""Non-measurement checks for the frozen DOT2 protocol and candidate source."""
from __future__ import annotations

import ast
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIMITIVE = HERE / "dot2-primitive.py"
PROTOCOL = HERE / "dot2-protocol.json"
CANDIDATE_FUNCTIONS = {"_mul", "_add", "_sub", "split", "two_sum", "two_product", "dot2"}
FORBIDDEN_CALLS = {"sum", "fsum", "fma", "Decimal", "Fraction"}


def call_name(node: ast.Call) -> str | None:
    if isinstance(node.func, ast.Name):
        return node.func.id
    if isinstance(node.func, ast.Attribute):
        return node.func.attr
    return None


def main() -> None:
    protocol = json.loads(PROTOCOL.read_text())
    assert protocol["candidate"] == "M1-DOT2-RESIDUAL-PROTOTYPE-01"
    assert protocol["status"] == "FROZEN_PRE_RUN_PENDING_INDEPENDENT_REVIEW"
    tree = ast.parse(PRIMITIVE.read_text(), filename=str(PRIMITIVE))
    functions = {node.name: node for node in tree.body if isinstance(node, ast.FunctionDef)}
    assert CANDIDATE_FUNCTIONS <= functions.keys()
    forbidden = []
    for name in CANDIDATE_FUNCTIONS:
        for node in ast.walk(functions[name]):
            if isinstance(node, ast.Call) and call_name(node) in FORBIDDEN_CALLS:
                forbidden.append((name, call_name(node)))
    assert not forbidden, forbidden
    # Algorithm 5.3 has initial TwoProduct, n-1 loop TwoProduct/TwoSum and
    # final p+s.  Keep its arithmetic count independently visible here.
    n = 22
    assert 17 + (n - 1) * (17 + 6 + 2) + 1 == 25 * n - 7 == 543
    print("PASS dot2 preflight: frozen protocol, candidate-path exclusions, 22-term ORO count")


if __name__ == "__main__":
    main()

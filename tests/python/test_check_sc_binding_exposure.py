from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "tools" / "check_sc_binding_exposure.py"


def contract_with_binding(core_id: str, indexed_id: str) -> str:
    return f"""# Contract

| Invariant or obligation | Statement |
|---|---|
| `{core_id}` | Core binding. |

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-X-001` | test addendum | `active` | `maps-to-existing-INV` | `{indexed_id}` | `none` | test |
"""


def run_checker(tmp_path: Path, core_id: str, indexed_id: str) -> subprocess.CompletedProcess[str]:
    contract = tmp_path / "SC-X-001.md"
    contract.write_text(contract_with_binding(core_id, indexed_id), encoding="utf-8")
    return subprocess.run(
        [sys.executable, str(CHECKER), str(contract)],
        check=False,
        capture_output=True,
        text=True,
    )


def test_accepts_role_qualified_obligation_id(tmp_path: Path) -> None:
    result = run_checker(tmp_path, "OBL-X-P-001", "OBL-X-P-001")
    assert result.returncode == 0, result.stdout + result.stderr


def test_rejects_absent_role_qualified_obligation_id(tmp_path: Path) -> None:
    result = run_checker(tmp_path, "OBL-X-P-001", "OBL-X-P-999")
    assert result.returncode == 1
    assert "not present in contract" in result.stdout


def test_rejects_absent_invariant_id(tmp_path: Path) -> None:
    result = run_checker(tmp_path, "INV-X-001", "INV-X-999")
    assert result.returncode == 1
    assert "not present in contract" in result.stdout

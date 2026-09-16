"""Collect explicitly named focused receipts without assigning package acceptance."""
import json,re,hashlib
from pathlib import Path
A=Path(__file__).resolve().parent
labels=["terminal-dto","terminal-phase-positive","terminal-phase-poisons","terminal-provider-poisons","terminal-manifest-poisons","terminal-beginning-poison","terminal-archive-controls","terminal-native-refusal","terminal-chronology-1","terminal-chronology-2","terminal-chronology-3"]
source=json.loads((A/"terminal-reconciliation.json").read_text())["tree_sha256"]
binary=json.loads((A/"terminal-focused-binary.json").read_text())
rows=[]
for label in labels:
    receipt=json.loads((A/(label+".json")).read_text())
    output=(A/(label+".stdout")).read_text()
    assert receipt["source_tree_sha256"]==source
    assert receipt["binary_sha256"]==binary["sha256"]
    assert all(receipt[key] is True for key in ["source_unchanged","binary_unchanged","inputs_unchanged","pinned_files_unchanged"])
    assert receipt["exit_code"]==0 and "test result: ok. 1 passed; 0 failed;" in output
    audit=[line for line in output.splitlines() if line.startswith("probe-")]
    rows.append(dict(label=label,receipt=label+".json",stdout=label+".stdout",elapsed_seconds=receipt["elapsed_seconds"],
      test_result="PASS 1/1",audit_lines=audit,stdout_sha256=hashlib.sha256(output.encode()).hexdigest()))
result=dict(source_tree_sha256=source,binary=binary,focused_tests=rows,passed_tests=len(rows),
 native_outcome="EXPECTED REFUSAL, not restored",native_six_owner_projection="NOT REACHED",
 physics="No new physical/model execution; audited file diagnostics report zero physical work and publication",
 critical_campaign="BLOCKED / NOT RUN under the owner run allowance",strict_clippy="Historical FAIL, not waived",
 scope="Fresh-process file/representation/chronology controls; no package or production acceptance.")
(A/"terminal-focused-results.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({key:result[key] for key in ["source_tree_sha256","passed_tests","native_outcome","native_six_owner_projection"]}))


"""Repeat the two explicitly selected owning-target diagnostic commands on terminal source."""
import json,subprocess
from pathlib import Path
A=Path(__file__).resolve().parent
python="/workdir/openWEPP/.venv/bin/python"
for package,prior,label in [
    ("orchestrator","lint-audited-orchestrator","terminal-lint-orchestrator"),
    ("runner","lint-final-runner","terminal-lint-runner")]:
    argv=json.loads((A/(prior+".json")).read_text())["argv"]
    result=subprocess.run([python,str(A/"run_recorded.py"),"--timeout","600",label,"--",*argv],check=False)
    print(label,result.returncode,flush=True)
    subprocess.run([python,str(A/"compare_reader_lint.py"),package,label],check=True)


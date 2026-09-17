"""Verify published scoped bytes at an explicit commit; no automatic retries."""
from concurrent.futures import ThreadPoolExecutor
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import urllib.request


commit, output = sys.argv[1:]
paths = subprocess.check_output(
    ["git", "diff-tree", "--no-commit-id", "--name-only", "-r", commit], text=True
).splitlines()


def check(path):
    expected = subprocess.check_output(["git", "show", commit + ":" + path])
    url = "https://raw.githubusercontent.com/rogerlew/openWEPP/" + commit + "/" + path
    result = {"path": path, "bytes": len(expected), "sha256": hashlib.sha256(expected).hexdigest()}
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            actual = response.read()
        result.update(remote_sha256=hashlib.sha256(actual).hexdigest(), equal=actual == expected)
    except Exception as error:
        result.update(equal=False, error=str(error))
    return result


with ThreadPoolExecutor(max_workers=8) as pool:
    entries = list(pool.map(check, paths))
receipt = {"commit": commit, "checked_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
           "method": "Exact-commit raw GitHub bytes versus git show commit:path; one request per file",
           "entries": entries, "all_equal": all(x["equal"] for x in entries),
           "availability": "Scoped published evidence only; full source bases, frozen binaries and corpus remain local."}
Path(output).write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps({"commit": commit, "files": len(entries), "all_equal": receipt["all_equal"]}))
raise SystemExit(0 if receipt["all_equal"] else 1)

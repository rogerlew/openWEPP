"""One bounded selective pass; ordered events retain member identity and exact numbers."""
import argparse
import datetime
import decimal
import hashlib
import json
from pathlib import Path
import resource
import time
import ijson

LIMIT = 1073741824
MAX_SELECTED_BYTES = 2 * 1024 * 1024

class HashReader:
    def __init__(self, source):
        self.source = source
        self.digest = hashlib.sha256()
        self.count = 0
        self.eof = False
    def read(self, size=-1):
        if size < 0:
            raise ValueError('unbounded read forbidden')
        data = self.source.read(size)
        self.digest.update(data)
        self.count += len(data)
        if size and not data:
            self.eof = True
        return data

def identity(stat):
    return {k: getattr(stat, k) for k in ('st_dev','st_ino','st_size','st_mtime_ns','st_ctime_ns')}

def select(source, destination, targets, expected_hash, expected_bytes, expected_rows):
    destination.mkdir(parents=True, exist_ok=False)
    summary = {'status':'INCOMPLETE', 'expected_source_sha256':expected_hash,
               'targets':targets, 'source':str(source), 'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
               'format':'ordered [physical_ordinal,event,value]; numbers encoded as exact decimal text; terminal COMPLETE required'}
    summary_path = destination / 'summary.json'
    summary_path.write_text(json.dumps(summary, indent=2)+'\n')
    start = time.monotonic()
    selected_bytes = 0
    seen = {}
    rows = 0
    try:
        before = identity(source.stat())
        with source.open('rb') as raw, (destination/'events.provisional.jsonl').open('w') as output:
            reader = HashReader(raw)
            depth = 0
            current = -1
            selected = False
            kinds = []
            top_key = None
            root_done = False
            root_key = None
            physical_arrays = 0
            in_physical = False
            for event, value in ijson.basic_parse(reader, use_float=False):
                if depth == 0:
                    if event != 'start_map' or root_done:
                        raise ValueError('expected one top-level object')
                    depth = 1
                    continue
                if depth == 1:
                    if event == 'end_map':
                        depth = 0
                        root_done = True
                        continue
                    if event == 'map_key':
                        root_key = value
                        continue
                    if root_key == 'physical':
                        physical_arrays += 1
                        if physical_arrays != 1:
                            raise ValueError('duplicate top-level physical member')
                        if event != 'start_array':
                            raise ValueError('top-level physical must be array')
                        in_physical = True
                    root_key = None
                    if event in ('start_map', 'start_array'):
                        depth += 1
                    continue
                if in_physical and depth == 2:
                    if event == 'end_array':
                        depth -= 1
                        in_physical = False
                        continue
                    if event != 'start_map':
                        raise ValueError('physical row must be object')
                    current = rows
                    selected = current in targets
                    kinds = []
                    top_key = None
                if selected:
                    if depth == 3 and event == 'map_key':
                        top_key = value
                    elif depth == 3 and event not in ('end_map', 'map_key'):
                        if top_key == 'kind':
                            kinds.append(value if event == 'string' else None)
                        top_key = None
                    encoded = str(value) if event == 'number' else value
                    line = json.dumps([current,event,encoded],ensure_ascii=True,separators=(',',':'))+'\n'
                    selected_bytes += len(line.encode())
                    if selected_bytes > MAX_SELECTED_BYTES:
                        raise ValueError('selected output exceeds declared 2MiB bound')
                    output.write(line)
                if event in ('start_map','start_array'):
                    depth += 1
                elif event in ('end_map','end_array'):
                    depth -= 1
                    if in_physical and depth == 2:
                        if selected:
                            if kinds != [targets[current]]:
                                raise ValueError('target kind missing, duplicate or mismatch')
                            seen[current] = kinds[0]
                        rows += 1
                        selected = False
            if not root_done or depth != 0 or not reader.eof:
                raise ValueError('incomplete root or EOF')
            if physical_arrays != 1:
                raise ValueError('missing top-level physical array')
            after_fd = identity(__import__('os').fstat(raw.fileno()))
        after = identity(source.stat())
        actual_hash = reader.digest.hexdigest()
        summary.update(source_stat_before=before,source_stat_after=after,source_fd_stat_after=after_fd,
                       actual_source_sha256=actual_hash,actual_bytes=reader.count,physical_rows=rows,selected=seen,
                       syntax_complete=True,eof=reader.eof,physical_arrays=physical_arrays)
        if before != after or before != after_fd:
            raise ValueError('source identity drift')
        if (actual_hash,reader.count,rows) != (expected_hash,expected_bytes,expected_rows):
            raise ValueError('source hash/byte/count mismatch')
        if set(seen) != set(targets):
            raise ValueError('target missing')
        summary['selected_event_sha256'] = hashlib.sha256((destination/'events.provisional.jsonl').read_bytes()).hexdigest()
        summary['status'] = 'COMPLETE'
    except Exception as error:
        summary['error'] = type(error).__name__+': '+str(error)
        raise
    finally:
        summary.update(elapsed_s=time.monotonic()-start,selected_event_bytes=selected_bytes,
                       peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                       rlimit_as=list(resource.getrlimit(resource.RLIMIT_AS)))
        summary_path.write_text(json.dumps(summary,indent=2)+'\n')
    return summary

def decode_selected(destination):
    """Decode only bounded selected events after successful source/EOF verification."""
    summary = json.loads((destination/'summary.json').read_text())
    if summary['status'] != 'COMPLETE':
        raise ValueError('incomplete extraction')
    path = destination/'events.provisional.jsonl'
    if path.stat().st_size > MAX_SELECTED_BYTES:
        raise ValueError('selected event file too large')
    if hashlib.sha256(path.read_bytes()).hexdigest() != summary['selected_event_sha256']:
        raise ValueError('selected event hash mismatch')
    result = {}
    stack = []
    keys = []
    with path.open() as events:
        for line in events:
            ordinal,event,value = json.loads(line)
            if event == 'map_key':
                if value in stack[-1]:
                    raise ValueError('duplicate selected member')
                keys[-1] = value
                continue
            if event in ('end_map','end_array'):
                stack.pop(); keys.pop()
                continue
            if event == 'number':
                value = decimal.Decimal(value) if any(c in value for c in '.eE') else int(value)
            elif event == 'start_map':
                value = {}
            elif event == 'start_array':
                value = []
            if not stack:
                result[ordinal] = value
            elif isinstance(stack[-1],list):
                stack[-1].append(value)
            else:
                stack[-1][keys[-1]] = value
            if event in ('start_map','start_array'):
                stack.append(value);keys.append(None)
    if stack:
        raise ValueError('truncated selected event log')
    return result

def typed_bytes(value):
    if not isinstance(value,dict) or list(value) != ['Ok'] or not isinstance(value['Ok'],list):
        raise ValueError('required typed payload is not Ok array')
    if any(type(v) is not int or not 0 <= v <= 255 for v in value['Ok']):
        raise ValueError('byte must be integer in 0..255')
    return bytes(value['Ok'])

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('source',type=Path)
    parser.add_argument('destination',type=Path)
    args = parser.parse_args()
    resource.setrlimit(resource.RLIMIT_AS,(LIMIT,LIMIT))
    print(json.dumps(select(args.source,args.destination,
        {123091:'surface_liquid_wb14_cadence_failure',123092:'surface_liquid_wb14_cadence_caller_failure'},
        '709dc4992b2fd08fd8bae755c730a5a01341a9a18cee8a0817ca76a249974e03',8236702644,123093)))

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

parser = argparse.ArgumentParser()
parser.add_argument('--source', type=Path)
args = parser.parse_args()
root = Path(__file__).resolve().parents[2]

def run(argv, cwd=root):
    return subprocess.check_output(argv, cwd=cwd)

metadata_argv = ['cargo', 'metadata', '--locked', '--offline', '--format-version=1']
metadata = json.loads(run(metadata_argv))
assert Path(metadata['workspace_root']) == root
packages = {p['id']: p for p in metadata['packages']}
nodes = {n['id']: n for n in metadata['resolve']['nodes']}
closures = {}
for name in ['lisp', 'shell-api', 'shell-client', 'shell-declaration']:
    package = next(p for p in packages.values() if p['name'] == name)
    seen = set()
    pending = [package['id']]
    while pending:
        item = pending.pop()
        if item in seen:
            continue
        seen.add(item)
        pending.extend(nodes[item]['dependencies'])
    names = sorted({packages[item]['name'] for item in seen})
    if name == 'lisp':
        assert not set(names).intersection(['env', 'system-api', 'terminal-api',
            'pipe-api', 'resource', 'execution', 'shell-api', 'shell-client',
            'shell-declaration', 'programs'])
    else:
        assert 'lisp' not in names
    closures[name] = names

archives = []
for profile in ['debug', 'release']:
    for name in ['lisp', 'shell_api', 'shell_client', 'shell_declaration']:
        path = root / f'target/riscv64gc-unknown-none-elf/{profile}/lib{name}.rlib'
        machines = []
        for member in run(['ar', 't', str(path)]).decode().splitlines():
            content = run(['ar', 'p', str(path), member])
            if content[:4] == b'\x7fELF':
                machines.append(int.from_bytes(content[18:20], 'little'))
        assert machines and set(machines) == {243}
        archives.append({'crate': name, 'profile': profile,
                         'elf_machine': 243, 'elf_objects': len(machines)})

source_files = []
history = []
lock_matches = None
if args.source:
    source = args.source.resolve()
    for original in sorted((source / 'crates/lisp').rglob('*')):
        if not original.is_file():
            continue
        relative = original.relative_to(source / 'crates/lisp')
        target = root / 'lisp' / relative
        assert original.read_bytes() == target.read_bytes()
        source_files.append({'path': str(relative),
            'sha256': hashlib.sha256(target.read_bytes()).hexdigest()})
    assert len(source_files) == 20
    mappings = json.loads((root / 'docs/lisp/commit-map.json').read_text())['commits']
    for mapping in mappings:
        fields = '--format=%an%n%ae%n%aI%n%cn%n%ce%n%cI%n%B'
        original = run(['git', 'show', '-s', fields, mapping['source']], source)
        candidate = run(['git', 'show', '-s', fields, mapping['candidate']])
        assert original == candidate
        history.append(mapping['candidate'])
    original_lock = tomllib.loads((source / 'Cargo.lock').read_text())['package']
    actual_lock = tomllib.loads((root / 'Cargo.lock').read_text())['package']
    original = {(p['name'], p['version'], p.get('source'), p.get('checksum'))
                for p in original_lock if p.get('source')}
    actual = {(p['name'], p['version'], p.get('source'), p.get('checksum'))
              for p in actual_lock if p.get('source')}
    assert actual <= original
    lock_matches = len(actual)

print(json.dumps({'metadata_command': metadata_argv,
    'workspace_packages': len(packages), 'dependency_closures': closures,
    'archives': archives, 'original_files': source_files,
    'source_metadata_equal_commits': history,
    'external_lock_entries_matching_source': lock_matches}, indent=2))

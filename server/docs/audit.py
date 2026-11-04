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
server = root / 'server'
provider = 'b72bbfca6455159a9bf9a0b250877b7e660e8c52'

def run(argv, cwd=root):
    return subprocess.check_output(argv, cwd=cwd)

command = ['cargo', 'metadata', '--manifest-path', 'server/Cargo.toml',
           '--locked', '--offline', '--format-version=1']
metadata = json.loads(run(command))
assert Path(metadata['workspace_root']) == server
packages = {p['id']: p for p in metadata['packages']}
nodes = {n['id']: n for n in metadata['resolve']['nodes']}
identities = {}
for name in ['env', 'system-api', 'pipe-api', 'shell-api', 'shell-declaration', 'lisp']:
    matching = [p for p in packages.values() if p['name'] == name]
    assert len(matching) == 1
    identities[name] = matching[0]['id']
lisp = packages[identities['lisp']]
assert lisp['source'] is None
assert Path(lisp['manifest_path']) == root / 'lisp/Cargo.toml'
assert not any(p['name'] == 'programs' or 'sqware' in (p['source'] or '')
               for p in packages.values())
seen = set()
pending = [lisp['id']]
while pending:
    item = pending.pop()
    if item in seen:
        continue
    seen.add(item)
    pending.extend(nodes[item]['dependencies'])
lisp_closure = sorted({packages[item]['name'] for item in seen})
assert not set(lisp_closure).intersection(['env', 'system-api', 'pipe-api',
    'resource', 'execution', 'terminal-api', 'shell-api', 'shell-server'])
assert not run(['git', 'diff', provider, '--', 'Cargo.toml', 'Cargo.lock',
               'lisp', 'api', 'client', 'declaration', 'rust-toolchain.toml', 'commands'])

archives = []
for profile in ['debug', 'release']:
    path = server / f'target/riscv64gc-unknown-none-elf/{profile}/libshell_server.rlib'
    machines = []
    for member in run(['ar', 't', str(path)]).decode().splitlines():
        content = run(['ar', 'p', str(path), member])
        if content[:4] == b'\x7fELF':
            machines.append(int.from_bytes(content[18:20], 'little'))
    assert machines and set(machines) == {243}
    symbol_result = subprocess.run(['nm', '-C', '--defined-only', str(path)],
        cwd=root, capture_output=True, text=True, check=True)
    symbols = symbol_result.stdout
    actual = [line for line in symbols.splitlines()
              if line.strip().endswith((' T shell_server::run', ' T shell_server::repl::run'))]
    assert len(actual) == 2
    archives.append({'profile': profile, 'machine': 243,
                     'objects': len(machines), 'run_symbols': actual,
                     'nm_metadata_diagnostics': symbol_result.stderr.splitlines()})

original_files = []
history = []
external_entries = None
source_revision = None
if args.source:
    source = args.source.resolve()
    source_revision = run(['git', 'rev-parse', 'HEAD'], source).decode().strip()
    assert source_revision == '94e114614bec53424ac55e640fc80cf19a3e7107'
    mappings = [('programs/src/user/shell', 'server/src'),
                ('programs/tests/shell', 'server/tests/shell'),
                ('programs/tests/shell-adapter', 'server/tests/shell-adapter')]
    for oldroot, newroot in mappings:
        for original in sorted((source / oldroot).rglob('*')):
            if not original.is_file():
                continue
            relative = original.relative_to(source / oldroot)
            if oldroot == 'programs/src/user/shell' and (
                    relative.parts[0] in ['api', 'client', 'declaration'] or str(relative) == 'main.rs'):
                continue
            target = root / newroot / relative
            if oldroot == 'programs/src/user/shell' and str(relative) == 'mod.rs':
                target = server / 'src/lib.rs'
            actual = target.read_bytes()
            original_bytes = original.read_bytes()
            normalized = actual
            category = 'unchanged'
            if target == server / 'src/lib.rs':
                prefix = b'#![no_std]\nextern crate alloc;\n'
                assert normalized.startswith(prefix)
                normalized = normalized[len(prefix):]
                category = 'crate headers only'
            elif original.suffix == '.rs' and oldroot.startswith('programs/tests/'):
                normalized = normalized.replace(b'../../src/core/mod.rs', b'../../src/user/shell/core/mod.rs')
                normalized = normalized.replace(b'../../src/native/', b'../../src/user/shell/native/')
                normalized = normalized.replace(b'../../src/adapt/ports.rs', b'../../src/user/shell/adapt/ports.rs')
                normalized = normalized.replace(b'../../../src/adapt/controller.rs', b'../../../src/user/shell/adapt/controller.rs')
                if normalized != actual:
                    category = 'source path/include only'
            elif original.name == 'Cargo.toml':
                old = tomllib.loads(original_bytes.decode())
                new = tomllib.loads(actual.decode())
                old.pop('workspace')
                old['dependencies']['lisp'] = {'path': '../../../lisp'}
                old['dependencies']['shell-api'] = {'path': '../../../api'}
                assert old == new
                normalized = original_bytes
                category = 'workspace and sibling dependency paths only'
            assert normalized == original_bytes
            original_files.append({'source': str(original.relative_to(source)),
                'destination': str(target.relative_to(root)), 'change': category,
                'byte_equal': actual == original_bytes, 'normalized_equal': True,
                'sha256': hashlib.sha256(actual).hexdigest()})
    assert len(original_files) == 56
    for mapping in json.loads((server / 'docs/commit-map.json').read_text())['commits']:
        fields = '--format=%an%n%ae%n%aI%n%cn%n%ce%n%cI%n%B'
        assert run(['git', 'show', '-s', fields, mapping['source']], source) == run(
                   ['git', 'show', '-s', fields, mapping['candidate']])
        history.append(mapping['candidate'])
    original_lock = tomllib.loads((source / 'Cargo.lock').read_text())['package']
    actual_lock = tomllib.loads((server / 'Cargo.lock').read_text())['package']
    old = {(p['name'], p['version'], p.get('source'), p.get('checksum'))
           for p in original_lock if p.get('source')}
    new = {(p['name'], p['version'], p.get('source'), p.get('checksum'))
           for p in actual_lock if p.get('source')}
    assert new <= old
    external_entries = len(new)

print(json.dumps({'metadata_command': command, 'workspace_packages': len(packages),
    'unique_package_identities': identities, 'lisp_closure': lisp_closure,
    'provider_packages_and_lock_unchanged': True, 'archives': archives,
    'original_files': original_files, 'source_metadata_equal_commits': history,
    'external_lock_entries_matching_source': external_entries,
    'source_revision': source_revision}, indent=2))

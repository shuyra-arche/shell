"""Verify the entry boundary and real image-builder artifacts; never boots them."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import tomllib

BASE = '7a2f99e268784693975b5e244b28795be50c218c'
SOURCE = '94e114614bec53424ac55e640fc80cf19a3e7107'
ROOT = Path(__file__).resolve().parents[2]


def run(argv, cwd=ROOT):
    return subprocess.check_output(argv, cwd=cwd)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def elf(path):
    data = path.read_bytes()
    assert data[:7] == b'\x7fELF\x02\x01\x01'
    kind, machine, version, entry, offset = struct.unpack_from('<HHIQQ', data, 16)
    assert (kind, machine, version) == (2, 243, 1)
    size, count = struct.unpack_from('<HH', data, 54)
    assert size == 56
    loads = []
    for index in range(count):
        typ, flags, off, addr, physical, filesz, memsz, align = struct.unpack_from(
            '<IIQQQQQQ', data, offset + index * size)
        assert typ not in (2, 3), 'dynamic segment or interpreter'
        if typ == 0x6474e551:
            assert not flags & 1, 'executable stack'
        if typ != 1:
            continue
        assert filesz <= memsz and off + filesz <= len(data)
        assert align == 4096 and off % align == addr % align == 0
        assert physical == addr and flags & 4
        assert flags & 3 != 3, 'writable executable segment'
        loads.append({'address': addr, 'file_size': filesz, 'memory_size': memsz,
                      'flags': flags, 'sha256': digest(data[off:off + filesz])})
    loads.sort(key=lambda item: item['address'])
    assert loads and loads[0]['address'] == 0x10000
    assert all(a['address'] + a['memory_size'] <= b['address']
               for a, b in zip(loads, loads[1:]))
    assert any(p['flags'] & 1 and p['address'] <= entry < p['address'] + p['file_size']
               for p in loads)
    return {'bytes': len(data), 'sha256': digest(data), 'entry': entry, 'loads': loads}


def external(lock):
    return {(p['name'], p['version'], p['source'], p.get('checksum'))
            for p in lock['package'] if 'source' in p}


parser = argparse.ArgumentParser()
parser.add_argument('--source', type=Path, required=True)
parser.add_argument('--builder-log', type=Path, action='append', required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)
assert run(['git', 'rev-parse', 'HEAD'], args.source).decode().strip() == SOURCE

# Existing behavior, SDK identities, command bindings and original tests are frozen.
frozen = ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'lisp', 'api', 'client',
          'server/src', 'server/tests', 'declaration/src/program.rs',
          'declaration/src/commands.rs', 'declaration/tests/commands.rs',
          'api/tests', 'client/tests', 'commands', 'cat']
assert not run(['git', 'diff', BASE, '--', *frozen])
original = run(['git', 'show', SOURCE + ':programs/src/user/shell/main.rs'], args.source)
expected = original.replace(b'extern crate programs;', b'extern crate shell_server;')
expected = expected.replace(b'#[programs::entry]', b'#[mold::entry]')
expected = expected.replace(b'programs::user::shell::run()', b'shell_server::run()')
expected = expected.replace(b'programs::debug::put', b'system_support::debug::put')
assert (ROOT / 'server/entry/src/main.rs').read_bytes() == expected
aliases = run(['git', 'show', SOURCE + ':programs/src/lib.rs'], args.source)
assert b'pub use mold::entry;' in aliases
assert b'pub use system_support::{boot, debug, support};' in aliases
metadata_format = '%an%x00%ae%x00%aI%x00%cn%x00%ce%x00%cI%x00%B'
assert run(['git', 'show', '-s', '--format=' + metadata_format,
            '35b3d73e13f695c935665efa6d79b12265af98e7'], args.source) == run([
                'git', 'show', '-s', '--format=' + metadata_format,
                '57be49f24cbbcf9bfbd3c675d0b1c3fc553367a9'])
base_lib = run(['git', 'show', BASE + ':declaration/src/lib.rs'])
assert base_lib.count(b'pub mod commands;') == 1
assert base_lib.count(b'pub use commands::') == 1
assert base_lib.replace(
    b'pub mod commands;', b'pub mod build;\npub mod commands;').replace(
    b'pub use commands::', b'pub use build::BUILD;\npub use commands::') == (
        ROOT / 'declaration/src/lib.rs').read_bytes()
config = tomllib.loads((ROOT / '.cargo/config.toml').read_text())
assert config == {'target': {'riscv64gc-unknown-none-elf': {'rustflags': [
    '-C', 'relocation-model=static', '-C', 'force-frame-pointers=yes',
    '-C', 'code-model=medium']}}}

metadata = json.loads(run(['cargo', 'metadata', '--manifest-path', 'server/Cargo.toml',
                          '--locked', '--offline', '--format-version=1']))
packages = {p['id']: p for p in metadata['packages']}
nodes = {n['id']: n for n in metadata['resolve']['nodes']}
assert len(packages) == len({p['name'] for p in packages.values()}) == 43


def closure(name):
    matches = [p for p in packages.values() if p['name'] == name]
    assert len(matches) == 1
    seen, pending = set(), [matches[0]['id']]
    while pending:
        item = pending.pop()
        if item not in seen:
            seen.add(item)
            pending.extend(nodes[item]['dependencies'])
    return {packages[item]['name'] for item in seen}


for name in ['env', 'system-api', 'pipe-api', 'execution', 'shell-api',
             'shell-declaration', 'lisp']:
    assert len([p for p in packages.values() if p['name'] == name]) == 1
assert not any(p['name'] == 'programs' or 'sqware' in (p['source'] or '')
               for p in packages.values())
entry, server, lisp = map(closure, ['shell-entry', 'shell-server', 'lisp'])
assert len(entry) == 41 and len(server) == 37 and len(lisp) == 13
assert entry - server == {'shell-entry', 'system-support', 'hub-api', 'fdt'}
assert not lisp.intersection({'env', 'execution', 'shell-server', 'system-api'})
old = tomllib.loads(run(['git', 'show', BASE + ':server/Cargo.lock']).decode())
new = tomllib.loads((ROOT / 'server/Cargo.lock').read_text())
source_lock = tomllib.loads((args.source / 'Cargo.lock').read_text())
assert external(old) <= external(new) <= external(source_lock)
old_manifest = tomllib.loads(run(['git', 'show', BASE + ':server/Cargo.toml']).decode())
new_manifest = tomllib.loads((ROOT / 'server/Cargo.toml').read_text())
assert old_manifest['dependencies'] == new_manifest['dependencies']

symbols = ['_start', 'clean_ret', '__rustc::__rust_alloc', '__rustc::__rust_dealloc',
           '__rustc::__rust_realloc', '__rustc::__rust_alloc_zeroed',
           '__rustc::__rust_alloc_error_handler', '__rustc::rust_begin_unwind',
           'execution::memory::allocator::HEAP']
artifacts = []
for log in args.builder_log:
    report = json.loads(log.read_text().splitlines()[-1])
    assert report['image'] == 'shell' and report['manifest'] == 'server/entry/Cargo.toml'
    assert report['package'] == 'shell-entry' and report['bin'] == 'prog-shell'
    assert report['target'] == 'riscv64gc-unknown-none-elf'
    assert report['declared_profile'] == 'release'
    assert report['features'] == [] and report['default_features'] is True
    path = Path(report['executable'])
    raw = elf(path)
    defined = {}
    for line in run(['nm', '-C', '--defined-only', str(path)]).decode().splitlines():
        fields = line.split(maxsplit=2)
        if len(fields) == 3 and fields[2] in symbols:
            defined.setdefault(fields[2], []).append(int(fields[0], 16))
    assert set(defined) == set(symbols) and all(len(v) == 1 for v in defined.values())
    assert raw['entry'] == defined['_start'][0]
    assert not run(['nm', '-u', str(path)]).strip()
    stripped_path = args.output / (report['built_profile'] + '-prog-shell.elf')
    subprocess.run(['llvm-objcopy', '--strip-all', str(path), str(stripped_path)], check=True)
    stripped = elf(stripped_path)
    assert raw['entry'] == stripped['entry'] and raw['loads'] == stripped['loads']
    assert stripped['bytes'] <= report['max_image'] == 16 * 1024 * 1024
    assert gzip.decompress(gzip.compress(stripped_path.read_bytes(), mtime=0)) == stripped_path.read_bytes()
    artifacts.append({'profile': report['built_profile'], 'raw': raw, 'stripped': stripped,
                      'symbols': {k: v[0] for k, v in defined.items()}})
assert {p['profile'] for p in artifacts} == {'dev', 'release'}
print(json.dumps({'packages': len(packages), 'server_closure': len(server),
                  'entry_closure': len(entry), 'lisp_closure': len(lisp),
                  'external_lock_entries': len(external(new)),
                  'source_entry_sha256': digest(original), 'artifacts': artifacts}, indent=2))

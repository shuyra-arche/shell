"""Audit consolidation provenance, Cargo identities, and actual seven-image ELFs."""
import argparse
import ast
import gzip
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]
ENTRY = '1165fa40a2ed3f32cbd6bc1966bb70041cdcc600'
COMMANDS = 'effeb09c170f0259c3d79687754b02aa41f615fe'
CAT = 'df651c37d99f15e26cf7d76a24f0a63a4b58df58'
BASE = 'f4ab6a2dc3924959ac5096305b582cfb479569bd'
SOURCE = '94e114614bec53424ac55e640fc80cf19a3e7107'
NAMES = ['shell', 'cat', 'emit', 'upper', 'fail', 'spin', 'workers']


def run(argv, cwd=ROOT):
    return subprocess.check_output(argv, cwd=cwd)


def git(*argv):
    return run(['git', *argv])


def original(revision, path):
    return git('show', revision + ':' + path)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def external(lock):
    return {(p['name'], p['version'], p['source'], p.get('checksum'))
            for p in lock['package'] if 'source' in p
            and not p['source'].startswith('git+https://github.com/shuyra-arche/shell.git')}


def elf(path):
    data = path.read_bytes()
    assert data[:7] == b'\x7fELF\x02\x01\x01'
    kind, machine, version, entry, phoff = struct.unpack_from('<HHIQQ', data, 16)
    assert (kind, machine, version) == (2, 243, 1)
    phsize, count = struct.unpack_from('<HH', data, 54)
    assert phsize == 56
    loads = []
    for index in range(count):
        typ, flags, off, addr, physical, filesz, memsz, align = struct.unpack_from(
            '<IIQQQQQQ', data, phoff + index * phsize)
        assert typ not in (2, 3), 'dynamic segment or interpreter'
        if typ == 0x6474e551:
            assert not flags & 1, 'executable stack'
        if typ != 1:
            continue
        assert filesz <= memsz and off + filesz <= len(data)
        assert align == 4096 and off % align == addr % align == 0
        assert physical == addr and flags & 4 and flags & 3 != 3
        loads.append({'address': addr, 'file_size': filesz, 'memory_size': memsz,
                      'flags': flags, 'sha256': sha(data[off:off + filesz])})
    loads.sort(key=lambda item: item['address'])
    assert loads and loads[0]['address'] == 0x10000
    assert all(a['address'] + a['memory_size'] <= b['address']
               for a, b in zip(loads, loads[1:]))
    assert any(p['flags'] & 1 and p['address'] <= entry < p['address'] + p['file_size']
               for p in loads)
    return {'bytes': len(data), 'sha256': sha(data), 'entry': entry, 'loads': loads}


parser = argparse.ArgumentParser()
parser.add_argument('--source', type=Path, required=True)
parser.add_argument('--builder-log', type=Path, action='append', required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)
assert run(['git', 'rev-parse', 'HEAD'], args.source).decode().strip() == SOURCE
heads = [BASE, ENTRY, COMMANDS, CAT,
         '7a2f99e268784693975b5e244b28795be50c218c',
         'b72bbfca6455159a9bf9a0b250877b7e660e8c52']
for head in heads:
    subprocess.run(['git', 'merge-base', '--is-ancestor', head, 'HEAD'], cwd=ROOT, check=True)

# Every inherited file is frozen except the explicitly adapted SDK coordinates,
# their locks, the extraction-coordinate test and its historical README notes.
for revision, allowed in [
        (ENTRY, {'README.md'}),
        (COMMANDS, {'commands/Cargo.toml', 'commands/Cargo.lock',
                    'commands/tests/test_migration.py', 'commands/README.md'}),
        (CAT, {'cat/command/Cargo.toml', 'cat/Cargo.lock', 'cat/README.md'})]:
    for path in git('ls-tree', '-r', '--name-only', revision).decode().splitlines():
        if revision == COMMANDS and not path.startswith('commands/'):
            continue
        if revision == CAT and not path.startswith('cat/'):
            continue
        if path not in allowed:
            assert (ROOT / path).read_bytes() == original(revision, path), path
readme = original(ENTRY, 'README.md')
title = b'# Shell interfaces and Lisp library\n'
note = (b'\nThe consolidated provider also contains the server entry and all six command\n'
        b'images. Use [`shell-images-declaration`](images/README.md) for the seven-image\n'
        b'build inventory and the current SDK dependency contract. The extraction notes\n'
        b'below describe the individual interface, Lisp and server slices.\n')
assert readme.count(title) == 1
assert (ROOT / 'README.md').read_bytes() == readme.replace(title, title + note)

for revision, path, changes in [
        (COMMANDS, 'commands/Cargo.toml', {'shell-client': {'path': '../client'}}),
        (CAT, 'cat/command/Cargo.toml', {'shell-api': {'path': '../../api'},
                                      'shell-client': {'path': '../../client'}})]:
    old = tomllib.loads(original(revision, path).decode())
    new = tomllib.loads((ROOT / path).read_text())
    for name, coordinate in changes.items():
        assert old['dependencies'][name] == {
            'git': 'https://github.com/shuyra-arche/shell.git',
            'rev': 'b910582fc9dc0fd732f279d94f2cfca89f4c8c8f'}
        old['dependencies'][name] = coordinate
    assert old == new
historical = tomllib.loads((ROOT / 'commands/provenance/sdk-at-extraction.toml').read_text())
assert historical == tomllib.loads(original(COMMANDS, 'commands/Cargo.toml').decode())['dependencies']['shell-client']

test_path = 'commands/tests/test_migration.py'
old_test = original(COMMANDS, test_path).decode()
new_test = (ROOT / test_path).read_text()


def methods(text):
    cls = next(n for n in ast.parse(text).body if isinstance(n, ast.ClassDef))
    return {n.name: ast.get_source_segment(text, n) for n in cls.body
            if isinstance(n, ast.FunctionDef)}


old_methods, new_methods = methods(old_test), methods(new_test)
assert set(new_methods) - set(old_methods) == {'test_historical_sdk_pin_is_recorded'}
for name, body in old_methods.items():
    if name == 'test_runtime_manifest_coordinates_and_immutable_pins':
        anchor = ('self.assertEqual(\n            dependencies["shell-client"],\n'
                  '            {"git": SHELL_URL, "rev": SHELL_REVISION},\n        )')
        assert body.count(anchor) == 1
        body = body.replace(anchor, 'self.assertEqual(dependencies["shell-client"], {"path": "../client"})')
    assert new_methods[name] == body, name

source_lock = tomllib.loads((args.source / 'Cargo.lock').read_text())
graphs = []
for label, manifest, expected in [('commands', 'commands/Cargo.toml', 25),
                                  ('cat', 'cat/Cargo.toml', 27),
                                  ('server', 'server/Cargo.toml', 43),
                                  ('images', 'images/Cargo.toml', 17)]:
    data = tomllib.loads((ROOT / manifest).read_text())
    assert not {'patch', 'replace'}.intersection(data)
    metadata = json.loads(run(['cargo', 'metadata', '--manifest-path', manifest,
                              '--locked', '--offline', '--format-version=1']))
    packages = metadata['packages']
    assert len(packages) == len({p['name'] for p in packages}) == expected
    for package in packages:
        assert package['name'] != 'programs' and 'sqware' not in (package['source'] or '')
        if package['name'].startswith('shell-') or package['name'] == 'lisp':
            assert package['source'] is None, package['id']
            assert Path(package['manifest_path']).is_relative_to(ROOT)
    lock_path = Path(manifest).parent / 'Cargo.lock'
    lock = tomllib.loads((ROOT / lock_path).read_text())
    assert external(lock) <= external(source_lock)
    if label in ('commands', 'cat'):
        revision = COMMANDS if label == 'commands' else CAT
        old = tomllib.loads(original(revision, str(lock_path)).decode())
        assert external(old) == external(lock)
    if label == 'images':
        assert not {p['name'] for p in packages}.intersection({'execution', 'lisp', 'shell-server', 'shell-client'})
    graphs.append({'workspace': label, 'packages': expected,
                   'external_lock_entries': len(external(lock)),
                   'sdk': [p['id'] for p in packages if p['name'] in ('shell-api', 'shell-client')]})

symbols = ['_start', 'clean_ret', '__rustc::__rust_alloc', '__rustc::__rust_dealloc',
           '__rustc::__rust_realloc', '__rustc::__rust_alloc_zeroed',
           '__rustc::__rust_alloc_error_handler', '__rustc::rust_begin_unwind',
           'execution::memory::allocator::HEAP']
required_symbols = set(symbols) - {'__rustc::__rust_alloc_zeroed'}
artifacts = []
profiles = set()
for log in args.builder_log:
    report = json.loads(log.read_text().splitlines()[-1])
    profile = report['profile']
    assert profile not in profiles
    profiles.add(profile)
    assert report['max_image'] == 16 * 1024 * 1024
    assert [b['image'] for b in report['builds']] == NAMES
    assert all(b['profile'] == profile and b['target'] == 'riscv64gc-unknown-none-elf'
               and b['features'] == [] and b['default_features'] is True for b in report['builds'])
    actual = report['artifacts']
    assert len(actual) == 7 and sorted(a['image'] for a in actual) == sorted(NAMES)
    for item in actual:
        path = Path(item['executable'])
        raw = elf(path)
        defined = {}
        for line in run(['nm', '-C', '--defined-only', str(path)]).decode().splitlines():
            fields = line.split(maxsplit=2)
            if len(fields) == 3 and fields[2] in symbols:
                defined.setdefault(fields[2], []).append(int(fields[0], 16))
        # The unused alloc_zeroed compiler wrapper can be removed in release.
        # Allocator ownership, core entry/panic symbols and all present wrappers
        # must still be unique in every image.
        assert required_symbols <= set(defined), (profile, item['image'], set(defined))
        assert all(len(v) == 1 for v in defined.values())
        assert raw['entry'] == defined['_start'][0]
        assert not run(['nm', '-u', str(path)]).strip()
        slim_path = args.output / (profile + '-' + item['image'] + '.elf')
        subprocess.run(['llvm-objcopy', '--strip-all', str(path), str(slim_path)], check=True)
        slim = elf(slim_path)
        assert (raw['entry'], raw['loads']) == (slim['entry'], slim['loads'])
        assert slim['bytes'] <= report['max_image']
        assert gzip.decompress(gzip.compress(slim_path.read_bytes(), mtime=0)) == slim_path.read_bytes()
        artifacts.append({'profile': profile, 'image': item['image'], 'raw': raw,
                          'stripped': slim, 'runtime_symbols': {k: v[0] for k, v in defined.items()},
                          'omitted_optional_wrappers': sorted(set(symbols) - set(defined))})
assert profiles == {'dev', 'release'} and len(artifacts) == 14
print(json.dumps({'reviewed_heads_retained': heads, 'original_method_bodies_preserved': len(old_methods) - 1,
                  'explicit_sdk_coordinate_contract_updated': True, 'graphs': graphs,
                  'artifacts': artifacts, 'runtime_acceptance': False}, indent=2))

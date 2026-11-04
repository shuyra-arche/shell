# Source and verification scope

The original source is public `https://github.com/anreonyr/sqware.git`, fixed
at `94e114614bec53424ac55e640fc80cf19a3e7107`. The executor's source path is a
local checkout of that public revision, not a private source or substitute.

After building the target archives and host suites, the committed audit can
compare source files, locks and original commit metadata with a public Source
checkout:

```sh
git clone --no-checkout https://github.com/anreonyr/sqware.git ../source-baseline
git -C ../source-baseline fetch origin 94e114614bec53424ac55e640fc80cf19a3e7107
git -C ../source-baseline checkout --detach 94e114614bec53424ac55e640fc80cf19a3e7107
python3 server/docs/audit.py --source ../source-baseline
```

The 56 original-file comparisons, 33 baseline external lock entries and 22
original commit-metadata comparisons are executor-run checks against that
public Source revision. The source tree is a prerequisite of those comparisons.
Without `--source`, the audit checks the actual Cargo graph, unique package
identities, sibling Lisp path, provider-base preservation and built RISC-V
archives. Anonymous Shell checkout and external Git-consumer checks follow
upload; their completed results belong to the PR description.

`server/tests/shell/Cargo.lock` and `server/tests/shell-adapter/Cargo.lock` are
intentional byte-identical Source fixtures. Cargo ignores them as workspace
member locks. The effective runtime/test lock is `server/Cargo.lock`.

The import retains original authors, committers, dates, messages and filtered
source ancestry. The commit map records rewritten object IDs after prefixing
the tree into `server/`. The source baseline contained no project license;
none is invented by this extraction.

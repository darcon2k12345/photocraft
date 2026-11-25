# craft-update (vendored)

A copy of `crates/update` from https://github.com/storytold/craft-libs at commit
`683fe06031224725b913c8ad3697d8e8bfec6c90` (MIT OR Apache-2.0; see LICENSE-MIT, LICENSE-APACHE and NOTICE here). It replaces the git
copy through `[patch."https://github.com/storytold/craft-libs"]` in the root Cargo.toml, so the
`craft` crate's re-export (`craft::update`) uses it too. The manifest was made standalone
(workspace-inherited fields written out; the sibling craft crates it needs are git dependencies at the
same commit; the custom test harness and the dev-dependencies were left out, so this copy's own unit
tests do not run here). Two behavioural changes:

- `src/detect.rs`: a Windows Installer entry counts as ours when its publisher is `Patronus` (as well
  as the previous publisher, `Learning Machines LLC`, so installs made before the rename are still
  found and upgraded). Upstream only accepts `Learning Machines LLC`.
- `src/msi_migrate.rs`: the suite folder under Programs is `Patronus` rather than `ArtCraft`.

To move to a newer craft-libs revision, copy `crates/update` again and re-apply those changes.

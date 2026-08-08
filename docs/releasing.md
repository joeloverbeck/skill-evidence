# Releasing, and bringing consumers forward

Procedure, not law. The rules this procedure serves are in
[`principles/consumer-contract.md`](principles/consumer-contract.md); when the two disagree, the
principles win and this document is what gets fixed.

## What is actually at stake

Recorded evidence in the consuming repositories, at the time of writing:

| Consumer | Evidence stores | Recorded events | How it depends |
|---|---|---|---|
| `playbench` | 32 | 994 | crates.io / git tag; host command `playbench` |
| `mundifold` | 15 | 75 | **vendored fork** at `crates/skill-evidence` |
| `what-we-bring-home` | 3 | 4 | crates.io / git tag; host command `cargo run --locked -p developer-tools --` |

Over a thousand append-only events that no release can regenerate. That is the number to have in
mind when deciding whether a change is really additive.

## 1. Decide the version from the surfaces you touched

| You changed | Version consequence |
|---|---|
| Private helpers, tests, docs, fixtures (additively) | patch |
| Added a public API item, a subcommand, an optional flag, a new installed package | patch or minor; minor if a consumer would want to opt in deliberately |
| Removed or renamed anything public, changed a flag or exit code, retired or renamed an installed package | **minor while `0.x`, major after `1.0.0`** |
| Anything at all about the shape of a recorded event | Stop. Go to §2 before choosing a number. |

While the crate is `0.x`, Cargo treats the minor position as the breaking one: `0.1.0` ↔ `0.1.1`
compatible, `0.1.0` ↔ `0.2.0` not.

Do not cut `1.0.0` as a milestone. Cut it when the recorded-event shape has stopped moving.

## 2. If you touched the recorded-event shape

Every one of these must hold before the release is allowed to exist:

- [ ] Every new field is **optional**, with a defined meaning when absent.
- [ ] No existing field became required, changed type, or narrowed its accepted values.
- [ ] The reader still accepts every shape this crate has ever written — including the two
      decontamination event types, whose writer is retired and whose readers are not.
- [ ] `cargo test --all-targets --locked` passes, which includes replaying the frozen corpora in
      `fixtures/skill-evidence/*-v1/` and validating them against the published schemas.
- [ ] No fixture was edited to make any of the above pass.

If a change cannot satisfy these, it needs a new schema version whose reader handles both, not a
looser interpretation of "additive".

## 3. Pre-release checks

```console
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo package --list          # confirm the shipped file set is what you expect
cargo publish --dry-run
```

`cargo package --list` matters more than it looks: the four skill packages and two schemas under
`assets/` are contract, and a consumer that receives the library without them has machinery and
no operator for it.

## 4. Publish and tag

```console
cargo publish
git tag vX.Y.Z && git push --tags
```

crates.io versions are immutable. A bad release is yanked, never replaced — so the dry run is not
a formality.

## 5. Write the release note

It exists so a consumer can decide whether to upgrade and knows what to do afterward. It must
name, explicitly:

- **Breaking Rust API changes** — what a consumer's build will complain about.
- **Command-surface changes** — renamed flags, removed subcommands, changed exit-code meanings.
  These break operators and skill packages without breaking `cargo build`.
- **Installed-package changes** — additions, and above all **retirements or renames, with the
  exact directories each consumer must delete by hand**. See §7.
- **Schema changes** — and the reason they satisfy §2.

## 6. Bring a consumer forward

From the consumer's repository root, on a clean tree:

```console
# 1. Take the new version.
cargo update -p skill-evidence          # or edit the pinned version first

# 2. Confirm it still builds and the consumer's own suites pass.
cargo test --all-targets --locked

# 3. Dry-run the asset install. Without --force this REFUSES on any file that
#    differs, names every one of them, and writes nothing at all.
<host-command> skills evidence install --root .

# 4. If it refused, read the list. Files that differ because the crate changed
#    are the upgrade; files that differ because you edited them locally are not.
#    Resolve the second kind first, then:
<host-command> skills evidence install --root . --force

# 5. Review what actually landed.
git diff

# 6. Delete any directories the release note named as retired. Nothing upstream
#    can do this for you — see §7.

# 7. Re-derive each evidence store and confirm history is untouched.
git status --short reports/skill-evidence/    # events.jsonl must not appear
```

Step 3 is the useful one. Because `install` decides every write before the first byte lands and
refuses atomically, running it *without* `--force` is a free, side-effect-free preview of exactly
which files the upgrade will change.

Step 7 is the one that matters. A projection may legitimately change; `events.jsonl` may not.

Host commands: `playbench` for playbench, `cargo run --locked -p developer-tools --` for
what-we-bring-home.

## 7. The orphan problem

`skills evidence install` writes and refuses to clobber. **It has no uninstall and no prune.**

A skill package retired or renamed upstream stays in the consumer's `.claude/skills/` forever,
where agents will keep discovering and offering it, and nothing this crate ships can reach it.

This is not hypothetical. `legacy-skill-decontamination` was retired before `v0.1.0` was cut and
is still installed in playbench.

Until the installer can withdraw what it wrote, a retirement is a two-part release: the code
change here, and a line in the release note telling every consumer which directory to `rm -rf`.
Both parts, or the retirement is not done.

## 8. A consumer that stays behind

Being behind is not a defect. A consumer may pin an old version indefinitely and nothing about
that obliges anyone.

A consumer that *vendors* a copy is a different matter — it has left the contract entirely, gets
no compatibility guarantee, and a defect reported against its line numbers is a report about that
fork until it is reproduced against a published version. `mundifold` is currently in this state.

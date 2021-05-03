# Diff-report-rs

> ⚠️ Moved to https://github.com/fpoli/cargo-diff-tools ⚠️

Only report `cargo`/`clippy` warnings related to the lines of a `git diff`. Useful to hide `cargo` and `clippy` warnings that are likely not related to the changed introduced by a pull request.

Inspired by [`Patryk27/clippy-dirty`](https://github.com/Patryk27/clippy-dirty).

## Examples

Hide the `clippy` warnings that are not on to the lines mentioned in a `git diff $GITHUB_BASE_REF...$GITHUB_HEAD_REF $GITHUB_HEAD_REF` (useful in the GitHub actions triggered by a pull request):

```bash
cargo clippy --message-format=json-diagnostic-rendered-ansi \
    | diff-report-rs $GITHUB_BASE_REF...$GITHUB_HEAD_REF $GITHUB_HEAD_REF
```

Hide the `cargo` warnings that are not on to the lines mentioned in a `git diff origin/master HEAD`:

```bash
cargo check --message-format=json-diagnostic-rendered-ansi \
    | diff-report-rs origin/master HEAD
```

Filter by `git diff origin/master`:

```bash
... | diff-report-rs origin/master
```

Filter by `git diff` only:

```bash
... | diff-report-rs
```

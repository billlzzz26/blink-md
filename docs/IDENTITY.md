# blink-md Engineering Agent Guardrails

## Role
- I am the blink-md engineering agent for this repository.
- My job is to keep code, tests, docs, CI, and packaging aligned with the actual project state.

## Source of truth
- `Cargo.toml` is the version/build source of truth.
- `CHANGELOG.md` is the release history source of truth.
- `README.md` is the user-facing source of truth.
- `TODO.md` is the active work tracker.
- `docs/PLAN.md` is the roadmap and definition-of-done reference.
- GitHub Actions logs are the CI source of truth.

## Pre-work checklist
1. Check `git status --short --branch` and branch sync.
2. Check latest CI run before changing CI-sensitive files.
3. Read the relevant code, tests, and docs before editing.
4. If adding or changing behavior, write or update tests first.
5. Update README, TODO.md, CHANGELOG.md, and relevant docs in the same change.
6. Run `make ci` before reporting completion.

## Packaging and secret rules
- Never commit or package local agent data: `.gemini/`, `.learnings/`, `.qwen/`.
- Never commit or package secrets: `*.key`, `*.pem`, `*.secret`, `secrets/`, `.env*`.
- Never package internal conductor docs: `docs/mcp/conductor/*`.
- Run `python scripts/check-package-hygiene.py` before merge.

## CI rules
- CI failures are user-facing blockers because releases and installers depend on them.
- Do not mask CI failures with `|| echo ...`.
- Android cross build must stay green because release artifacts include mobile targets.
- Release publish must fail loudly if crates.io publish fails.

## Working style
- Prefer small, verifiable changes.
- Prefer fixing root causes over documenting around failures.
- Prefer Rustls/TLS choices that work in cross-platform CI unless there is a clear reason not to.
- Keep the repo clean: no stale TODO versions, no dead scripts, no local-agent state, no package pollution.

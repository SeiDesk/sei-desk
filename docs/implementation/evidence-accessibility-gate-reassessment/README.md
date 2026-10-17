# Accessibility reassessment and independent runner evidence

2026-10-15. Accessibility verdict: [BLOCKED](../ACCESSIBILITY-GATE-REASSESSMENT.md). P3 owned-probe observer cleanup: separately passed. No new accessibility runtime or human tests were performed.

- `recorded-accessibility-summary.json`: extracted historical Stage 2D run identities and counts (29/29 each). It is an inspection summary, not a new run. Separate manual attestation remains in evidence-stage2d unchanged.
- `preservation-baseline.json` / `baseline-provenance.txt`: preservation comparison inputs. Baseline collection occurred after the targeted runner edit because an initial inspection command failed PowerShell parsing before executing. Other pre-existing files were hashed unchanged; original runner hash comes from the preserved prior independent-review manifest. New helper/test files were excluded. No pre-change test or capture is fabricated.
- `runner-regression/`: first three-case development execution, actual Windows child process/event gates.
- `runner-regression-final/` and `runner-regression-final.txt`: final helper tested with Windows PowerShell 5.1; three cases passed, exit 0. Observer failure while child is waiting, normal release, and observer failure after child exit. Exact owned child absent after cleanup. No arbitrary process is killed.
- `powershell-parse.json`: syntax parsing of the three affected scripts, no errors.
- `stage3b-run/`: fresh established runner using the final helper, full workspace gates, 56 ordinary tests + 5 explicitly enabled later, and 50 enabled service tests including real Windows COM/owned-launch fixtures. Includes resource/stdout/stderr logs and non-admin token metadata.
- `stage3b-run-exit.txt`: fresh overall exit 0.
- `final-review.json`: preservation results, local report links, whitespace/diff, owned-process and script hash checks.

Changes: Test-Stage3B.ps1 now delegates process ownership to Invoke-OwnedProbe.ps1; Test-OwnedProbe.ps1 is the deterministic regression harness. No Rust, AccessKit/iced, lockfile, previous report/evidence or approved design modification. No commit or push. The historical shortcut sandbox failure is preserved elsewhere; this fresh runner used a filesystem sandbox exception, not Windows elevation.

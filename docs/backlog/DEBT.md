# Parked debt — haptics-sdk

Anything out of scope, a known follow-up, or deliberately not fixed. Never
delete a row — retire it by strike-through naming the PR/date that closed it.

| ID | What and where | Where parked | Repay trigger | Status |
|---|---|---|---|---|
| DEBT-2026-11-04-no-audio-fixtures | `haptic_audio_analysis` `audio_loading::tests::{load_test_audio_file,multichannel_behavior}` read `resources/audio`, which Meta's public repo does not track, so they fail with NotFound. `tests/run_all.sh` skips them by name. | `chore/claude-setup`, 2026-11-04 | Meta publishes the fixtures, or this repo adds its own under `resources/audio` | OPEN |

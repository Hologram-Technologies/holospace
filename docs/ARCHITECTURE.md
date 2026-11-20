# Holospace: architecture

2026-11-20. Sources read at: holospaces `fd744fd`, Hologram `12bc8cb`, PrismPM `d18ff06`, and the IDE and Node session documents.
Nothing here is built or proved. "Measured" means one run on the live site, desktop Chromium, pane in the background. "Read" means read in source, not run.

## Answer

Holospace is four parts.
A kernel: six small LexLean models that make every decision, each shipped as a Prism holo.
A shell: one page and one service worker that open any Prism `.holo`, verify it and mount its View.
A machine: Hologram's emulator and store, which run Linux, always resumed from a snapshot, never cold-booted.
An editor: Prism IDE, to which Holospace supplies providers. Holospace ships no workbench.

The new repository holds the kernel, the shell and the providers. Emulator and store changes land in Hologram.

## The eight hard truths, resolved

1. Speed. Measured: Platform Manager loads in 0.7 s (1.05 MB); workbench visible at about 2.3 s (3.8 MB, 175 requests); x86-64 shell prompt between 33 s and 58 s cold. Read: all three cores are plain interpreters with no block or decode cache; the only figure in either repo is a comment, "tens of MIPS". The guest panicked ("Attempted to kill init") on my first command, so instructions per second were not measured.
2. Serverless. With static hosting alone: the shell, every Prism app, a pre-built Linux image fetched by hash, files, editor, search, git against hosts that allow cross-origin requests. Not serverless: pulling an OCI image from a registry, and any guest TCP (`apt`, `npm`, `git clone` in the guest). Those need a Chrome extension with a browser flag, or a native node. WebRTC needs a second peer and manual signalling.
3. Verification boundary. Confirmed. `PP2011` holds: a verified holo is an import-free guest with an empty capability set and a generated View. Holospace cannot itself be one verified holo. Kernels are verified holos; the body is audited and tested.
4. Proof scope. Confirmed. Decisions are proved; the emulator is tested against the RISC-V suite and QEMU.
5. Two `.holo` formats. They are different. Prism writes holo v5: a UORC archive sealed by a SHA-256 chain. Hologram writes HOLO v4: sections with a BLAKE3 footer, able to carry Wasm, View and root file system layers. Prism's legacy v4 shares Hologram's frame. Decision: two existing loaders behind one "open by hash" call, `prism-code/browser` for Prism holos and `hologram-archive` for Hologram ones. No third loader.
6. Two hashes. Prism holo v5 identity is SHA-256 only. Hologram's store accepts `sha256:` as a second key axis. Decision: content is keyed by BLAKE3; a Prism holo's SHA-256 name is checked at import and kept as a field.
7. Any browser. Read: the existing peer uses no threads, no shared memory and no service worker, and is tested on headless Chromium only. Decision: stay single-threaded per worker so cross-origin isolation is never required, which keeps GitHub Pages and Safari possible. Support matrix below.
8. Authoring. `prism/spec.json` is removed at PrismPM head; the path is hand-written `.lex.tex`. The `prism` tools attached to this session still describe spec.json and are stale. Only rule apps build without Docker; the kernels here are stateful, so they build through `prism-code build --remote github`.

Two findings the prompt did not anticipate.
holospaces has already moved into Hologram (`spaces/holospaces`, `spaces/holospaces-browser`). The standalone repository is the stale copy.
Prism cannot mount a View in a browser today: "HOLOVIEW mounting needs an intent service worker and is not in this release". Running any Prism app in a tab therefore needs that service worker first. It is the first thing Holospace builds, and it belongs upstream in PrismPM as a feature request.

## Kernel

Each model is `step : State × Event → State × List Effect`, small and separately proved.

| Kernel | Decides |
|---|---|
| Lifecycle | provision, enter, suspend, resume, terminate |
| Isolation | which storage key a holospace may read or write |
| Admission | whether received bytes are accepted |
| Launch verdict | whether an app is shown as verified and allowed to run |
| Capability gate | what a guest or app may read, write or reach. Reused from Prism IDE, not rewritten |
| Provisioning identity | the identity of a holospace from its definition |

The guarantees are listed in `GUARANTEES.md` for approval before any proving.
Limit: Lean proves the model. It does not prove the Rust and JavaScript body obeys it. Each kernel gets a differential test against the body.

## Shell and Prism app runtime

One service worker serves hash-named assets, cached forever. A release is one root hash.
It also answers `/_hologram/intent`, which is what mounts a Prism View in a tab.
Opening an app: fetch by hash, check seal and member identities, replay the acceptance vectors, then mount. Vector replay is not automatic in `prism-code` today; the launch verdict kernel makes it mandatory.
Guests are import-free Core-Wasm, so they run on the browser's own WebAssembly engine, not on the `wasmi` interpreter the current browser crate uses.
Compatibility is shown by a suite over published apps, both v4 and v5.

## Machine

Never cold-boot. A base image is booted once in CI, and its snapshot is published as static content by hash. Opening Linux is then a resume.
RAM and disk are fetched on demand by hash, so a resume loads only the pages it touches.
First architecture: riscv64. It is the only core with snapshot and restore, and the simplest to interpret. Then aarch64. x86-64 last, after its open heap-corruption defect is found.
Consequence: the current default (x86-64) changes. Images must be riscv64 builds.
The emulator moves out of the editor's extension worker into its own worker.
Interpreter speed is a separate track: a decoded-block cache first, measured; translation to WebAssembly only if the numbers demand it.
Store: one worker owns OPFS through synchronous handles, with a persisted index and working pins. This fixes two confirmed defects in Hologram's synchronous store (index lost on reload; `pin` does nothing). Skipping the re-hash on local reads is holospaces ADR-019 policy, not a defect; received bytes are always re-hashed.
A browser implementation of Hologram's `Space` contract does not exist. Holospace writes it, once, in Hologram.

## Editor

Holospace supplies to Prism IDE: a file system provider over the store and 9p, a terminal, tasks, source control, search and port preview.
Deleted from the old web peer: the VS Code web 1.91.1 build script, the patched `workbench.html`, the fallback Monaco workspace, and the Node shim inside `holospace-fs` (see `NODE-DECISION.md`).
Dependency: Prism IDE is a spike and one kernel today. Gate 4 waits on it.

## Browser support matrix (targets; only Chromium desktop is measured)

| | Prism apps | Linux | Editor |
|---|---|---|---|
| Chromium desktop | yes | yes | yes |
| Firefox desktop | yes | yes, slower (unverified) | yes |
| Safari desktop | yes | unverified: OPFS worker handles and memory limits | unverified |
| Android Chromium | yes | small images only (unverified) | reduced: terminal and single file |
| iOS Safari | yes | unverified: tab memory limit | reduced |

## Latency budgets (targets, to be confirmed by measurement)

| Step | Budget | Baseline |
|---|---|---|
| Shell warm start | 300 ms | 0.7 s cold today |
| Prism app, warm, verified and interactive | 100 ms | PrismPM records 20 to 48 ms for a returning visit of its proof checker |
| Prism app, cold | 1 s | not measured |
| Kernel decision | 1 ms | Forge measured 0.003 to 0.2 ms |
| Linux prompt, resume, warm | 2 s | no resume exists on the default path |
| Linux prompt, first visit | 5 s plus download | 33 to 58 s cold boot today |

## Build order and gates

1. Shell and store. Gate: loads offline.
2. Prism app runtime with the intent service worker. Gate: compatibility suite passes.
3. Linux on riscv64 resumed from a published snapshot. Gate: prompt within budget.
4. Prism IDE attached. Gate: edit in the editor, read in the guest.
5. aarch64, x86-64, provisioning from a repository, network exits, peers, sign-in.

## Open

Repository name: proposed `holospace` under Hologram-Technologies. Not created.
Not measured: instructions per second, Firefox, Safari, any phone, memory, resume time.
The guest panic on the live site is unexplained.

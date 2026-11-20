# Holospace kernels: what is proved

## Current state: one LexLean app, published (2026-11-20)

The kernels are one hand-maintained LexLean app, started from the Prism SDK's `decision-app` starter (prism-code 0.10.1) and published from a Prism template instance: https://github.com/Hologram-Technologies/holospace, page https://hologram-technologies.github.io/holospace/. Everything below this section describes the earlier per-kernel builds and is kept as history.

One request is a kernel word followed by that kernel's fields; `Holospace::dispatch` routes on the word.

| Word | Module | Theorems |
|---|---|---|
| `verdict` | `HolospaceLaunchVerdict` | 5 |
| `admission` | `HolospaceAdmission` | 7 |
| `isolation` | `HolospaceIsolation` | 3 |
| `gate` | `HolospaceCapabilityGate` | 16 |
| `lifecycle` | `HolospaceLifecycle` | 12 |
| none (not routed) | `HolospaceIdentity` | 3 |

Proved: the template's gate runs `prismpm verify` on every commit, which is Lean checking all 46 theorems with each one's exact axiom set. The same 46 are also accepted by Lean's kernel as WebAssembly on a developer machine in about 20 s (`prism_prove`), with nothing sent anywhere.

Built: the holo at the page above. Its 48 acceptance vectors pass in the template's build, and again when the earlier shell opens the live bytes and asks its own launch verdict kernel (tested). Fingerprint `3caff77ede83bc3c3e5f680380d1f217e0903e51ac04959152f91965d6bbebf7`.

Registered claims: six, `HK-01` to `HK-06`, one per module. Each is every theorem of that module, pinned by `features/suites/holospace.pins.tsv`. One claim per kernel, not per theorem, is what keeps the test file under the template's size ceiling.

Changes from the earlier kernels:
- Isolation is part of the proved, built app; it no longer needs an unproved interpreter.
- Lifecycle is one step, not a stateful app: phase, whether a snapshot exists, and the action go in; the answer comes out. The body applies the change each answer means. The earlier theorem "a suspended holospace always has a snapshot" is restated as an invariant every step preserves (`lifecycleInv011`), and resume is proved never to proceed without a snapshot.
- The capability gate carries its full 16 theorems.
- Every definition carries its kernel's word as a prefix, because the SDK exports all modules into one Rust crate.

Limits:
- That the routing module sends each word to the right kernel is not a theorem. It is pinned by a test and exercised by the vectors.
- 48 vectors, not more: the template's LexLean refuses a deeper vector list. Each kernel has eight valid requests covering every answer, and one invalid.
- Provisioning identity is proved for a stand-in number encoding only, as before, and is not reachable through `dispatch`.
- The shell, host and their tests are not in the repository: a template instance refuses non-Prism code. They exist only in a local working copy.
- prism-code 0.10.1 could not build or publish this app itself; the template wiring (claims, scenarios, tests, page description) was written by a script, by hand.

2026-11-20. Each kernel is a Prism model under `kernels/<name>/`, rendered by `prism-code` 0.9.5 from `spec.json` and verified by `prism-code verify --lean --hosted --all` on `https://mcp.uor.foundation/` (profile `prism-app`).
"Proved" below means that run returned "verified" with an attestation. The attestations were not re-checked locally. No holo was built.

| Kernel | Theorems | Attestation |
|---|---|---|
| lifecycle | 16 | `31a2878d9304a5105d210cad18b96b2610b6298eb70b8505c63cf72b20101b5b` |
| launch-verdict | 5 | `9313dc50199f970abde87f68248cf3db3141a2dec9b110ac14606504087b2849` |
| admission | 6 | `742b43ed34b8d9335a6e1dd973cf12a18a9b96bb926d2bfd72d84f838b42ffd5` |
| isolation | 3 | `e8946c17c9d8ab37a9c31a8b9d79c5a885eff7998829307d78b9be8f9a045875` |
| capability-gate | 4 published, 16 checked | none; see below. App `sha256:b8f9b98cf57695f69819fb30822cd4a903f7a16be7666ea1bc917e92a7fe5671` |
| provisioning-identity | 3 | none returned; draft `prism-app/dac89afd2ce156c14d16cee710efdfec25e690a72642d8957c1269ed24d3124c` |

The last two rows were proved differently from the first four.
capability-gate: `prism_prove` checked the rules with the Lean kernel (4.32.1) in process, 16 of 16, axioms `Quot.sound`, `propext`. This checks the rules as the rule judge reads them, not the rendered LexLean model. 48 of 48 acceptance vectors pass through the judge (tested).
capability-gate, published: the template's gate refused the 16-guarantee build, because prism-code generates one Rust test per guarantee and the template lets non-Prism code only shrink. The published app therefore carries four guarantees, proved on the hosted verifier from the rendered model: never allow without a grant, never relayed without a grant, denied always answers deny, and a network request through an extension or a node never answers allow. The rules are unchanged. The other twelve statements hold for the same rules (checked in process, above) but are not theorems of the published holo.
Built by GitHub Actions in `Hologram-Technologies/holospace-prism`, replacing the launch verdict app there; gate, page and page checks pass at `a96fc13`. prism-code checked the live holo against `spec.json` at `d40e9e1`: `blake3:c2e9bee6bd638f9c1c5114a9cc1d5839534f115eb6ad2a8cd7851fa3e438feab`. Page: https://hologram-technologies.github.io/holospace-prism/.
Two tooling faults found: prism-code's generated tests exceed the template's ceiling above about six guarantees, and replacing an app leaves the previous app's ids, feature and spec behind, which fails the honesty gate until removed by hand.
The launch verdict no longer has a published holo; its kernel is in `kernels/launch-verdict/`.
provisioning-identity: hand-written LexLean in `kernels/provisioning-identity/src/`, verified through the LexLean gateway (task 223, state "verified"). The gateway returned no attestation id. It is a bare model file, not a full Prism project, and has no judge or vectors.

## The 22 guarantees against the published app

Status words. Proved: a theorem of the published app, checked by Lean in the template's gate. By construction: true of the model's shape, with no theorem. Tested: a property of the running code, covered by the shell's tests, which are not in this repository. Assumed: taken on trust.

| # | Guarantee | Status | Where |
|---|---|---|---|
| 1 | One state at a time | By construction | The request carries one `phase`, one of four values. |
| 2 | Only the four moves happen | Proved | `lifecycleInv001` to `003`: started, paused and resumed are answered only from the phase each needs. `lifecycleInv009`: terminate is always answered ended unless already terminated. That the body moves to the phase an answer names is tested. |
| 3 | Terminated is final | Proved | `lifecycleInv005`: every action in the terminated phase is refused. `lifecycleInv012`: the next phase after terminated is terminated. |
| 4 | An acknowledged suspend has a snapshot | Proved for the model's step | `lifecycleInv010`: whenever the answer is paused, the next state keeps a snapshot. `lifecycleInv011`: if "suspended implies a snapshot" holds before a step, it holds after, so it holds in every state reached from the start. That the body moves to the model's next state is tested. |
| 5 | Resume uses the most recent snapshot | Proved in part | `lifecycleInv004`: resume is never answered without a snapshot. That it is the most recent one is tested. |
| 6 | Same state and event, same result | By construction | The judge is a pure function. |
| 7 | Every durable key derives from the identity | Tested | A property of the body's key function. |
| 8 | Different identities never share a key | Assumed | Rests on the hash having no collisions. `encodingInjective` proves it for the model encoding only. |
| 9 | No read returns another holospace's state | Proved | `isolationInv001`, with identities modelled as numbers. `isolationInv003`: a holospace is never locked out of its own. |
| 10 | Accepted only if the recomputed hash matches | Proved | `admissionInv001`, `002`: a differing hash is never accepted or re-keyed. |
| 11 | A mismatch moves to the next source | Proved | `admissionInv007`: a mismatch with a source left always answers retry. `admissionInv005`: never retry with none left. |
| 12 | All sources failed is a failure | Proved | `admissionInv006`: a mismatch with no source left always answers reject. |
| 13 | SHA-256 only at import, stored under BLAKE3 | Proved | `admissionInv003`: a SHA-256 name is never answered accept. `admissionInv004`: never re-keyed at runtime. |
| 14 | Verified only if every check passes | Proved | `verdictInv001` to `004`, and `005`: a sound app is always verified. |
| 15 | Any failed or skipped check is unverified | Proved | The same four; blocked is the only other answer. |
| 16 | An unverified app does not run | Tested | A property of the shell; its tests alter real apps at many bytes and none runs. |
| 17 | Everything denied unless granted | Proved | `gateInv001`, `002`: without a grant the answer is never allow or relayed. It is ask or deny. |
| 18 | A deny wins over a grant | Proved | `gateInv003`. |
| 19 | Network only under an explicit grant | Proved | `gateInv004`, `005`. |
| 20 | A connection through an extension or node is labelled not serverless | Proved in part | `gateInv006`, `007`: such a request is never answered allow, so one that proceeds is answered relayed. That the shell shows the label is tested. |
| 21 | Same definition, same identity, on any device | Proved for the model | `sameGivesSame`, for a stand-in number encoding. The real byte encoding and "any device" are tested. |
| 22 | The architecture is part of the identity | Proved for the model | `archChangesIdentity`, `encodingInjective`, same stand-in encoding. The real encoding is tested. |

Count: 12 proved outright, 2 proved in part, 3 proved for the model (4, 21, 22), 2 by construction, 2 tested only, 1 assumed. Guarantees 7 and 16 cannot become theorems of this app: they are about what the body does. The lifecycle model defines the next phase and next snapshot of every step (`lifecycleNextPhase`, `lifecycleNextSnapshot`); the app still answers with one word, and the body is expected to move to exactly that next state.

## Limits

The provisioning identity model uses numbers, not bytes: the definition is one number standing in for the canonical bytes of everything except the architecture. The profile has no byte lists to state the real encoding over.
The in-process kernel check cannot settle a guarantee written with "is not"; two such guarantees in the capability gate were split into one per value, which says the same thing.
Local proving in the SDK image is switched off on this machine (`PRISM_DOCKER` is set to a disabled marker), so no rendered model was proved locally and no holo was built for these two.

Lean proved the models. A first body exists in `shell/` for three of them: launch verdict, admission and capability gate run there as rule-app records through one verified rule judge, and `shell/test/` checks the judge against each source spec on every request (24, 16 and 576) and the shell against the launch verdict and admission kernels (tested, not proved). All six kernels now have running code in `shell/host.mjs` (tested, not proved). Isolation and lifecycle have no built judge: they run from their proved specs through `shell/spec-kernel.mjs`, an interpreter that is itself unproved, each compared in the tests with a second, separately written reading of the spec (100 isolation requests; 21845 lifecycle sequences of up to 7 actions). Provisioning identity is computed in the body as the BLAKE3 of a canonical encoding and tested for guarantees 21 and 22; it calls no kernel. The capability gate is asked by the host on every storage access and connection, and the host is checked against it on all 576 combinations of facts. The host keeps state in memory only, and the page does not use it yet. Compatibility (tested, 2026-11-20): all 16 apps in the Prism registry index `a9061f48dabb` were fetched by SHA-256 name through the admission kernel and open verified in the shell, with the fingerprint and vector count the registry's indexer recorded (740 vectors), in Node, Chromium, Firefox and Playwright's WebKit; an altered copy of each is blocked. Every one is a stateless holo v4: no holo v5, no stateful app and no Hologram-format archive was available to test, so those are unverified. The slowest app took 1250 ms to open in Firefox, over the 1 s cold budget.
Each model decides from results the body supplies (a hash comparison, a check outcome). The body computing those results correctly is a matter for tests.
`prism-code` 0.9.5 could not prove an invariant comparing two state numbers: its generated proof for the start state is rejected by Lean ("omega could not prove the goal"). The lifecycle model was simplified to avoid it.
The generated models declare too few axioms for some theorems; each run reports one mismatch, and the observed set (`Classical.choice`, `Quot.sound`, `propext`) was recorded and the model re-rendered until the run passed.
The sources were sent to the hosted verifier, which stores them unlisted, not privately.

# Holospace kernels: what is proved

## Current state: one LexLean app, published (2026-11-20)

The kernels are one hand-maintained LexLean app, started from the Prism SDK's `decision-app` starter (prism-code 0.10.1) and published from a Prism template instance: https://github.com/Hologram-Technologies/holospace, page https://hologram-technologies.github.io/holospace/. Everything below this section describes the earlier per-kernel builds and is kept as history.

One request is a kernel word followed by that kernel's fields; `Holospace::dispatch` routes on the word.

| Word | Module | Theorems |
|---|---|---|
| `verdict` | `HolospaceLaunchVerdict` | 5 |
| `admission` | `HolospaceAdmission` | 6 |
| `isolation` | `HolospaceIsolation` | 3 |
| `gate` | `HolospaceCapabilityGate` | 16 |
| `lifecycle` | `HolospaceLifecycle` | 8 |
| none (not routed) | `HolospaceIdentity` | 3 |

Proved: the template's gate ran `prismpm verify` on the app at `79ed6a2` and passed, which is Lean checking all 41 theorems with each one's exact axiom set. The same 41 are also accepted by Lean's kernel as WebAssembly on a developer machine in about 20 s (`prism_prove`), with nothing sent anywhere.

Built: the holo at the page above. Its 48 acceptance vectors pass in the template's build, and again when the earlier shell opens the live bytes and asks its own launch verdict kernel (tested). Fingerprint `3caff77ede83bc3c3e5f680380d1f217e0903e51ac04959152f91965d6bbebf7`.

Registered claims: six, `HK-01` to `HK-06`, one per module. Each is every theorem of that module, pinned by `features/suites/holospace.pins.tsv`. One claim per kernel, not per theorem, is what keeps the test file under the template's size ceiling.

Changes from the earlier kernels:
- Isolation is part of the proved, built app; it no longer needs an unproved interpreter.
- Lifecycle is one step, not a stateful app: phase, whether a snapshot exists, and the action go in; the answer comes out. The body applies the change each answer means. The earlier theorem "a suspended holospace always has a snapshot" is gone; in its place resume is proved never to proceed without one.
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

## Against the 22 approved guarantees

| Lines | Status |
|---|---|
| 1 to 4 | Proved (lifecycle). |
| 5, resume uses the most recent snapshot | By construction: the model keeps one snapshot number. No theorem. |
| 6, same input gives same result | By construction: the model is a pure function. No theorem. |
| 7, every durable key derives from the identity | Not proved. A property of the body; to be tested. |
| 8, different identities never share a key | Not proved. Rests on the hash having no collisions, an assumption. |
| 9, no read returns another holospace's state | Proved (isolation), with identities modelled as numbers. |
| 10 to 13 | Proved (admission). |
| 14 to 16 | Proved (launch-verdict). |
| 17 | Proved (capability-gate): without a grant the answer is never allow or relayed. It is ask or deny; ask means nothing happens until the user decides. |
| 18 | Proved (capability-gate): a denied request always answers deny. |
| 19 | Proved (capability-gate): network never answers allow or relayed without a grant. |
| 20 | Proved (capability-gate): a network request through an extension or a node never answers allow. When granted it answers relayed (the labelled answer) by rule 8; that it does is tested, not proved. That the shell shows the label is the body's job. |
| 21 | Proved (provisioning-identity), but trivially: the encoding is a function, so equal inputs give equal outputs. "On any device" is a property of the body; to be tested. |
| 22 | Proved (provisioning-identity) for a stand-in encoding, `definition * 3 + arch`: changing the architecture changes the encoding, and two different (definition, architecture) pairs never share one. Not proved for the real canonical byte encoding, and the step from encoding to identity rests on the hash having no collisions. |

## Limits

The provisioning identity model uses numbers, not bytes: the definition is one number standing in for the canonical bytes of everything except the architecture. The profile has no byte lists to state the real encoding over.
The in-process kernel check cannot settle a guarantee written with "is not"; two such guarantees in the capability gate were split into one per value, which says the same thing.
Local proving in the SDK image is switched off on this machine (`PRISM_DOCKER` is set to a disabled marker), so no rendered model was proved locally and no holo was built for these two.

Lean proved the models. A first body exists in `shell/` for three of them: launch verdict, admission and capability gate run there as rule-app records through one verified rule judge, and `shell/test/` checks the judge against each source spec on every request (24, 16 and 576) and the shell against the launch verdict and admission kernels (tested, not proved). All six kernels now have running code in `shell/host.mjs` (tested, not proved). Isolation and lifecycle have no built judge: they run from their proved specs through `shell/spec-kernel.mjs`, an interpreter that is itself unproved, each compared in the tests with a second, separately written reading of the spec (100 isolation requests; 21845 lifecycle sequences of up to 7 actions). Provisioning identity is computed in the body as the BLAKE3 of a canonical encoding and tested for guarantees 21 and 22; it calls no kernel. The capability gate is asked by the host on every storage access and connection, and the host is checked against it on all 576 combinations of facts. The host keeps state in memory only, and the page does not use it yet. Compatibility (tested, 2026-11-20): all 16 apps in the Prism registry index `a9061f48dabb` were fetched by SHA-256 name through the admission kernel and open verified in the shell, with the fingerprint and vector count the registry's indexer recorded (740 vectors), in Node, Chromium, Firefox and Playwright's WebKit; an altered copy of each is blocked. Every one is a stateless holo v4: no holo v5, no stateful app and no Hologram-format archive was available to test, so those are unverified. The slowest app took 1250 ms to open in Firefox, over the 1 s cold budget.
Each model decides from results the body supplies (a hash comparison, a check outcome). The body computing those results correctly is a matter for tests.
`prism-code` 0.9.5 could not prove an invariant comparing two state numbers: its generated proof for the start state is rejected by Lean ("omega could not prove the goal"). The lifecycle model was simplified to avoid it.
The generated models declare too few axioms for some theorems; each run reports one mismatch, and the observed set (`Classical.choice`, `Quot.sound`, `propext`) was recorded and the model re-rendered until the run passed.
The sources were sent to the hosted verifier, which stores them unlisted, not privately.

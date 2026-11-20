# holospaces feature map
Commit fd744fd3931ca92b052885fbe53bfad94eb7884f (shallow clone, read-only study; nothing built or run). Paths are relative to the repo root; `core/` = `crates/holospaces/src/`, `web/` = `crates/holospaces-web/web/`, `websrc/` = `crates/holospaces-web/src/`.

Column notes. ADR: the ADR the catalog row cites; `~` means not cited in the row, nearest by topic (my inference). Offline: my inference from the code path, meaning "needs no network once the static site assets are loaded" (there is no service worker, so the site itself is not cached for offline reload). Status: checked against the directory listing of `vv/suites` and `vv/targets`, not the doc text.

| CC | ADR | User-visible behaviour | Main source files | Hologram primitive | Browser API | Offline | Status |
|----|-----|------------------------|-------------------|--------------------|-------------|---------|--------|
| CC-1 | ~002 | Content gets a correct hash address (kappa label) | core/lib.rs, core/identity.rs | substrate-core address/verify (KappaLabel) | none | yes | live: vv/suites/cc1-kappa-addressing.sh |
| CC-2 | ~003 | Browser .holo compute result equals native result | core/engine.rs, websrc/lib.rs (run_holo) | hologram-exec, hologram-archive | none (wasm) | yes | live: cc2-holo-engine.sh |
| CC-3 | ~003 | Peer storage passes the substrate conformance battery | core/peer.rs | KappaStore (substrate TCK) | none | yes | live: cc3-substrate-tck.sh |
| CC-4 | ~004 | devcontainer.json parsed, validated, same source gives same id | core/config.rs, core/realizations.rs | Realization (kappa of definition) | none | yes | live: cc4-devcontainer.sh |
| CC-5 | ~008 | Wasm code modules are validated before use | core/wasm.rs | none (wasmparser) | none | yes | live: cc5-wasm.sh |
| CC-6 | 008, 009 | Wasm module boots, suspends, resumes, migrates as container | core/surface.rs, core/boot.rs, crates/holospaces-emulator/src/lib.rs | ContainerRuntime (Wasmtime, wasmi) | none (wasm) | yes | live: cc6-execution-surface.sh |
| CC-7 | ~011 | Disk image stored as deduplicated content-addressed sectors | core/disk.rs | KappaStore, bare-hal BlockDevice | OPFS (browser backing, websrc/opfs_store.rs) | yes | live: cc7-kdisk.sh |
| CC-8 | ~008 | Imported code runs with capability-limited storage I/O | core/boot.rs, core/surface.rs | ContainerRuntime, capabilities, storage_put/get | none | yes | live: cc8-import-run.sh |
| CC-9 | ~009 | Real RISC-V Linux boots to userspace on the emulator | core/emulator.rs | none in product path (codemodule uses ContainerRuntime) | none | yes | live: cc9-emulator.sh |
| CC-10 | ~013 | OCI image and repo ingested, every blob digest verified | core/oci.rs, core/assembly.rs | KappaStore, verify_kappa_axis | none | yes (given bytes) | live: cc10-ingestion.sh |
| CC-11 | ~010 | Real terminal: raw keys, line editing, Ctrl-C | core/projection.rs, core/emulator.rs (console) | KappaStore (event channel) | none | yes | live: cc11-workspace.sh |
| CC-12 | ~010 | Manager console: sign in, launch from repo URL, lifecycle | core/manager.rs, core/identity.rs, web/index.html | KappaStore, Realization | BroadcastChannel, OPFS, fetch | partly (repo fetch needs network) | live: cc12-manager-console.sh |
| CC-13 | ~010 | Fallback workspace: Monaco editor plus xterm terminal | web/workspace.html, websrc/lib.rs (Workspace) | KappaStore | none (emulator on main thread) | yes | live: cc13-vscode-workspace.sh |
| CC-14 | 011 | Linux mounts real ext4 rootfs over virtio-blk | core/emulator.rs, core/assembly/ext4.rs, core/machine.rs | KappaStore (kappa-disk) | none | yes | live: cc14-virtio-block.sh |
| CC-15 | 011 | Editor and guest OS share one workspace folder (9p) | core/emulator.rs (9p fs), core/emulator/devbus.rs | none (in-RAM 9p fs) | none | yes | live: cc15-virtio-9p.sh |
| CC-16 | 014 | Guest reaches internet through userspace NAT and tunnel | core/emulator/net.rs, websrc/wsnet.rs, web/relay.mjs | none | WebSocket | no | live: cc16-network.sh |
| CC-17 | 012, 015 | Real VS Code web workbench bound to running holospace | web/build-workbench.mjs, web/builtin-extensions/holospace-fs/extension.js | none (workbench sha256-verified at build) | Worker (extension host), OPFS | yes after load (gallery needs network) | live: cc17-vscode-workbench.sh, cc17-workspace-fs.sh, cc17-workbench-fs.sh, cc17-workbench-holospace.sh |
| CC-18 | 012, 015, 020 | Hover, completion, definition from language server in guest | holospace-fs/extension.js (bridge client), core/emulator/net.rs | none | Worker | yes | live: cc18-lsp.sh |
| CC-19 | 015, 017, 020 | Install and run extensions from Open VSX | web/build-workbench.mjs (gallery config), web/extensions/holospace-demo | none | Worker, fetch | no (gallery) | live: cc19-extensions.sh |
| CC-20 | 013 | Provision from repo URL; pull OCI image; verify bytes | core/import.rs, core/oci.rs | net-http /cas gateway, get_with_fetch | none (native path) | no | live: cc20-import.sh |
| CC-21 | 016 | Guest server reachable as a forwarded port | core/emulator/net.rs, core/machine.rs (forward_port) | none | none natively; relay route in browser | partly | live: cc21-port-forward.sh |
| CC-22 | ~016 | devcontainer lifecycle commands run on create | core/boot.rs, core/assembly.rs | none | none | yes unless commands fetch | live: cc22-lifecycle.sh |
| CC-23 | ~016 | Settings, dotfiles, secrets follow the operator | core/personalization.rs | Realization, KappaStore, KappaSync (claimed) | none | yes | live: cc23-personalization.sh |
| CC-24 | 017 | Guest tools sign in with OAuth device flow | core/emulator/net.rs (network only) | none | WebSocket (egress) | no | live: cc24-auth.sh |
| CC-25 | 016 | Dev Container Features install before lifecycle commands | core/boot.rs, core/import.rs, core/config.rs | KappaStore (feature by kappa) | none | no (feature pull) | live: cc25-features.sh |
| CC-26 | ~016 | Dockerfile build executed inside the guest, no Docker | core/dockerfile.rs, core/boot.rs | none | none | no (FROM pull) | live: cc26-build.sh |
| CC-27 | ~016 | Compose file resolves the devcontainer service (parse only) | core/compose.rs | none | none | yes | live: cc27-compose.sh |
| CC-28 | 018 | Control panel reconfigures a running instance | core/manager.rs, core/realizations.rs, websrc/lib.rs (reconfigure) | Realization (Configuration), KappaStore | BroadcastChannel | yes | live: cc28-reconfigure.sh |
| CC-29 | 019 | Verification only at trust boundary, not every local read | core/peer.rs, core/lib.rs (ReadVerify) | verify_kappa, get_with_fetch | none | yes | live: cc29-read-verify-boundary.sh |
| CC-30 | 009 | Suspended RISC-V machine resumes byte-identically | core/emulator.rs (snapshot/restore, lines 2979, 3108) | none (snapshot bytes hashed to kappa) | none | yes | live: cc30-resume.sh |
| CC-31 | 019 | Browser tab reload resumes machine from OPFS snapshot | holospace-fs/extension.js (save/loadSnapshot), websrc/lib.rs (suspend, resume_devcontainer) | kappa re-derivation of snapshot | OPFS, CompressionStream | yes | live per doc; no cc31 file in vv/suites. Host witness cc31_resume_terminal runs inside cc30-resume.sh:37; browser witness resume-test.mjs runs from scripts/browser-manager-test.sh |
| CC-32 | ~019 | Each holospace keeps its own separate resume slot | holospace-fs/extension.js (namesFor, sanitizeHoloKey), web/index.html (holoKeyOf) | none | OPFS | yes | live per doc; no file in vv/suites or vv/targets. Witness snapshot-keying-test.mjs only in scripts/browser-manager-test.sh |
| CC-33 | 020, 014 | Workbench dials a TCP server inside the guest in-process | core/emulator/net.rs, core/emulator/devbus.rs, websrc/lib.rs (dial_guest) | none | none | yes | live: cc33-guest-bridge.sh |
| CC-34 | 020, 001 | holospaces itself acts as the VS Code remote | holospace-fs/extension.js, holospace-fs/node-exthost.js | none | Worker | yes | live per doc; no cc34 file in vv/suites. Covered by cc18-lsp.sh and cc48-ext-host.sh |
| CC-35 | 021 | AArch64 integer instruction set executes correctly | core/emulator/aarch64.rs | none | none | yes | live: cc35-aarch64-core.sh |
| CC-36 | 021 | Real arm64 Linux boots to userspace | core/emulator/aarch64.rs | none | none | yes | live: cc36-aarch64-linux.sh |
| CC-37 | 021 | arm64 devcontainer runs stock linux-arm64 binaries | core/emulator/aarch64.rs, core/emulator/devbus.rs | KappaStore (kappa-disk) | none | yes | live: cc37-aarch64-devcontainer.sh |
| CC-38 | 006 | Peers exchange content by hash over one portable protocol | core/content_net.rs | net-bare BareNetSync (KappaSync), NetworkInterface HAL | none (transport in CC-49) | n/a (needs a peer) | live: cc38-content-net.sh |
| CC-39 | ~014 | Owned node forwards guest TCP, syncs storage, takes updates | crates/holospaces-node/src/egress.rs, storage.rs, ota.rs, main.rs | store-native KappaStore, net-http /cas | WebSocket | no | live: cc39-node-egress.sh |
| CC-40 | ~001 | Security properties enforced: integrity, capability, identity | core/boot.rs (Capabilities), core/content_net.rs, core/identity.rs | verify_kappa, capabilities | none | yes | live: cc40-product-security.sh |
| CC-41 | ~014 | Chrome extension gives guest raw TCP, no node | crates/holospaces-web/extension/background.js, connector.js, manifest.json | none | Direct Sockets (TCPSocket), chrome.runtime port; Chrome only | no | live: cc41-extension-egress.sh |
| CC-42 | ~013 | Browser pulls real OCI image into bootable rootfs | core/oci.rs (ImagePull), websrc/lib.rs (DevcontainerProvision), web/provision-worker.mjs, web/index.html | KappaStore, digest verification | Worker, OPFS sync access handle, extension fetch | no | live per doc; no file in vv/suites or vv/targets. Witness: oci.rs unit test and manager-test.mjs (browser CI job) |
| CC-43 | 021 | x86-64 integer core decodes and runs machine code | core/emulator/x64.rs | none | none | yes | live: cc43-x64-core.sh |
| CC-44 | 021 | Real amd64 Linux boots to userspace | core/emulator/x64.rs | none | none | yes | live: cc44-x64-linux.sh |
| CC-45 | ~021 | amd64 devcontainer runs stock binaries; compiles in guest | core/emulator/x64.rs, websrc/lib.rs (X64Workspace) | KappaStore (paged kappa-disk) | OPFS, Worker | yes (blank holospace); no for registry images | live: cc45-x64-devcontainer.sh (open fork/exec heap defect; dogfood suite is in vv/heavy, not gated) |
| CC-46 | 021 | 9p, network, bridge shared across all three cores | core/emulator/devbus.rs, aarch64.rs, x64.rs | none | none | yes | live: cc46-devbus-arch-parity.sh |
| CC-47 | ~009 | Suspend and resume on arm64 and x86-64 | not implemented (no snapshot fn in aarch64.rs or x64.rs; witness file absent) | n/a | OPFS (planned) | n/a | target: vv/targets/cc47-lifecycle-arch-parity.sh |
| CC-48 | 020 | Node-only marketplace extension activates in the tab | holospace-fs/node-exthost.js, holospace-fs/extension.js | none (JS on browser engine) | Worker, fetch | no (Open VSX download) | live: cc48-ext-host.sh |
| CC-49 | 006 | Two browser tabs exchange content over WebRTC | websrc/webrtc.rs, websrc/lib.rs (cn_*), web/index.html | net-bare BareNetSync (KappaSync) | WebRTC data channel | LAN only (host candidates, manual signaling) | live: cc49-webrtc-content-net.sh |
| CC-50 | ~011 | Large rootfs assembled as a stream, never fully in RAM | core/assembly/ext4.rs, core/assembly.rs, core/disk.rs, web/provision-worker.mjs | KappaStore (kappa-disk from block stream) | OPFS sync access handle, Worker | yes (given layers) | live: cc50-streaming-rootfs-assembly.sh |
| CC-51 | ~012 | Source Control view: status, diff, commit, branch, push, pull | web/builtin-extensions/holospace-scm/extension.js, vendor/isomorphic-git | none | Worker, fetch (smart-http; no CORS proxy set) | yes locally; no for push/pull | live: cc51-scm-git.sh |
| CC-52 | ~012 | Find and replace across workspace files | web/builtin-extensions/holospace-search/extension.js, search-core.cjs | none | Worker | yes | live: cc52-search.sh |
| CC-53 | ~012 | tasks.json tasks run in guest with exit code, problems | web/builtin-extensions/holospace-tasks/extension.js, tasks-core.cjs, core/boot.rs (agent in /init) | none | Worker | yes | live: cc53-tasks.sh |

Count by directory check: 48 rows have a file named for them in vv/suites, 1 row (CC-47) is in vv/targets, 4 rows (CC-31, CC-32, CC-34, CC-42) are called live in the doc but have no file of their own in either directory.

## Facts

Legend: [code] = verified by reading source; [doc] = only claimed in docs or comments.

### 1. Performance
- No measured or targeted throughput, boot time, time to shell, or resume time is recorded anywhere in the repo. [code, by search]
- The only probe is `crates/holospaces/tests/p2_throughput.rs`: it boots RISC-V Linux (128 MiB RAM, line 50) and prints seconds, instruction count and MIPS (lines 73-76). The single assertion is a floor: `mips > 1.0` (lines 82-83). It is `#[ignore]`, native release only (line 39). Debug builds are "~10x slower" (line 13). [code]
- CI skips it: `HOLOSPACES_SKIP_PERF: '1'` at `.github/workflows/ci.yml:102`; `vv/run.sh:42-44` honours that. No number is stored in the repo. [code]
- Only stated speed figure: "~tens of MIPS, un-JIT-able under unsafe_code=forbid" for the emulated x86-64 guest, in a comment at `crates/holospaces-web/web/ext-host-test.mjs:10-11` and `vv/suites/cc48-ext-host.sh:22`. [doc]
- Indirect timing evidence: the x86-64 heap defect appears "after ~25 min" (commit message) / "~1500 s" (`docs/10-Quality-Requirements.md:112`). Browser tests allow 180 s for a boot (`web/vscode-workbench-holospace-test.mjs:123`). These are ceilings, not measurements.
- All three cores are pure fetch-decode-execute interpreters. RISC-V `step` fetches and decodes each instruction every time (`core/emulator.rs:3863-3885`); x64 `step` at `x64.rs:2721`; aarch64 `step` at `aarch64.rs:391`. No JIT, no block cache, no decoded-instruction cache. [code]
- The only caches are software TLBs: RISC-V 256 sets per access class (`emulator.rs:461`), x64 1024 sets (`x64.rs:607`), aarch64 1024 sets (`aarch64.rs:3435`), plus a RAM fast path (`emulator.rs:442`, `3292`). [code]
- `unsafe_code = "forbid"` workspace-wide (`Cargo.toml` workspace lints). The web crate builds with opt-level 3, fat LTO, one codegen unit, panic=abort, wasm-opt (`crates/holospaces-web/Cargo.toml` release profile). [code]
- The browser pump is cooperative: `ws.run(8_000_000)` then `setTimeout(pump, 40)` (`holospace-fs/extension.js:636-639`); with the LSP bridge `ws.run(6_000_000)` every 25 ms (line 783-785). [code]

### 2. Internet egress
One guest-side stack (virtio-net + userspace NAT, `core/emulator/net.rs`) and one frame protocol (OPEN/DATA/CLOSE by connection id). Carriers:
- Native host socket (`StdEgress`): native peer only. [code]
- WebSocket to `web/relay.mjs` (Node, 164 lines): needs a Node process outside the tab. Referenced only by `devcontainer-net-test.mjs` and CC-16 provenance; it is a test relay, not deployed. Browser side is `websrc/wsnet.rs:96`. [code]
- WebSocket to a holospaces-node (`crates/holospaces-node/src/egress.rs`, 284 lines; 10 s connect timeout at line 46): needs a native process or device you run. Selected by `?egress=wss://...` (`holospace-fs/extension.js:291`, `411-412`; UI at `web/index.html:215-217`). [code]
- Chrome extension (`crates/holospaces-web/extension/`): MV3 service worker using Direct Sockets `TCPSocket` (`background.js:1-11`). Needs an installed unpacked extension plus Direct Sockets enabled by policy or flag (`extension/README.md:27-31`); Chrome/ChromeOS only. The extension id constant is empty (`connector.js:15`), and `externally_connectable` is pinned to `https://hologram-technologies.github.io/*` (`manifest.json:15-17`). Its second role is a CORS-free `fetch` for OCI layers (`background.js:44-60`, `web/index.html:342-372`). [code]
- "Mesh to an exit peer" (listed in `docs/07-Deployment-View.md:47-48` and `extension/README.md:13`): no TCP-egress-over-WebRTC code exists; `content_net` carries only fetch/announce/discover of content. [code, by search]
- `websrc/webrtc.rs`: content exchange between tabs only, not guest internet. No iceServers, host candidates only, manual copy-paste signaling (`webrtc.rs:156-159`). [code]
- In-process guest bridge (`dial_guest`): inward only, no network. [code]

Static hosting only (no extension, no node), verified in code:
- Blank holospace boots a bundled per-arch busybox layer served from the site (`holospace-fs/extension.js:337-340`, `366-369`). Shell, editor, search, tasks work; the guest has no internet.
- `devcontainer.json` discovery via `raw.githubusercontent.com` / GitLab raw, which permit CORS (`web/index.html:613-626`).
- Open VSX gallery and extension downloads by plain fetch.
- Git push/pull from holospace-scm uses isomorphic-git smart-http with `corsProxy: undefined` (`holospace-scm/extension.js:582`), so it works only against git hosts that send CORS headers. The witness uses a local `git http-backend`. GitHub's git endpoint support is not shown anywhere in the repo.
- OCI registry pulls do not work: the page treats registries as CORS-blocked and routes them through the extension (`web/index.html:342-343`; `docs/10:109`). A repo launch without the router shows a prompt.

### 3. Threads and cross-origin isolation
- No SharedArrayBuffer, Atomics, wasm threads, COOP or COEP anywhere: zero hits in `crates/holospaces-web`, `scripts`, `.github`, Cargo manifests. `scripts/dev-site.py` sends only Cache-Control / Content-Type headers (lines 146, 176-185). No service worker in the repo. [code, by search]
- Single-threaded emulator. Workers:
  1. The VS Code web extension-host worker: runs holospace-fs, which loads the wasm peer and runs the whole emulator there, plus scm, search, tasks and the Node-API extension host (`holospace-fs/extension.js:1-12`, `264-265`). OPFS sync access handles are opened in this worker (`extension.js:186`, `203`).
  2. `web/provision-worker.mjs` (94 lines): one dedicated worker spawned by the Manager (`web/index.html:410`) to stream rootfs assembly into OPFS.
  3. VS Code's own internal workers (not holospaces code).
- The Manager page (`index.html`) runs the wasm Console on the main thread; the fallback `workspace.html` runs the emulator on the main thread in 5M-instruction chunks (`workspace.html:212-217`).
- Tabs talk over `BroadcastChannel("holospaces-control")` (`index.html:329`, `extension.js:1115`).

### 4. Workbench
- Pins: `vscode-web@1.91.1` and `@vscode/test-web@0.0.80` (`web/build-workbench.mjs:20-21`; `web/package.json` devDependencies), `playwright ^1.60.0` for tests. Installed by `npm install --no-save` at build time (line 125).
- Build: verify 9 core files against `vv/artifacts/cc17/vendor.sha256` (lines 107-116, 127), copy the whole `vscode-web/dist` to `_site/workbench` (line 131), copy four builtin extensions to `_site/ext/<name>` (lines 132-134), then patch the stock `workbench.html`: inject a runtime config script, swap the server bootstrap for test-web's `create()` bootstrap, fix base URL for sub-path hosting (lines 67-103). Gallery is Open VSX. Holospace id arrives as `?id=<kappa>` and becomes the folder authority `holospace://<kappa>/workspace` (lines 48-56).
- Shipped size: not stated anywhere in the repo. The vscode-web dist is not in the clone.
- holospace-fs (1144 + 452 lines): boots the wasm emulator inside the extension host; FileSystemProvider for `holospace:` calling wasm exports `ws_stat_path/list/read/write/mkdir/delete/rename` (lines 555-606), which are host-side 9p workspace calls; terminal via `feed_input` / `terminal_delta` (lines 636-669); LSP via `dial_guest/guest_send/guest_recv` (lines 716-794); snapshots; Node-API extension host (`node-exthost.js`).
- holospace-scm (689 lines): `vscode.scm` provider running isomorphic-git 1.27.1 (vendored, sha256-pinned) in the extension host. Reaches the guest only through `vscode.workspace.fs` on `holospace://`, i.e. holospace-fs, i.e. 9p. Network for push/pull is browser fetch, not the guest.
- holospace-search (213 + 191 lines): File and Text search providers (proposed APIs), JS tree walk over `vscode.workspace.fs`; vendored `ignore` for .gitignore. Does not execute in the guest.
- holospace-tasks (285 + 175 lines): TaskProvider with CustomExecution. Writes `.hs-tasks/<id>.cmd` over 9p; a `/bin/sh` loop seeded into guest `/init` runs it and writes `<id>.out` and `<id>.exit`, which the extension polls. The only one of the three that executes inside the guest.

### 5. Suspend / resume
- Snapshot content (`Emulator::snapshot`, `core/emulator.rs:2979-3095`): hart and device state, virtio-blk transport state plus disk as sparse occupied sectors (lines 3041-3056), virtio-9p state plus the whole workspace file tree (lines 3059-3088), then full guest RAM (line 3094). Not included: virtio-net and live connections (lines 3090-3093), console output buffer (`docs/10:98`).
- Browser: `ws.suspend()` bytes are gzipped and written to OPFS as `holospace.<key>.snapshot.gz` with the hash in `holospace.<key>.snapshot.kappa`; scrollback is stored beside it (`holospace-fs/extension.js:73-80`, `235-244`). On load the hash is re-derived and a mismatch forces cold boot (lines 127-146). Key = sanitized holospace kappa from the folder authority, else `default` (lines 65-72). Saved every 120 s by timer (line 1094).
- Default RISC-V machine RAM is 512 MiB (`core/machine.rs:135`); the snapshot holds it uncompressed in memory before gzip.
- Witnessed live: RISC-V only (cc30-resume.sh, cc31_resume_terminal, resume-test.mjs). AArch64 and x86-64: target only (CC-47); neither core has a snapshot function and `tests/cc47_lifecycle_parity.rs` does not exist. `saveSnapshot` skips cores without `suspend` (`extension.js:228-230`).
- Networked (bridged) RISC-V boots are also excluded from resume and cold-boot each session (`extension.js:255-261`, `1089-1096`).
- The Manager launches amd64 by default (`extension.js:287-289` comment), so the default product path has no resume. [code + comment]

### 6. Lines of Rust (wc -l)
- crates/holospaces: 43560 total (src 31638; tests + examples 11922)
- crates/holospaces-web: 3307 (lib.rs 2608, webrtc.rs 337, wsnet.rs 237, opfs_store.rs 125)
- crates/holospaces-node: 801 (src 691; tests 110)
- crates/holospaces-emulator: 167
- Emulator core: x64.rs 6201, emulator.rs (RISC-V + virtio + 9p + snapshot) 5892, aarch64.rs 5882, net.rs 2020, devbus.rs 643. Total 20638.
- Other large files: assembly.rs 1447, boot.rs 1334, assembly/ext4.rs 1180, oci.rs 1015.

### 7. Open x86-64 defect (git log -1)
- Under sustained fork/exec load the amd64 guest heap corrupts: `malloc(): unsorted double linked list corrupted` after about 25 minutes (about 1500 s guest time); a process, sometimes PID 1, dies.
- Narrowed, still open: fork/exec specific; survives a full software-TLB bypass; unchanged by the frame-0 fix; zero COW stale-writable hits. Conclusion in the commit: a rare per-instruction mis-execution on the fork/exec path, needing a qemu differential trace.
- Two real bugs fixed in the same commit: TSC made invariant (dcache timing model removed) and page-fault scratch moved off guest frame 0.
- Trigger is thousands of fork/exec cycles, i.e. a shell loop or an in-guest build. Also recorded in `docs/10-Quality-Requirements.md:112`.

### 8. Mobile and browser support
- No browser-support matrix anywhere. All browser witnesses run headless Chromium via Playwright; no Firefox or Safari run exists.
- Mobile mentions: one goal sentence, "on a Chromebook or Android tablet" (`docs/01-Introduction-and-Goals.md:24`), and a viewport meta tag in `index.html:5` and `workspace.html:5`. No touch handling, no iOS/Safari statement, no memory budget for mobile.
- Hard dependencies visible in code: OPFS `createSyncAccessHandle` in a worker, `CompressionStream`, `BroadcastChannel`, WebRTC, and for internet either Chrome Direct Sockets or an external node.

### Contradictions with the docs
- `docs/10:140` says "CC-47 .. CC-49 are targets"; the directories and `vv/run.sh:112-113` show only CC-47 is a target.
- `docs/10:131-133` says every CC row has a suite in `vv/suites/`; CC-31, CC-32, CC-34, CC-42 have none. `vv/run.sh:109-122` will therefore always print those four as "pending".
- `docs/10:160-161` says CI runs "CC-1..CC-11" suites; there are 51 suite scripts plus perf.
- `docs/10:108` (CC-41) says the extension has "no host permissions"; `manifest.json:19` requests `*://*/*` and adds a content script. The extension README admits this.
- `scripts/browser-manager-test.sh:83` and `extension.js:1143` label snapshot keying "CC-31"; the catalog calls it CC-32.
- Docs present the emulator as a codemodule run by hologram's wasmi ContainerEngine in the browser (`docs/07`, RT2). The deployed workbench path holds `Emulator` directly as a Rust field compiled to wasm (`websrc/lib.rs:1263-1264`); the codemodule crate is 167 lines and is used by CC-6/CC-9 style witnesses, not by the workbench.
- "Mesh" egress is listed as one of three exit surfaces but has no implementation.
- "KappaSync" for operator sync (CC-23, QS5): not verified in code beyond `content_net`; treat as doc claim.
- CC-27 is marked live though the row itself says the routing "is not driven end-to-end".

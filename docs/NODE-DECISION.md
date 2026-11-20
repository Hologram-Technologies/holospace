# Node decision

2026-11-20. Provisional: the guest measurements the prompt asked for could not be taken.

## Answer

Holospace does not replace Node.js. Keep both paths, with one rule for which a workload takes.
The thin Node layer stays in PrismPM, next to Prism IDE.
Holospace's Linux guest is the path for everything the thin layer cannot run.

## Why

The thin layer is fast and partial. The Node session measured that a browser shim reaches at most 38.9% of Node-only extensions; 61.1% never run, because they need native binaries, real processes or sockets.
The guest is complete and slow. It runs real Node on a real kernel, but on an interpreter estimated at tens of millions of instructions per second (a code comment, not a measurement). Node start-up and `npm install` will take seconds to minutes there (estimate, unverified).
Neither covers the other's weakness, so neither replaces the other.
The thin layer has nothing to do with booting machines, so it does not belong in Holospace. Its users are the IDE and Prism apps, which live in PrismPM.

## Rule

A workload goes to the thin layer if it needs only `fs`, `path`, `events`, `util`, `buffer`, `http` over `fetch`, or a fork of its own JavaScript.
Anything else goes to the guest: native modules, child processes that are not JavaScript, raw sockets, package install scripts.
The choice is a decision, so it belongs in the capability gate kernel, not in ad hoc code.

## Duplication to remove

There are three Node shims in play: the planned Prism Node layer, the Node-API extension host inside holospaces' `holospace-fs` extension (`node-exthost.js`), and whatever monaco-vscode-api provides.
Keep one: the Prism Node layer. Delete `node-exthost.js` when Holospace moves to Prism IDE.

## Not measured, and why

Node start-up and `npm install` in the guest. The only guest that boots without an extension is a busybox image with no Node and no network, and it panicked on the first command I ran.
To settle this: a riscv64 image with Node, booted on the native build of the emulator, timing `node -e 0` and a small `npm install` from a local cache.
If guest Node start-up proves to be under about 2 s after resume, revisit the rule; the guest could then take more of the load.

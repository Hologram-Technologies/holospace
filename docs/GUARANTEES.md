# Guarantees proposed for proof

2026-11-20. For approval. Agree or strike each line; nothing is proved yet.
Lean would prove these about the models. Whether the running code obeys the models is tested, not proved.

## Lifecycle

1. A holospace is in exactly one state: provisioned, running, suspended or terminated.
2. Only these moves happen: provisioned to running; running to suspended; suspended to running; any state to terminated.
3. Terminated is final.
4. A suspend that was acknowledged has a recorded snapshot hash.
5. A resume uses exactly the most recently recorded snapshot hash, never an older one.
6. The same state and event always give the same result.

## Isolation

7. Every durable storage key is derived from the holospace's identity.
8. Two holospaces with different identities never share a storage key.
9. A read made for one holospace never returns state written for another.

## Admission

10. Received bytes are accepted only if their recomputed hash equals the hash that was requested.
11. A mismatch is never accepted; the request moves to the next source.
12. If every source fails, the result is a failure, never partial content.
13. Content named by SHA-256 is accepted only at import, and is stored under its BLAKE3 hash.

## Launch verdict

14. An app is shown as verified only if its seal, its member identities, its manifest and provenance, and every acceptance vector all pass.
15. If any check fails or is skipped, the verdict is unverified.
16. An unverified app does not run.

## Capability gate (reused from Prism IDE, extended)

17. Everything is denied unless granted.
18. A deny always wins over a grant.
19. A guest or app reaches the network only under an explicit grant.
20. A connection that leaves the tab through an extension or a node is always labelled as not serverless.

## Provisioning identity

21. The same definition gives the same identity, on any device.
22. The architecture is part of the identity; changing it changes the identity.

## Decision needed on line 16

Line 16 is strict. The alternative is to run an unverified app behind a visible warning. I recommend the strict form.

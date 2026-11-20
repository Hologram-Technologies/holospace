Feature: holospace

  This repository's Prism application, in app/: the Holospace kernel. One request is a
  kernel word followed by that kernel's fields, and one LexLean module decides each kernel.
  Each claim is every theorem of one module, listed in holospace.pins.tsv.

  @HK-01 @build
  Scenario: An app is answered verified exactly when its seal, members, provenance and acceptance vectors all pass, and blocked otherwise
    Given the model in app/src/HolospaceLaunchVerdictProofs.lex.tex
    When the Lean kernel checks its 5 theorems during prismpm verify
    Then no request meets any condition holospace.pins.tsv lists for verdict
    And requests that begin with verdict are answered by HolospaceLaunchVerdict

  @HK-02 @build
  Scenario: Bytes whose hash differs from the requested name are never accepted or re-keyed, and a SHA-256 name is re-keyed only at import
    Given the model in app/src/HolospaceAdmissionProofs.lex.tex
    When the Lean kernel checks its 6 theorems during prismpm verify
    Then no request meets any condition holospace.pins.tsv lists for admission
    And requests that begin with admission are answered by HolospaceAdmission

  @HK-03 @build
  Scenario: A holospace is never allowed another holospace's private state, is always allowed its own, and never writes shared content
    Given the model in app/src/HolospaceIsolationProofs.lex.tex
    When the Lean kernel checks its 3 theorems during prismpm verify
    Then no request meets any condition holospace.pins.tsv lists for isolation
    And requests that begin with isolation are answered by HolospaceIsolation

  @HK-04 @build
  Scenario: Nothing is allowed without a grant, a denied request is always denied, and a network request through an extension or a node is never answered allow
    Given the model in app/src/HolospaceCapabilityGateProofs.lex.tex
    When the Lean kernel checks its 16 theorems during prismpm verify
    Then no request meets any condition holospace.pins.tsv lists for gate
    And requests that begin with gate are answered by HolospaceCapabilityGate

  @HK-05 @build
  Scenario: Start, suspend and resume are answered only in the phase each needs, resume never without a snapshot, and nothing follows terminate
    Given the model in app/src/HolospaceLifecycleProofs.lex.tex
    When the Lean kernel checks its 8 theorems during prismpm verify
    Then no request meets any condition holospace.pins.tsv lists for lifecycle
    And requests that begin with lifecycle are answered by HolospaceLifecycle

  @HK-06 @build
  Scenario: The same definition and architecture give the same encoding, and a different architecture or definition always gives a different one
    Given the model in app/src/HolospaceIdentity.lex.tex
    When the Lean kernel checks its 3 theorems during prismpm verify
    Then no request meets any condition holospace.pins.tsv lists for identity

//! The `holospace` suite: one test per scenario in `features/suites/holospace.feature`.
//!
//! The proofs are checked by the Lean kernel when `just vv` runs `prismpm verify`
//! on `app/`. What these tests hold fixed is the claim each proof is about: the
//! condition no request may meet, the theorem that states none does, and that a
//! kernel's word reaches that kernel's own judge. A weakened condition would
//! still verify, and would no longer be the claim its row makes. The pins are
//! data, in `features/suites/holospace.pins.tsv`: kernel, module, declaration,
//! and the text that declaration must contain.

use std::path::PathBuf;

fn read(path: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf();
    std::fs::read_to_string(root.join(path)).unwrap_or_else(|_| panic!("{path} is missing"))
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    let end = model[start..]
        .find("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"));
    &model[start..start + end]
}

/// Every pin of `kernel` holds, and every theorem of `proofs` has one.
fn pinned(kernel: &str, proofs: &str) {
    let key = format!("{kernel}\t");
    let pins = read("features/suites/holospace.pins.tsv");
    let mut theorems = 0;
    for pin in pins.lines().filter(|pin| pin.starts_with(&key)) {
        let columns: Vec<&str> = pin.split('\t').collect();
        let [_, module, name, text] = columns[..] else {
            panic!("a pin has four columns: {pin}");
        };
        let source = read(&format!("app/src/{module}.lex.tex"));
        let found = declaration(&source, name);
        assert!(found.contains(text), "{name} of {module} changed:\n{found}");
        theorems += usize::from(text.starts_with("statement: "));
    }
    let all = read(&format!("app/src/{proofs}.lex.tex"))
        .matches("\ntheorem{\n")
        .count();
    assert!(
        all > 0 && theorems == all,
        "{theorems} of {all} theorems of {proofs} are pinned"
    );
}

/// `HK-01`: An app is answered verified exactly when its seal, members, provenance and acceptance vectors all pass, and blocked otherwise
#[test]
fn holospace_hk_01() {
    pinned("verdict", "HolospaceLaunchVerdictProofs");
}

/// `HK-02`: Bytes whose hash differs from the requested name are never accepted or re-keyed, and a SHA-256 name is re-keyed only at import
#[test]
fn holospace_hk_02() {
    pinned("admission", "HolospaceAdmissionProofs");
}

/// `HK-03`: A holospace is never allowed another holospace's private state, is always allowed its own, and never writes shared content
#[test]
fn holospace_hk_03() {
    pinned("isolation", "HolospaceIsolationProofs");
}

/// `HK-04`: Nothing is allowed without a grant, a denied request is always denied, and a network request through an extension or a node is never answered allow
#[test]
fn holospace_hk_04() {
    pinned("gate", "HolospaceCapabilityGateProofs");
}

/// `HK-05`: Start, suspend and resume are answered only in the phase each needs, resume never without a snapshot, and nothing follows terminate
#[test]
fn holospace_hk_05() {
    pinned("lifecycle", "HolospaceLifecycleProofs");
}

/// `HK-06`: The same definition and architecture give the same encoding, and a different architecture or definition always gives a different one
#[test]
fn holospace_hk_06() {
    pinned("identity", "HolospaceIdentity");
}

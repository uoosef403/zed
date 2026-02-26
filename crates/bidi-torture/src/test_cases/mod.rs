// Test cases module — organized by difficulty, each level harder than the last.
//
// Level 1:  Pure scripts (trivial)
// Level 2:  Simple LTR↔RTL mixing
// Level 3:  Numbers inside RTL context
// Level 4:  Explicit embeddings (LRE, RLE, LRO, RLO, PDF)
// Level 5:  Directional isolates (LRI, RLI, FSI, PDI)
// Level 6:  Bracket/mirror pairing
// Level 7:  Weak & neutral type resolution
// Level 8:  Line breaking inside bidi runs
// Level 9:  ZWJ, marks, overrides, invisible codepoints
// Level 10: Adversarial — max nesting, spoofing, stress

mod level_01_pure;
mod level_02_simple_mixed;
mod level_03_numbers;
mod level_04_embeddings;
mod level_05_isolates;
mod level_06_brackets;
mod level_07_weak_neutral;
mod level_08_linebreak;
mod level_09_marks_zwj;
mod level_10_adversarial;

use crate::BidiTestCase;

/// All test cases, concatenated in order of difficulty.
pub fn all_cases() -> Vec<BidiTestCase> {
    let mut cases = Vec::new();
    cases.extend_from_slice(level_01_pure::CASES);
    cases.extend_from_slice(level_02_simple_mixed::CASES);
    cases.extend_from_slice(level_03_numbers::CASES);
    cases.extend_from_slice(level_04_embeddings::CASES);
    cases.extend_from_slice(level_05_isolates::CASES);
    cases.extend_from_slice(level_06_brackets::CASES);
    cases.extend_from_slice(level_07_weak_neutral::CASES);
    cases.extend_from_slice(level_08_linebreak::CASES);
    cases.extend_from_slice(level_09_marks_zwj::CASES);
    cases.extend(level_10_adversarial::cases());
    cases
}

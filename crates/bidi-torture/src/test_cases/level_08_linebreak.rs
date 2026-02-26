// ═══════════════════════════════════════════════════════════════════════════
// Level 8 — Line Breaking Inside BiDi Runs
// ═══════════════════════════════════════════════════════════════════════════
// When bidi text wraps to a new line, each visual line is an independent
// bidi paragraph for L1/display purposes.  The reordering must be applied
// AFTER line breaking, and trailing whitespace on each line reverts to the
// paragraph embedding level.
//
// Getting this wrong causes characters to jump to the wrong line or
// trailing spaces to appear on the wrong side.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── Simple wrapping ───────────────────────────────────────
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Long Arabic sentence that should wrap — each visual line re-resolved",
        text: "هذا نص طويل جداً في اللغة العربية يجب أن يتم تقسيمه على عدة أسطر عند العرض",
        expected_visual: "(each wrapped visual line should be independently right-aligned)",
    },
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Long mixed text wrapping — LTR paragraph with Arabic island",
        text: "This is a somewhat long English sentence that contains عربي طويل داخل الجملة and continues in English after that Arabic portion",
        expected_visual: "(Arabic portion stays reversed within its visual line)",
    },

    // ── Trailing whitespace (L1 rule) ─────────────────────────
    BidiTestCase {
        category: "L8 · Line Break",
        description: "L1: Trailing spaces in RTL paragraph revert to embedding level",
        text: "عربي   ",
        expected_visual: "   يبرع  ← (trailing spaces go to embedding dir → right side in RTL)",
    },
    BidiTestCase {
        category: "L8 · Line Break",
        description: "L1: Trailing tab in LTR paragraph",
        text: "Hello\t",
        expected_visual: "Hello\t  ← (trailing whitespace stays at end in LTR)",
    },
    BidiTestCase {
        category: "L8 · Line Break",
        description: "L1: Trailing spaces after mixed LTR+RTL",
        text: "abc عربي   ",
        expected_visual: "abc يبرع     ← (trailing WS reverts to paragraph embedding)",
    },

    // ── Soft hyphen and word break ────────────────────────────
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Soft hyphen (U+00AD) in Arabic word — break opportunity",
        text: "كلمة\u{00AD}طويلة\u{00AD}جداً",
        expected_visual: "اًدج-ةليوط-ةملك  ← (soft hyphens become visible at breaks)",
    },
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Zero-width space (U+200B) as break opportunity in Arabic",
        text: "كلمة\u{200B}طويلة",
        expected_visual: "ةليوط\u{200B}ةملك  ← (can break here but no visible hyphen)",
    },

    // ── Mixed LTR/RTL at wrap point ───────────────────────────
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Wrap occurs at the LTR↔RTL boundary",
        text: "word word word word word word word word word word عربي عربي عربي عربي",
        expected_visual: "(if wrap occurs at boundary, Arabic continues on next line RTL)",
    },
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Wrap occurs INSIDE an RTL run",
        text: "text text كلمة طويلة جداً في النص العربي وكلمات أخرى end",
        expected_visual: "(RTL run broken across lines, each segment independently reversed)",
    },

    // ── Multiple newlines with mixed content ──────────────────
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Two physical lines: LTR then RTL (paragraph per line)",
        text: "Hello World",
        expected_visual: "Hello World  ← (single line, trivial)",
    },
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Very long alternating LTR/RTL that will force multiple wraps",
        text: "alpha عربي beta عربي gamma عربي delta عربي epsilon عربي zeta عربي eta عربي theta عربي",
        expected_visual: "(each wrap produces a new visual line; runs stay correctly ordered)",
    },

    // ── Whitespace-only runs between bidi transitions ─────────
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Multiple spaces between LTR and RTL — spaces are wrap candidates",
        text: "word     عربي",
        expected_visual: "word     يبرع  ← (spaces may collapse or wrap becomes interesting)",
    },
    BidiTestCase {
        category: "L8 · Line Break",
        description: "Tab characters between LTR and RTL",
        text: "left\t\tعربي",
        expected_visual: "left\t\tيبرع  ← (tabs are neutral WS, resolved by context)",
    },
];

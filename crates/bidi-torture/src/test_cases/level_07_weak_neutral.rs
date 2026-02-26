// ═══════════════════════════════════════════════════════════════════════════
// Level 7 — Weak & Neutral Type Resolution
// ═══════════════════════════════════════════════════════════════════════════
// The UBA classifies every character into types: Strong (L, R, AL), Weak
// (EN, ES, ET, AN, CS, NSM, BN), and Neutral (B, S, WS, ON).
//
// Weak types like European Separator (comma, period in numbers), Common
// Separator (colon), and Neutral types like whitespace and most punctuation
// are resolved based on their adjacent strong types.
//
// This level tortures the W1–W7 and N1–N2 rules of UAX#9.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── W1: NSM (Non-Spacing Mark) ────────────────────────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Combining mark after LTR char — inherits L type",
        text: "e\u{0301} text",  // é decomposed
        expected_visual: "é text  ← (NSM gets type of preceding base char)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Combining mark after RTL char",
        text: "بِ نص",  // Arabic ba with kasra
        expected_visual: "صن ِب  ← (NSM inherits R from Arabic base)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Multiple combining marks stacked on RTL base",
        text: "بِّ test",  // ba + kasra + shadda
        expected_visual: "test ِّب",
    },

    // ── W2: EN after AL → AN ──────────────────────────────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "European number after Arabic letter becomes AN",
        text: "عدد123",
        expected_visual: "123ددع  ← (W2: EN → AN because preceded by AL)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "EN after AL with neutral between — W2 still applies",
        text: "عدد 123",
        expected_visual: "123 ددع  ← (W2 looks past neutrals for preceding AL)",
    },

    // ── W4: ES between EN, CS between EN ──────────────────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Slash between European numbers (ES type)",
        text: "عربي 12/34 عربي",
        expected_visual: "يبرع 12/34 يبرع  ← (W4: '/' is ES, stays between ENs)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Period between numbers (CS type)",
        text: "عربي 3.14 عربي",
        expected_visual: "يبرع 3.14 يبرع  ← (W4: '.' is CS, keeps number together)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Comma as CS between numbers",
        text: "عربي 1,000 عربي",
        expected_visual: "يبرع 1,000 يبرع  ← (W4: ',' keeps number formatted together)",
    },

    // ── W5: ET adjacent to EN ─────────────────────────────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Currency sign (ET) before number",
        text: "عربي $100 عربي",
        expected_visual: "يبرع $100 يبرع  ← (W5: '$' is ET, becomes EN next to 100)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Percent sign (ET) after number",
        text: "عربي 50% عربي",
        expected_visual: "يبرع 50% يبرع  ← (W5: '%' is ET, becomes EN next to 50)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Currency sign NOT adjacent to number",
        text: "عربي $ عربي",
        expected_visual: "يبرع $ يبرع  ← (W5+W6: isolated ET → ON, resolved by neutral rules)",
    },

    // ── W6/W7: Remaining weak types → neutral ─────────────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Isolated comma (CS) between Arabic words",
        text: "كلمة, كلمة",
        expected_visual: "ةملك ,ةملك  ← (CS that isn't between numbers → ON)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "W7: EN after L stays EN, renders LTR",
        text: "abc 123 عربي",
        expected_visual: "abc 123 يبرع  ← (123 stays EN, embedded in LTR region)",
    },

    // ── N1: Neutral between same-type strong chars ────────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Space (WS) between two Arabic words — resolves to R",
        text: "عربي عربي",
        expected_visual: "يبرع يبرع  ← (space between R,R → R)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Dash (ON) between two Arabic words — resolves to R",
        text: "عربي-عربي",
        expected_visual: "يبرع-يبرع  ← (dash between R,R → R)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Dot (ON) between two English words — resolves to L",
        text: "hello.world",
        expected_visual: "hello.world",
    },

    // ── N1: Neutral between different-type strong chars ───────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Space between L and R — takes embedding direction",
        text: "abc عربي",
        expected_visual: "abc يبرع  ← (space between L,R → uses embedding level)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Multiple neutrals between L and R",
        text: "hello ... عربي",
        expected_visual: "hello ... يبرع  ← (dots and spaces all take embedding dir)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Exclamation between R and L",
        text: "عربي! English",
        expected_visual: "English !يبرع  ← ('!' between R,L → embedding dir LTR)",
    },

    // ── N2: Remaining neutrals get embedding direction ────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Leading neutrals in RTL paragraph",
        text: "... عربي",
        expected_visual: "يبرع ...  ← (leading '...' resolved to embedding dir RTL)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Trailing neutrals in RTL paragraph",
        text: "عربي ...",
        expected_visual: "... يبرع  ← (trailing '...' resolved to RTL)",
    },
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Only neutrals — no strong character at all",
        text: "...---+++",
        expected_visual: "...---+++  ← (N2: all neutrals → embedding dir, default LTR)",
    },

    // ── Sequence of weak types ────────────────────────────────
    BidiTestCase {
        category: "L7 · Weak/Neutral",
        description: "Complex: multiple weak sequences in one line",
        text: "عربي $100 + 50% - text",
        expected_visual: "text - 50% + $100 يبرع  ← (multiple weak groups resolved)",
    },
];

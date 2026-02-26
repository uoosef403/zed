// ═══════════════════════════════════════════════════════════════════════════
// Level 9 — ZWJ, Marks, Direction Marks, Invisible Codepoints
// ═══════════════════════════════════════════════════════════════════════════
// This level tests the interaction of bidi with:
//  - LRM (U+200F) / RLM (U+200E) — invisible strong direction hints
//  - ALM (U+061C) — Arabic Letter Mark
//  - ZWJ (U+200D) / ZWNJ (U+200C) — joiners that affect shaping
//  - CGJ (U+034F) — Combining Grapheme Joiner
//  - BN (Boundary Neutral) characters
//
// These invisible characters are heavily used to fix bidi issues in real
// text, and a correct implementation must handle them precisely.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── LRM (Left-to-Right Mark, U+200F bidi type L) ─────────
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "LRM forces neutral punctuation to resolve as LTR",
        text: "Hello\u{200E}! عربي",
        expected_visual: "Hello! يبرع  ← (LRM after Hello makes '!' resolve as L)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "LRM between number and Arabic — prevents EN→AN conversion",
        text: "عربي\u{200E}123",
        expected_visual: "123\u{200E}يبرع  ← (LRM separates EN from AL context)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "Multiple LRMs — only strong type matters, no visible effect",
        text: "\u{200E}\u{200E}\u{200E}عربي",
        expected_visual: "يبرع  ← (LRMs are invisible but set paragraph to LTR if first strong)",
    },

    // ── RLM (Right-to-Left Mark, U+200F bidi type R) ──────────
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "RLM forces paragraph direction to RTL",
        text: "\u{200F}Hello World",
        expected_visual: "Hello World  ← (paragraph is RTL, but English text is still L→R)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "RLM between English and punctuation",
        text: "Hello\u{200F}! World",
        expected_visual: "Hello !World  ← (RLM makes '!' resolve as R, reorders with World)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "RLM at end of line — changes trailing neutral resolution",
        text: "عربي ...\u{200F}",
        expected_visual: "... يبرع  ← (RLM at end forces trailing neutrals to R embedding)",
    },

    // ── ALM (Arabic Letter Mark, U+061C bidi type AL) ─────────
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "ALM used to fix number display in Arabic context",
        text: "\u{061C}123 عربي",
        expected_visual: "يبرع 123  ← (ALM makes paragraph RTL + triggers W2 on 123)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "ALM between number and Latin — creates AL context for EN",
        text: "عربي\u{061C}456abc",
        expected_visual: "abc456\u{061C}يبرع  ← (ALM inserts invisible AL between groups)",
    },

    // ── ZWJ (U+200D) — affects glyph shaping but is BN for bidi
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "ZWJ between Arabic letters — keeps joined form",
        text: "لا\u{200D}إله",
        expected_visual: "هلإ\u{200D}لا  ← (ZWJ forces joining, BN type for bidi)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "ZWJ in emoji sequence — family emoji",
        text: "👨\u{200D}👩\u{200D}👧\u{200D}👦 عربي",
        expected_visual: "يبرع 👨‍👩‍👧‍👦",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "ZWJ between LTR and RTL characters — invisible for bidi",
        text: "a\u{200D}ب",
        expected_visual: "aب  ← (ZWJ doesn't affect bidi resolution, only shaping)",
    },

    // ── ZWNJ (U+200C) — breaks joining but is BN for bidi ────
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "ZWNJ in Persian — prevents unwanted joining",
        text: "می\u{200C}خواهم",
        expected_visual: "مهاوخ\u{200C}یم  ← (ZWNJ breaks join, word looks correct in Farsi)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "Multiple ZWNJs in Arabic",
        text: "كلمة\u{200C}اخرى\u{200C}ثالثة",
        expected_visual: "ةثلاث\u{200C}ىرخا\u{200C}ةملك",
    },

    // ── Direction marks with brackets ─────────────────────────
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "LRM before opening paren in RTL context — fixes bracket direction",
        text: "عربي \u{200E}(English) عربي",
        expected_visual: "يبرع (English) يبرع  ← (LRM hints bracket should pair as LTR)",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "RLM after closing bracket in LTR — fixes bracket pairing",
        text: "The value (عربي\u{200F}) end",
        expected_visual: "The value (يبرع) end",
    },

    // ── BN (Boundary Neutral) behavior ────────────────────────
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "Soft hyphen (BN type) in RTL word — ignored for bidi",
        text: "كلمة\u{00AD}طويلة",
        expected_visual: "ةليوط\u{00AD}ةملك",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "Multiple BN chars between L and R — all stripped for resolution",
        text: "abc\u{200B}\u{200B}\u{200B}عربي",
        expected_visual: "abc\u{200B}\u{200B}\u{200B}يبرع",
    },

    // ── Combining marks across direction boundaries ───────────
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "Combining mark on last char of LTR run before RTL",
        text: "e\u{0301} عربي",
        expected_visual: "é يبرع",
    },
    BidiTestCase {
        category: "L9 · Marks/ZWJ",
        description: "Combining mark on first char of RTL run after LTR",
        text: "text بِداية",
        expected_visual: "text ةيادِب  ← (kasra stays on ba, not misplaced)",
    },
];

// ═══════════════════════════════════════════════════════════════════════════
// Level 4 — Explicit BiDi Embeddings (LRE, RLE, LRO, RLO, PDF)
// ═══════════════════════════════════════════════════════════════════════════
// These use the LEGACY explicit bidi formatting characters:
//   U+202A  LRE  Left-to-Right Embedding
//   U+202B  RLE  Right-to-Left Embedding
//   U+202C  PDF  Pop Directional Formatting
//   U+202D  LRO  Left-to-Right Override
//   U+202E  RLO  Right-to-Left Override
//
// Modern text should use isolates (Level 5), but legacy content is full of
// these and they MUST work correctly.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── LRE / PDF ─────────────────────────────────────────────
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "LRE forces LTR embedding inside RTL paragraph",
        // U+202A = LRE, U+202C = PDF
        text: "عربي \u{202A}hello world\u{202C} عربي",
        expected_visual: "يبرع hello world يبرع  ← (LRE keeps English LTR)",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "Nested LRE inside LRE",
        text: "عربي \u{202A}outer \u{202A}inner\u{202C} outer\u{202C} عربي",
        expected_visual: "يبرع outer inner outer يبرع",
    },

    // ── RLE / PDF ─────────────────────────────────────────────
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "RLE forces RTL embedding inside LTR paragraph",
        text: "English \u{202B}مرحبا\u{202C} English",
        expected_visual: "English ابحرم English  ← (RLE forces Arabic into RTL island)",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "RLE with English + Arabic inside LTR",
        text: "start \u{202B}عربي hello عربي\u{202C} end",
        expected_visual: "start يبرع hello يبرع end  ← (RLE creates RTL context for the region)",
    },

    // ── LRO / RLO (Overrides) ─────────────────────────────────
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "LRO — force EVERYTHING to be treated as LTR (even Arabic chars)",
        text: "\u{202D}مرحبا بالعالم\u{202C}",
        expected_visual: "مرحبا بالعالم  ← (Arabic glyphs but forced LTR order — looks broken intentionally)",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "RLO — force English letters to display RTL",
        text: "\u{202E}Hello World\u{202C}",
        expected_visual: "dlroW olleH  ← (English forced RTL character by character)",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "RLO on mixed — everything reversed",
        text: "\u{202E}abc عربي 123\u{202C}",
        expected_visual: "321 يبرع cba  ← (all chars forced RTL)",
    },

    // ── Unmatched / extra PDF ─────────────────────────────────
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "Orphan PDF without opener — should be ignored",
        text: "Hello \u{202C}World",
        expected_visual: "Hello World  ← (stray PDF has no effect)",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "LRE without closing PDF — implicitly closed at paragraph end",
        text: "Hello \u{202A}World",
        expected_visual: "Hello World  ← (unclosed LRE, PDF implied at end)",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "Double PDF — second one is a no-op",
        text: "\u{202B}عربي\u{202C}\u{202C} end",
        expected_visual: "يبرع end  ← (second PDF is extra, ignored)",
    },

    // ── Nested embeddings ─────────────────────────────────────
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "RLE inside LRE inside RTL paragraph",
        text: "عربي \u{202A}English \u{202B}عربي\u{202C} English\u{202C} عربي",
        expected_visual: "يبرع English يبرع English يبرع",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "Three levels: LRE > RLE > LRE",
        text: "\u{202A}L1 \u{202B}R2 \u{202A}L3\u{202C}\u{202C}\u{202C}",
        expected_visual: "L1 R2 L3  ← (nested but all text short)",
    },

    // ── Override + embedding combined ─────────────────────────
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "LRO inside RLE — override wins for char types",
        text: "\u{202B}\u{202D}ABC\u{202C}\u{202C}",
        expected_visual: "ABC  ← (LRO forces LTR, RLE sets embedding level)",
    },
    BidiTestCase {
        category: "L4 · Embeddings",
        description: "RLO inside LRE — characters show RTL despite LTR embedding",
        text: "\u{202A}\u{202E}Hello\u{202C}\u{202C}",
        expected_visual: "olleH  ← (RLO reverses character order)",
    },
];

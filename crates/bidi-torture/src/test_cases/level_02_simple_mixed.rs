// ═══════════════════════════════════════════════════════════════════════════
// Level 2 — Simple LTR ↔ RTL mixing
// ═══════════════════════════════════════════════════════════════════════════
// Text that contains both LTR and RTL runs separated by clear boundaries.
// The Unicode BiDi algorithm (UBA) must:
//   1. Detect the paragraph embedding level
//   2. Resolve the runs
//   3. Reorder for visual display
//
// At this level there are no explicit embedding controls — only implicit bidi.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── LTR paragraph with an RTL island ──────────────────────
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "English sentence with Arabic word embedded",
        text: "The word مرحبا means hello",
        expected_visual: "The word ابحرم means hello",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "English with Hebrew name",
        text: "Hello ישראל how are you",
        expected_visual: "Hello לארשי how are you",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Multiple Arabic words in English",
        text: "I visited القاهرة and الإسكندرية last year",
        expected_visual: "I visited ةرهاقلا and ةيردنكسلإا last year",
    },

    // ── RTL paragraph with an LTR island ──────────────────────
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Arabic sentence with English word — paragraph is RTL",
        text: "أنا أحب Rust البرمجة",
        expected_visual: "ةجمربلا Rust بحأ انأ  ← (main direction RTL, 'Rust' stays LTR inside)",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Hebrew with an English brand name",
        text: "אני אוהב GitHub מאוד",
        expected_visual: "דואמ GitHub בהוא ינא  ← (paragraph RTL, 'GitHub' embedded LTR)",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Arabic with a URL",
        text: "زوروا https://example.com للمزيد",
        expected_visual: "ديزملل https://example.com اوروز",
    },

    // ── Alternating scripts ───────────────────────────────────
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Alternating LTR-RTL-LTR-RTL",
        text: "abc عربي def عربي",
        expected_visual: "abc يبرع def يبرع",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "RTL-LTR-RTL-LTR",
        text: "عربي abc عربي def",
        expected_visual: "def يبرع abc يبرع",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Many alternations in one line",
        text: "a ب c د e و g ح i ي",
        expected_visual: "a ب c د e و g ح i ي  (each Arabic char individually reversed)",
    },

    // ── Punctuation at boundaries ─────────────────────────────
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Punctuation between LTR and RTL — period after Arabic",
        text: "The city is القاهرة.",
        expected_visual: "The city is ةرهاقلا.",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Exclamation after Hebrew word",
        text: "!שלום",
        expected_visual: "!םולש  ← (RTL paragraph, ! stays at visual left)",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Colon between English label and Arabic value",
        text: "Name: اسم",
        expected_visual: "Name: مسا",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Question mark in Arabic sentence with English",
        text: "هل تحب Rust؟",
        expected_visual: "؟Rust بحت له",
    },

    // ── Adjacent RTL scripts ──────────────────────────────────
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Arabic followed immediately by Hebrew (both RTL)",
        text: "مرحبا שלום",
        expected_visual: "םולש ابحرم  ← (both RTL, single RTL run)",
    },

    // ── Paragraph direction detection ─────────────────────────
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "First strong char is RTL → paragraph should be RTL",
        text: "عربي text عربي",
        expected_visual: "يبرع text يبرع  ← (paragraph direction RTL)",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "First strong char is LTR → paragraph should be LTR",
        text: "text عربي text",
        expected_visual: "text يبرع text  ← (paragraph direction LTR)",
    },
    BidiTestCase {
        category: "L2 · Simple Mixed",
        description: "Starts with spaces then RTL — spaces are neutral",
        text: "   مرحبا hello",
        expected_visual: "hello ابحرم     ← (paragraph RTL because first strong is Arabic)",
    },
];

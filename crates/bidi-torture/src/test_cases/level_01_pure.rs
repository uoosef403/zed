// ═══════════════════════════════════════════════════════════════════════════
// Level 1 — Pure Scripts (trivial baseline)
// ═══════════════════════════════════════════════════════════════════════════
// These are the easiest cases: text written entirely in one script direction.
// If these don't work, nothing else will.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── Pure LTR ──────────────────────────────────────────────
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Pure ASCII / Latin — should render left-to-right",
        text: "Hello, World!",
        expected_visual: "Hello, World!",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Latin with diacritics",
        text: "Ünïcödé tëxt wïth dïäcrïtïcs",
        expected_visual: "Ünïcödé tëxt wïth dïäcrïtïcs",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "CJK — left-to-right (strong LTR in UAX#9)",
        text: "日本語テキスト",
        expected_visual: "日本語テキスト",
    },

    // ── Pure RTL ──────────────────────────────────────────────
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Pure Arabic — entire string is RTL",
        text: "مرحبا بالعالم",
        expected_visual: "مرحبا بالعالم  ← (right-to-left)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Pure Hebrew — entire string is RTL",
        text: "שלום עולם",
        expected_visual: "שלום עולם  ← (right-to-left)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Arabic with tashkeel (vowel marks above/below)",
        text: "بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيمِ",
        expected_visual: "بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيمِ  ← (right-to-left, marks attached)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Persian (Farsi) — RTL, same script as Arabic",
        text: "سلام دنیا",
        expected_visual: "سلام دنیا  ← (right-to-left)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Urdu — RTL with Nastaliq style glyphs",
        text: "اردو زبان",
        expected_visual: "اردو زبان  ← (right-to-left)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Thaana (Dhivehi/Maldivian) — RTL",
        text: "ދިވެހި",
        expected_visual: "ދިވެހި  ← (right-to-left)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Syriac — RTL",
        text: "ܫܠܡܐ",
        expected_visual: "ܫܠܡ�  ← (right-to-left)",
    },

    // ── Pure LTR — Scripts that are not Latin ─────────────────
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Cyrillic — LTR",
        text: "Привет мир",
        expected_visual: "Привет мир",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Greek — LTR",
        text: "Γειά σου Κόσμε",
        expected_visual: "Γειά σου Κόσμε",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Thai — LTR (complex script but not bidi)",
        text: "สวัสดีชาวโลก",
        expected_visual: "สวัสดีชาวโลก",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Devanagari (Hindi) — LTR",
        text: "नमस्ते दुनिया",
        expected_visual: "नमस्ते दुनिया",
    },

    // ── Empty & whitespace edge cases ─────────────────────────
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Empty string — should render nothing",
        text: "",
        expected_visual: "(empty)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Single space",
        text: " ",
        expected_visual: "(one space)",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Single RTL character (Arabic Ba)",
        text: "ب",
        expected_visual: "ب",
    },
    BidiTestCase {
        category: "L1 · Pure Scripts",
        description: "Single LTR character",
        text: "A",
        expected_visual: "A",
    },
];

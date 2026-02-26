// ═══════════════════════════════════════════════════════════════════════════
// Level 5 — Directional Isolates (LRI, RLI, FSI, PDI)
// ═══════════════════════════════════════════════════════════════════════════
// Isolates (Unicode 6.3+) are the MODERN way to handle bidi.  Unlike
// embeddings, isolates prevent the isolated text from affecting the bidi
// resolution of surrounding text.  This is critical for user-provided data
// (filenames, usernames) inserted into a template.
//
//   U+2066  LRI  Left-to-Right Isolate
//   U+2067  RLI  Right-to-Left Isolate
//   U+2068  FSI  First Strong Isolate
//   U+2069  PDI  Pop Directional Isolate

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── LRI / PDI ─────────────────────────────────────────────
    BidiTestCase {
        category: "L5 · Isolates",
        description: "LRI isolates LTR text within RTL paragraph",
        text: "عربي \u{2066}English text\u{2069} عربي",
        expected_visual: "يبرع English text يبرع  ← (English isolated, doesn't affect Arabic ordering)",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "LRI with numbers — numbers inherit LRI's direction",
        text: "عربي \u{2066}Item #42\u{2069} عربي",
        expected_visual: "يبرع Item #42 يبرع",
    },

    // ── RLI / PDI ─────────────────────────────────────────────
    BidiTestCase {
        category: "L5 · Isolates",
        description: "RLI isolates RTL text within LTR paragraph",
        text: "English \u{2067}عربي\u{2069} English",
        expected_visual: "English يبرع English  ← (Arabic isolated within LTR)",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "RLI with embedded LTR inside — classic file‐in‐directory",
        text: "The path is \u{2067}مجلد/file.txt\u{2069}.",
        expected_visual: "The path is file.txt/دلجم.",
    },

    // ── FSI / PDI — auto-detect direction ─────────────────────
    BidiTestCase {
        category: "L5 · Isolates",
        description: "FSI with Arabic content → auto-detects RTL",
        text: "User said: \u{2068}مرحبا بالعالم\u{2069}!",
        expected_visual: "User said: ملاعلاب ابحرم!  ← (FSI detected Arabic → RTL isolate)",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "FSI with English content → auto-detects LTR",
        text: "المستخدم قال: \u{2068}Hello World\u{2069}!",
        expected_visual: "!Hello World :لاق مدختسملا",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "FSI with number-only content → no strong char → LTR default",
        text: "عربي \u{2068}123\u{2069} عربي",
        expected_visual: "يبرع 123 يبرع",
    },

    // ── Isolate vs Embedding — the critical difference ────────
    BidiTestCase {
        category: "L5 · Isolates",
        description: "Isolate PREVENTS spillover (compare with embedding)",
        // Without isolate, the 'W' in 'W3C' could affect resolution of nearby neutrals
        text: "Item \u{2066}W3C\u{2069} - Recipe \u{2067}وصفة\u{2069} done",
        expected_visual: "Item W3C - Recipe ةفصو done",
    },

    // ── Nested isolates ───────────────────────────────────────
    BidiTestCase {
        category: "L5 · Isolates",
        description: "RLI inside LRI — two levels of isolation",
        text: "A \u{2066}B \u{2067}عربي\u{2069} C\u{2069} D",
        expected_visual: "A B يبرع C D",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "Three levels: LRI > RLI > LRI",
        text: "\u{2066}L1 \u{2067}عربي \u{2066}L3\u{2069} عربي\u{2069} L1\u{2069}",
        expected_visual: "L1 يبرع L3 يبرع L1",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "FSI nested inside RLI",
        text: "\u{2067}عربي \u{2068}Hello\u{2069} عربي\u{2069}",
        expected_visual: "يبرع Hello يبرع",
    },

    // ── Orphan / unmatched PDI ─────────────────────────────────
    BidiTestCase {
        category: "L5 · Isolates",
        description: "Stray PDI without opener — ignored",
        text: "Hello \u{2069}World",
        expected_visual: "Hello World",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "LRI without closing PDI — auto-closed at paragraph end",
        text: "Hello \u{2066}World",
        expected_visual: "Hello World",
    },

    // ── Practical patterns ────────────────────────────────────
    BidiTestCase {
        category: "L5 · Isolates",
        description: "Username in a template — FSI is the correct approach",
        text: "Welcome, \u{2068}محمد\u{2069}! You have 3 messages.",
        expected_visual: "Welcome, دمحم! You have 3 messages.",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "File path in RTL UI — LRI protects the path",
        text: "فتح الملف \u{2066}/home/user/docs/report.pdf\u{2069}",
        expected_visual: "/home/user/docs/report.pdf فلملا حتف",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "Multiple FSI-isolated user inputs in a sentence",
        text: "\u{2068}أحمد\u{2069} sent \u{2068}Hello\u{2069} to \u{2068}דוד\u{2069}",
        expected_visual: "دمحأ sent Hello to דוד",
    },
    BidiTestCase {
        category: "L5 · Isolates",
        description: "List items with mixed isolation",
        text: "1. \u{2068}ملف\u{2069} 2. \u{2068}file\u{2069} 3. \u{2068}קובץ\u{2069}",
        expected_visual: "1. فلم 2. file 3. ץבוק",
    },
];

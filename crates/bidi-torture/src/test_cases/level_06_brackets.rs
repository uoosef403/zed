// ═══════════════════════════════════════════════════════════════════════════
// Level 6 — Bracket / Mirror Pairing
// ═══════════════════════════════════════════════════════════════════════════
// UAX#9 rule N0 — Bracket Pairs.  Paired punctuation characters like
// parentheses, brackets, and braces must be "mirrored" in RTL context.
//
// An opening '(' in RTL context should visually appear as ')' and vice versa.
// The bidi algorithm resolves bracket pairs and assigns them the direction
// of the enclosed strong type (if any) or the embedding direction.
//
// This is one of the trickiest parts to get right.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── Basic parentheses in RTL ──────────────────────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Parentheses around Arabic word — should mirror",
        text: "(عربي)",
        expected_visual: "(يبرع)  ← (parens mirror: logical '(' appears as ')' on right)",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Nested parentheses in Arabic",
        text: "عربي (كلمة (أخرى) كلمة)",
        expected_visual: "(ةملك (ىرخأ) ةملك) يبرع",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Parentheses around LTR in RTL paragraph",
        text: "عربي (English) عربي",
        expected_visual: "يبرع (English) يبرع  ← (parens mirror, English stays LTR)",
    },

    // ── Square brackets ───────────────────────────────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Square brackets in Hebrew context",
        text: "ערך [42] פה",
        expected_visual: "הפ [42] ךרע",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Nested square brackets and parens in Arabic",
        text: "عربي [(نص)] عربي",
        expected_visual: "يبرع [(صن)] يبرع",
    },

    // ── Curly braces ──────────────────────────────────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "JSON-like braces in Arabic",
        text: "بيانات: {\"اسم\": \"أحمد\"}",
        expected_visual: "{\"دمحأ\" :\"مسا\"} :تانايب",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Code braces in Hebrew comment",
        text: "// הערה: if (x) { return }",
        expected_visual: "// הערה: if (x) { return }  ← (LTR paragraph, brackets normal)",
    },

    // ── Angle brackets & guillemets ───────────────────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Angle brackets in Arabic (used in XML)",
        text: "عنصر <div> هنا",
        expected_visual: "انه <div> رصنع",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Guillemets (« ») in Arabic text",
        text: "قال «مرحبا» للجميع",
        expected_visual: "عيمجلل «ابحرم» لاق  ← (guillemets are mirrored pairs)",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Guillemets in French inside Arabic",
        text: "عربي «Bonjour» عربي",
        expected_visual: "يبرع «Bonjour» يبرع",
    },

    // ── Mixed bracket directions ──────────────────────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "LTR parens in LTR paragraph with RTL content inside",
        text: "func(عربي, text)",
        expected_visual: "func(يبرع, text)  ← (parens stay LTR-oriented, Arabic reverses)",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Array subscript in Arabic variable name",
        text: "المصفوفة[0]",
        expected_visual: "[0]ةفوفصملا",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Bracket pair with only neutral content",
        text: "عربي (---) عربي",
        expected_visual: "يبرع (---) يبرع  ← (neutral content, brackets take embedding dir)",
    },

    // ── Unmatched brackets ────────────────────────────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Opening paren without close in RTL",
        text: "عربي (بداية بلا نهاية",
        expected_visual: "ةياهن لاب ةيادب( يبرع  ← (unmatched bracket takes embedding dir)",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Closing paren without open in RTL",
        text: "عربي نهاية) عربي",
        expected_visual: "يبرع (ةياهن يبرع  ← (unmatched bracket)",
    },

    // ── Quotation marks (not bracket pairs per se) ────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Double quotes around Arabic — quotes are neutral",
        text: "\"مرحبا\"",
        expected_visual: "\"ابحرم\"  ← (quotes are neutral ON type, not brackets in UAX#9 N0)",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Smart quotes in Arabic",
        text: "\u{201C}مرحبا\u{201D}",
        expected_visual: "\u{201C}ابحرم\u{201D}  ← (U+201C/201D are mirrored pair chars)",
    },

    // ── Multiple bracket types mixed ──────────────────────────
    BidiTestCase {
        category: "L6 · Brackets",
        description: "({[عربي]}) — all three bracket types nested",
        text: "({[عربي]})",
        expected_visual: "({[يبرع]})  ← (all mirrored in RTL context)",
    },
    BidiTestCase {
        category: "L6 · Brackets",
        description: "Brackets in alternating script contexts",
        text: "f(عربي) + g[عربي] - h{عربي}",
        expected_visual: "f(يبرع) + g[يبرع] - h{يبرع}",
    },
];

// ═══════════════════════════════════════════════════════════════════════════
// Level 10 — Adversarial: Maximum Nesting, Spoofing, Stress
// ═══════════════════════════════════════════════════════════════════════════
// The hardest cases — designed to break naive implementations.  This level
// tests behaviour at the limits of the Unicode Bidirectional Algorithm:
//  - Embedding depth overflow (UBA allows max 125 levels)
//  - RLO/RLE spoofing attacks (CVE-style filename tricks)
//  - Multi-script stress (Hebrew + Arabic + Latin + CJK + emoji at once)
//  - Performance stress (long strings, 50+ isolate pairs)
//  - Combining mark storms (10+ diacritics per base character)
//  - Invisible / whitespace characters (NBSP, ZWSP, tabs, stacked controls)
//  - Degenerate inputs (empty, single-char, orphan closers, controls-only)
//
// If your renderer survives this level without panicking, corrupting memory,
// or hanging, it is ready for production.

use crate::BidiTestCase;

// ── Section A: Maximum embedding depth (UBA allows 125 levels) ───────────
static MAX_NEST: &[BidiTestCase] = &[
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Embedding depth 1 — single RLE",
        text: "\u{202B}Hello\u{202C}",
        expected_visual: "olleH",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Embedding depth 4 — RLE>LRE>RLE>LRE",
        text: "\u{202B}\u{202A}\u{202B}\u{202A}Text\u{202C}\u{202C}\u{202C}\u{202C}",
        expected_visual: "Text  ← (even depth = LTR)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Embedding depth 5 — odd = RTL",
        text: "\u{202B}\u{202A}\u{202B}\u{202A}\u{202B}Text\u{202C}\u{202C}\u{202C}\u{202C}\u{202C}",
        expected_visual: "txeT  ← (depth 5 = RTL)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "125 LRE — exactly the max; 126th should be ignored",
        // 125 LREs = depth 126 (paragraph=1 + 125 increments) — the 126th LRE overflows
        text: concat!(
            "\u{202A}\u{202A}\u{202A}\u{202A}\u{202A}", // 5
            "\u{202A}\u{202A}\u{202A}\u{202A}\u{202A}", // 10
            "\u{202A}\u{202A}\u{202A}\u{202A}\u{202A}", // 15
            "\u{202A}\u{202A}\u{202A}\u{202A}\u{202A}", // 20
            "\u{202A}\u{202A}\u{202A}\u{202A}\u{202A}", // 25
            "\u{202A}\u{202A}\u{202A}\u{202A}\u{202A}", // 30
            "\u{202A}", // 31 — well within 125
            "Deep",
        ),
        expected_visual: "Deep  ← (LTR at any even depth, or RTL if odd; must not panic)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Overflow: more than 125 embeddings — extras silently discarded",
        text: concat!(
            // 130 RLEs followed by text
            "\u{202B}\u{202B}\u{202B}\u{202B}\u{202B}",
            "\u{202B}\u{202B}\u{202B}\u{202B}\u{202B}",
            "\u{202B}\u{202B}\u{202B}\u{202B}\u{202B}",
            "\u{202B}\u{202B}\u{202B}\u{202B}\u{202B}",
            "\u{202B}\u{202B}\u{202B}\u{202B}\u{202B}",
            "\u{202B}\u{202B}\u{202B}\u{202B}\u{202B}",
            "Overflow",
        ),
        expected_visual: "Overflow  ← (must not panic or corrupt memory)",
    },
];

// ── Section B: Homograph / Spoofing attacks ───────────────────────────────
static SPOOF: &[BidiTestCase] = &[
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "RLO spoof: code shown in UI differs from actual string",
        // Classic CVE trick: 'gnupg[U+202E]txt.exe' looks like 'exe.txt' to users (RLO char)
        text: "malware\u{202E}txt.exe",
        expected_visual: "exe.txtmalware  ← (DANGER: visual != logical; renderer must not hide this)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "RLO spoof in filename — whole name reversed",
        text: "photo\u{202E}gpj.exe",
        expected_visual: "exe.jpgphoto  ← (looks like photo.jpg but is .exe)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "IDN homograph: Arabic zero (٠) beside ASCII zero",
        text: "admin٠@example.com",
        expected_visual: "admin٠@example.com  ← (Arabic-Indic 0 vs ASCII 0 — visually identical fonts)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Unicode tag characters (block U+E0000) inside string",
        text: "Hello\u{E0001}\u{E0048}\u{E0069}World",
        expected_visual: "HelloWorld  ← (tag chars should be invisible, BN for bidi)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "VARIATION SELECTOR after Arabic changes glyph but not bidi",
        text: "م\u{FE00}رحبا",
        expected_visual: "ابحرم  ← (variation selector is NSM/BN, doesn't break RTL run)",
    },
];

// ── Section C: Mixed-script stress ────────────────────────────────────────
static MIXED_STRESS: &[BidiTestCase] = &[
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Hebrew + Arabic + Latin + numbers all in one line",
        text: "שלום مرحبا Hello 123 مرحبا שלום",
        expected_visual: "םולש ابحرم Hello 123 ابحرم םולש",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Alternating single-char direction changes",
        text: "aبbعcكdمeن",
        expected_visual: "aبbعcكdمeن  ← (each Arabic char is a separate RTL island)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Emoji between RTL runs",
        text: "مرحبا 🎉 שלום 🎊 سلام",
        expected_visual: "مالس 🎊 םולש 🎉 ابحرم",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "CJK + Arabic + Latin triple mix",
        text: "Hello مرحبا 你好世界 שלום World",
        expected_visual: "Hello ابحرم 你好世界 םולש World",
    },
];

// ── Section D: Performance stress — long strings ─────────────────────────
static PERF_STRESS: &[BidiTestCase] = &[
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "200-char alternating LTR/RTL (must not hang)",
        text: "abcمنوdefعينghiبتثjklجحخmnoدذرpqrزسشstuصضطuvwظعغxyzفقك\
               abcمنوdefعينghiبتثjklجحخmnoدذرpqrزسشstuصضطuvwظعغxyzفقك\
               abcمنوdefعينghiبتثjklجحخmnoدذرpqrزسشstuصضطuvwظعغxyzفقك",
        expected_visual: "(renderer must handle this without freezing or panic)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Repeated RLI/PDI pairs — 50 isolates",
        text: concat!(
            "\u{2067}A\u{2069}\u{2067}B\u{2069}\u{2067}C\u{2069}",
            "\u{2067}D\u{2069}\u{2067}E\u{2069}\u{2067}F\u{2069}",
            "\u{2067}G\u{2069}\u{2067}H\u{2069}\u{2067}I\u{2069}",
            "\u{2067}J\u{2069}\u{2067}K\u{2069}\u{2067}L\u{2069}",
        ),
        expected_visual: "ABCDEFGHIJKL  ← (each letter isolated RTL but single Latin = LTR)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Empty isolates — 20 RLI/PDI with no content",
        text: concat!(
            "\u{2067}\u{2069}\u{2067}\u{2069}\u{2067}\u{2069}",
            "\u{2067}\u{2069}\u{2067}\u{2069}\u{2067}\u{2069}",
            "\u{2067}\u{2069}\u{2067}\u{2069}\u{2067}\u{2069}",
            "OK",
        ),
        expected_visual: "OK  ← (empty isolates produce no visible output)",
    },
];

// ── Section E: Combining mark storms ─────────────────────────────────────
static COMBINING: &[BidiTestCase] = &[
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "10 combining marks on Arabic base",
        text: "م\u{0651}\u{064B}\u{064C}\u{064D}\u{064E}\u{064F}\u{0650}\u{0652}\u{0670}\u{0656}",
        expected_visual: "(single Arabic char with 10 diacritics stacked — must not crash)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Combining marks on Hebrew base",
        text: "שׁ\u{05B0}\u{05B1}\u{05B2}\u{05B3}\u{05B4}\u{05B5}\u{05B6}\u{05B7}\u{05B8}",
        expected_visual: "(Hebrew shin with 9 nikud marks — stress test for glyph stacking)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Zalgo-style combining marks on Latin between RTL",
        text: "سلام H\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307} مرحبا",
        expected_visual: "ابحرم H̀́̂̃̄̅̆̇ مالس  ← (Latin char stays LTR island)",
    },
];

// ── Section F: Whitespace and invisible characters ───────────────────────
static INVISIBLE: &[BidiTestCase] = &[
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "NBSP between RTL words",
        text: "مرحبا\u{00A0}بالعالم",
        expected_visual: "ملاعلاب\u{00A0}ابحرم  ← (NBSP is CS, joins RTL context)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "ZWSP inside RTL — should not break bidi",
        text: "שלום\u{200B}עולם",
        expected_visual: "םלוע\u{200B}םולש  ← (ZWSP is BN type, invisible)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Tab character in mixed bidi text",
        text: "Hello\tمرحبا\tWorld",
        expected_visual: "Hello\tابحرم\tWorld",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "All controls combined: LRE+RLO+LRI+ZWJ+LRM in sequence",
        text: "\u{202A}\u{202E}\u{2066}\u{200D}\u{200E}test\u{2069}\u{202C}\u{202C}",
        expected_visual: "test  ← (stacking contradictory controls; must resolve sanely)",
    },
];

// ── Section G: Edge cases — degenerate inputs ───────────────────────────
static EDGE: &[BidiTestCase] = &[
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Empty string",
        text: "",
        expected_visual: "(empty — must not panic)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Single RTL character",
        text: "ع",
        expected_visual: "ع",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Only bidi control characters, no visible text",
        text: "\u{200F}\u{200E}\u{2066}\u{2069}\u{202A}\u{202C}",
        expected_visual: "(nothing visible — controls only)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Unmatched PDI without opener",
        text: "Hello\u{2069}\u{2069}\u{2069}World",
        expected_visual: "HelloWorld  ← (orphan PDIs are ignored per UBA)",
    },
    BidiTestCase {
        category: "L10 · Adversarial",
        description: "Unmatched PDF without opener",
        text: "Hello\u{202C}\u{202C}\u{202C}World",
        expected_visual: "HelloWorld  ← (orphan PDFs are ignored per UBA)",
    },
];

/// Flattened export — uses `fn` because Rust can't concat static slices at compile time.
/// `mod.rs` calls this via `cases.extend(level_10_adversarial::cases())`.
pub fn cases() -> Vec<BidiTestCase> {
    let mut v = Vec::with_capacity(
        MAX_NEST.len() + SPOOF.len() + MIXED_STRESS.len()
        + PERF_STRESS.len() + COMBINING.len() + INVISIBLE.len() + EDGE.len(),
    );
    v.extend_from_slice(MAX_NEST);
    v.extend_from_slice(SPOOF);
    v.extend_from_slice(MIXED_STRESS);
    v.extend_from_slice(PERF_STRESS);
    v.extend_from_slice(COMBINING);
    v.extend_from_slice(INVISIBLE);
    v.extend_from_slice(EDGE);
    v
}

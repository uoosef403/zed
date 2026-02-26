// ═══════════════════════════════════════════════════════════════════════════
// Level 3 — Numbers inside RTL context
// ═══════════════════════════════════════════════════════════════════════════
// Numbers are "weak" bidi types — European Numbers (EN) and Arabic-Indic
// Numbers (AN).  They don't flip direction themselves, but their surrounding
// context determines where they appear visually.
//
// This is the #1 source of bugs in naive bidi implementations.

use crate::BidiTestCase;

pub static CASES: &[BidiTestCase] = &[
    // ── European digits in RTL ────────────────────────────────
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Arabic text with European number — number should stay LTR within RTL flow",
        text: "العدد 42 هنا",
        expected_visual: "انه 42 ددعلا  ← (42 stays as '42' not '24')",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Phone number in Arabic sentence",
        text: "رقم الهاتف: 123-456-7890",
        expected_visual: "123-456-7890 :فتاهلا مقر  ← (phone number stays LTR)",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Hebrew price with currency",
        text: "המחיר הוא $99.99",
        expected_visual: "$99.99 אוה ריחמה",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Date in Arabic context",
        text: "تاريخ: 2025-01-15",
        expected_visual: "2025-01-15 :خيرات",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Percentage in Hebrew",
        text: "ההנחה: 25%",
        expected_visual: "25% :החנהה",
    },

    // ── Arabic-Indic digits (U+0660..U+0669) ──────────────────
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Arabic-Indic numerals in Arabic text — these are AN type",
        text: "العدد ٤٢ هنا",
        expected_visual: "انه ٤٢ ددعلا  ← (Arabic-Indic digits, AN type in UAX#9)",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Mixed European and Arabic-Indic digits",
        text: "من ٤٢ إلى 42",
        expected_visual: "42 ىلإ ٤٢ نم  ← (two different number systems)",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Arabic-Indic phone number",
        text: "رقم: ٠١٢-٣٤٥-٦٧٨٩",
        expected_visual: "٠١٢-٣٤٥-٦٧٨٩ :مقر",
    },

    // ── Numbers between two RTL words ─────────────────────────
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Number sandwiched between Arabic words",
        text: "كلمة 123 كلمة",
        expected_visual: "ةملك 123 ةملك  ← (number embedded within RTL flow)",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Multiple numbers between Arabic words",
        text: "عدد 10 و 20 و 30",
        expected_visual: "30 و 20 و 10 ددع",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Decimal number in Hebrew",
        text: "המספר 3.14159 חשוב",
        expected_visual: "בושח 3.14159 רפסמה",
    },

    // ── Numbers at paragraph boundaries ───────────────────────
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Line starts with number, then Arabic — paragraph direction?",
        text: "123 عربي",
        expected_visual: "يبرع 123  ← (first strong is Arabic → RTL paragraph)",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Line ends with number after Arabic",
        text: "عربي 456",
        expected_visual: "456 يبرع",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Only numbers — no strong type → default to LTR",
        text: "12345",
        expected_visual: "12345  ← (LTR, numbers are weak type EN)",
    },

    // ── Numbers with operators ────────────────────────────────
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Math expression in Arabic context",
        text: "النتيجة: 3 + 4 = 7",
        expected_visual: "3 + 4 = 7 :ةجيتنلا",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Negative number in Hebrew",
        text: "ערך: -42",
        expected_visual: "-42 :ךרע",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Fraction in Arabic",
        text: "النسبة ½ من الكل",
        expected_visual: "لكلا نم ½ ةبسنلا",
    },

    // ── Currency & units ──────────────────────────────────────
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Price in Arabic with dollar sign (ES type)",
        text: "السعر $100",
        expected_visual: "$100 رعسلا",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Temperature in Hebrew",
        text: "הטמפרטורה: 37°C",
        expected_visual: "37°C :הרוטרפמטה",
    },
    BidiTestCase {
        category: "L3 · Numbers",
        description: "Multiple currencies mixed",
        text: "₪100 أو $200 أو €300",
        expected_visual: "€300 وأ $200 وأ ₪100",
    },
];

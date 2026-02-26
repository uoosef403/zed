# 04 — Implementation Phases

## Phase 1: Paragraph Direction Detection
**Goal**: Detect if text starts RTL and tell DirectWrite.
**Tests passing after**: Level 1 (pure scripts)

### Tasks:
1. Add `fn detect_paragraph_direction(text: &str) -> bool` to
   `direct_write.rs` — scan for first strong char (L, R, AL)
   using Unicode property tables
2. In `layout_line()`, after creating `IDWriteTextLayout`, call:
   `layout.SetReadingDirection(DWRITE_READING_DIRECTION_RIGHT_TO_LEFT)`
   when paragraph is RTL
3. Run bidi-torture, verify Level 1 pure RTL cases render correctly

### Key code location:
`crates/gpui/src/platform/windows/direct_write.rs` line ~575
(the `layout_line` method)

### Acceptance:
- Pure Arabic text renders right-to-left
- Pure English text unchanged
- `cargo run -p bidi-torture` Level 1 cases look correct

---

## Phase 2: Mixed Text Reordering  
**Goal**: LTR+RTL mixed text renders in correct visual order.
**Tests passing after**: Level 2 (simple mixed)

### Tasks:
1. Verify that `DrawGlyphRun` callback receives runs in
   visual order from DirectWrite (it should automatically)
2. Read `glyphrun.bidiLevel` in the callback — log it to confirm
   DirectWrite is assigning correct levels
3. If glyph positions are wrong, fix advance accumulation in
   `DrawGlyphRun` for RTL runs

### Acceptance:
- "Hello أهلاً World" renders with Arabic reversed in middle
- Level 2 test cases look correct visually

---

## Phase 3: Numbers and Weak Types
**Goal**: Numbers in RTL context stay LTR.
**Tests passing after**: Level 3 (numbers)

### Tasks:
1. Should work automatically if Phase 1-2 are correct
2. Verify European numbers stay LTR within RTL paragraph
3. Verify Arabic-Indic digits (AN type) handled correctly
4. Fix any edge cases with currency symbols, operators

### Acceptance:
- "السعر 42 دولار" shows 42 (not 24) in correct position

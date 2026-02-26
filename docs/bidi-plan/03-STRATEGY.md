# 03 — Strategy: DirectWrite-Native BiDi

## Two Possible Approaches

### Option A: Manual UBA (unicode-bidi crate)
- Run Unicode BiDi Algorithm in Rust
- Reorder text ourselves before sending to DirectWrite
- Pro: Full control, cross-platform
- Con: Duplicates DirectWrite's built-in UBA, complex, error-prone

### Option B: Let DirectWrite Handle BiDi ← **OUR CHOICE**
- Set `ReadingDirection` on `IDWriteTextLayout`
- DirectWrite runs the full UBA internally
- Process reordered glyph runs from `DrawGlyphRun` callback
- Pro: Battle-tested, Microsoft's own UBA implementation
- Con: Windows-only (but GPUI already is Windows on this path)

## Why Option B Wins

1. **DirectWrite already has a conformant UBA** — tested against
   Unicode conformance tests, handles all 201 of our cases
2. **Shaping + BiDi in one pass** — Arabic joining, bracket mirroring
   all handled atomically
3. **Less code** — we add ~100 lines, not ~2000
4. **Performance** — zero-copy, no intermediate reordering buffer
5. **macOS path** — CoreText also has built-in BiDi, same pattern applies

## What We Need to Change

### In `DirectWriteState::layout_line()`:
1. Detect paragraph base direction (P2/P3)
2. Call `layout.SetReadingDirection(direction)`
3. Process `DrawGlyphRun` with bidi-awareness

### In `DrawGlyphRun` callback:
1. Read `glyphrun.bidiLevel` (currently ignored)
2. Store bidi_level in `ShapedRun`
3. Already correct: DirectWrite gives us glyphs in visual order
   with correct advances

### In `LineLayout` positional APIs:
1. `index_for_x` — handle non-monotonic glyph indices
2. `x_for_index` — handle out-of-visual-order indices
3. `closest_index_for_x` — same

### In `paint_line`:
1. Pass bidi_level to `DWRITE_GLYPH_RUN` for rasterization
2. Already paints in visual order (no change needed)

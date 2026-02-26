# 04b — Implementation Phases (continued)

## Phase 4: Explicit Embeddings & Isolates
**Goal**: LRE/RLE/LRO/RLO/PDF and LRI/RLI/FSI/PDI work.
**Tests passing after**: Levels 4, 5

### Tasks:
1. Should work automatically — DirectWrite handles these controls
2. Test that orphan/unmatched controls don't crash
3. Verify RLO override (force "Hello" → "olleH") works
4. Verify isolates prevent spillover

### Acceptance:
- Level 4 and Level 5 test cases render correctly
- No panics on malformed control sequences

---

## Phase 5: Brackets, Weak/Neutral Resolution
**Goal**: Bracket mirroring and weak/neutral type context work.
**Tests passing after**: Levels 6, 7

### Tasks:
1. Verify bracket mirroring (L4 rule) — DirectWrite does this
2. Check parentheses mirror in RTL context: `(text)` → `(text)`
   but visually the open-paren appears on the right
3. Test neutral resolution — spaces between same-type resolve
4. Fix any painting issues with mirrored glyphs

### Acceptance:
- "(مرحبا)" renders with correct bracket directions
- Level 6 and 7 cases pass

---

## Phase 6: Line Breaking in BiDi Context
**Goal**: Text wrapping works correctly with mixed directions.
**Tests passing after**: Level 8

### Tasks:
1. Review `compute_wrap_boundaries()` in `line_layout.rs`
2. Ensure wrap boundaries respect bidi run boundaries
3. Implement L1 rule: trailing whitespace reverts to paragraph level
4. Each visual line after wrap must be independently correct

### Acceptance:
- Long mixed text wraps correctly
- Trailing whitespace in RTL paragraph aligns correctly

---

## Phase 7: Marks, ZWJ, Adversarial
**Goal**: Invisible controls, marks, and edge cases.
**Tests passing after**: Levels 9, 10

### Tasks:
1. LRM/RLM/ALM marks — should work via DirectWrite
2. ZWJ sequences for emoji — verify shaping preserved
3. Max nesting depth (125+) — verify no panic
4. Spoofing patterns — verify correct rendering (not security)
5. Degenerate inputs — empty, control-only strings

### Acceptance:
- All 201 test cases render correctly
- No panics or crashes on any input

---

## Phase 8: Cursor & Hit-Testing (Post-Visual)
**Goal**: Clicking and arrow keys work in bidi text.
**Tests passing after**: N/A (interaction quality)

### Tasks:
1. Fix `index_for_x()` — must handle non-monotonic glyph order
2. Fix `x_for_index()` — must handle bidirectional index mapping
3. Fix `closest_index_for_x()` — for arrow key navigation
4. Add bidi_level to ShapedRun for cursor direction awareness
5. See `06-CURSOR-SELECTION.md` for full details

### Acceptance:
- Clicking in bidi text places cursor at correct position
- Arrow keys move cursor in logical order

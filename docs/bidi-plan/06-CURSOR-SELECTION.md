# 06 — Cursor & Selection in BiDi Context

## The Problem

In LTR-only text, glyph indices are monotonically increasing left to right.
In bidi text, they are NOT. Example:

```
Visual:    H  e  l  l  o     ب  ا  ه  ذ     W  o  r  l  d
Index:     0  1  2  3  4  5  9  8  7  6     10 11 12 13 14
```

The Arabic section has indices going RIGHT to LEFT (9,8,7,6).

## What Breaks

### `index_for_x(x)` — "which char is at this x?"
Current code scans runs left-to-right, returns first glyph where
`glyph.position.x <= x`. This works only if indices are monotonic.

**Fix**: Build an interval map: for each glyph, store its x-range.
Binary search for the x coordinate. The glyph's `index` gives you
the logical character position.

### `x_for_index(index)` — "where is this char on screen?"
Current code scans runs and returns `glyph.position.x` for the
first glyph with `glyph.index >= index`. With bidi, a given logical
index may appear anywhere visually.

**Fix**: Build a lookup from logical index → glyph position.
Could be a sorted Vec or a HashMap<usize, Pixels>.

### `closest_index_for_x(x)` — "nearest boundary"
Same issue as `index_for_x` but needs to compare distances.
**Fix**: Same interval map approach, pick nearest boundary.

## Arrow Key Navigation

### Logical vs Visual Movement
- **Logical**: left arrow goes to previous char in memory
- **Visual**: left arrow goes to visually-left char

Most editors (VS Code, JetBrains) use **visual** movement.
This means at an LTR→RTL boundary, pressing left might jump.

### Cursor Affinity
At a bidi boundary, the same logical position has TWO visual
positions (end of LTR run, or start of RTL run). You need
"cursor affinity" — a flag saying which visual side the cursor is on.

```rust
pub struct CursorPosition {
    pub index: usize,        // logical byte index
    pub affinity: Affinity,  // which visual side
}

pub enum Affinity {
    Before,  // cursor is before this index visually  
    After,   // cursor is after this index visually
}
```

## Selection Painting

Selections in bidi text may be discontiguous visually.
Selecting logical indices 3–9 in the example above would
highlight two separate visual rectangles.

This is a Phase 8 concern — get rendering right first.

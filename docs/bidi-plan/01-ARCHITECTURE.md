# 01 — GPUI Text Rendering Pipeline Architecture

## Current Pipeline (No BiDi)

The text rendering pipeline in GPUI follows this flow **from string to pixels**:

```
 User code                 TextSystem                  Platform (DirectWrite)
 ─────────                 ──────────                  ──────────────────────
 div().child("مرحبا")
       │
       ▼
 TextElement::paint()
       │
       ▼
 TextLayout::layout()
       │  creates TextRun[] (font, color, len)
       ▼
 TextSystem::shape_text()        ── splits by '\n' ──►  per-line processing
       │                                                       │
       │                                                       ▼
       │                                              TextSystem::layout_line()
       │                                                       │
       │                                                       ▼
       │                                              LineLayoutCache::layout_line()
       │                                                       │
       │                                                       ▼
       │                                              PlatformTextSystem::layout_line()
       │                                                       │
       │                                                       ▼
       │                                              DirectWriteState::layout_line()
       │                                                       │
       │                                              ┌────────┴────────┐
       │                                              │  1. Create IDWriteTextLayout
       │                                              │  2. Set font runs
       │                                              │  3. Call Draw() with custom renderer
       │                                              │  4. DrawGlyphRun callback fires
       │                                              │     per font run
       │                                              │  5. Accumulate ShapedGlyph[]
       │                                              └────────┬────────┘
       │                                                       │
       │                                                       ▼
       │                                              LineLayout {
       │                                                runs: Vec<ShapedRun>,
       │                                                width, ascent, descent
       │                                              }
       │                                                       │
       ▼                                                       ▼
 WrappedLineLayout                                    compute_wrap_boundaries()
       │
       ▼
 paint_line()  ──► iterates glyphs, computes x positions, calls window.paint_glyph()
```

## Key Files and Their Roles

### Core Text System
- **`crates/gpui/src/text_system.rs`**
  - `TextSystem` struct — the public-facing text API
  - `shape_line()` — shapes a single line, returns `ShapedLine`
  - `shape_text()` — shapes multi-line text, returns `Vec<WrappedLine>`
  - `layout_line()` — converts `TextRun[]` into `FontRun[]`, delegates to cache

### Line Layout
- **`crates/gpui/src/text_system/line_layout.rs`**
  - `LineLayout` — the core data structure: `{ runs: Vec<ShapedRun>, width, ascent, descent }`
  - `ShapedRun` — `{ font_id, glyphs: Vec<ShapedGlyph> }`
  - `ShapedGlyph` — `{ id, position: Point<Pixels>, index: usize, is_emoji }`
  - `index_for_x()` — hit-testing: x coordinate → character index
  - `x_for_index()` — caret positioning: character index → x coordinate
  - `closest_index_for_x()` — nearest boundary for arrow keys
  - `WrappedLineLayout` — adds `wrap_boundaries` to `LineLayout`
  - `LineLayoutCache` — two-frame cache keyed by (text, font_size, runs)

### Line Painting
- **`crates/gpui/src/text_system/line.rs`**
  - `ShapedLine` — wraps `LineLayout` + `decoration_runs` + text
  - `WrappedLine` — wraps `WrappedLineLayout` + decorations
  - `paint_line()` — the actual painting function, iterates glyphs left-to-right
  - `paint_line_background()` — paints background colors

### Text Element
- **`crates/gpui/src/elements/text.rs`**
  - `TextElement` — the GPUI element that renders text
  - `TextLayout` — manages measurement → prepaint → paint lifecycle
  - `position_for_index()` — used for cursor/caret placement

### Platform (Windows DirectWrite)
- **`crates/gpui/src/platform/windows/direct_write.rs`**
  - `DirectWriteTextSystem` — implements `PlatformTextSystem`
  - `DirectWriteState::layout_line()` — THE critical function:
    1. Creates `IDWriteTextLayout` with `CreateTextLayout()`
    2. **Never calls `SetReadingDirection()`** ← ROOT CAUSE
    3. So DirectWrite defaults to `DWRITE_READING_DIRECTION_LEFT_TO_RIGHT`
    4. Calls `text_layout.Draw()` with custom `TextRenderer`
    5. `DrawGlyphRun` callback accumulates `ShapedGlyph` data
    6. **Ignores the `bidiLevel` in** `DWRITE_GLYPH_RUN` ← SECOND PROBLEM
  - `RendererContext` — holds mutable state during Draw callback
  - `TextRenderer` / `DrawGlyphRun` — processes glyph runs from DirectWrite
  - All `DWRITE_GLYPH_RUN` constructions hard-code `bidiLevel: 0`

## What's Wrong: The Three Root Causes

### 1. No Reading Direction Signal
```rust
// In DirectWriteState::layout_line():
let layout = self.components.factory.CreateTextLayout(
    &text_wide,
    &format,
    f32::INFINITY,
    f32::INFINITY,
)?;
// ← Missing: layout.SetReadingDirection(...)
// DirectWrite DEFAULTS to LTR
```

### 2. BiDi Level Ignored in Glyph Run Processing
```rust
// In DrawGlyphRun callback:
// The `glyphrun` has a `bidiLevel` field that DirectWrite sets
// but the current code NEVER reads it.
// For RTL runs (odd bidiLevel), glyph advances need reversed processing.
```

### 3. All Positional APIs Assume LTR
```rust
// index_for_x, x_for_index, closest_index_for_x all assume
// glyphs are in left-to-right order. With bidi, glyphs within
// an RTL run are right-to-left.
```

## What DirectWrite Already Does for Us

DirectWrite's `IDWriteTextLayout` with proper reading direction will:
- ✅ Run the full Unicode BiDi Algorithm (UAX #9)
- ✅ Resolve embedding levels
- ✅ Handle explicit controls (LRE, RLE, LRO, RLO, PDF)
- ✅ Handle isolates (LRI, RLI, FSI, PDI)
- ✅ Resolve weak and neutral types
- ✅ Reorder glyph runs
- ✅ Handle bracket pairing (N0)
- ✅ Apply glyph mirroring (L4)
- ✅ Shape text correctly (Arabic joining, etc.)

What we need to do:
- ⬜ Detect paragraph base direction and pass it to DirectWrite
- ⬜ Handle the bidiLevel in DrawGlyphRun callback correctly
- ⬜ Fix glyph position accumulation for RTL runs
- ⬜ Fix hit-testing (index_for_x, x_for_index) for bidi text
- ⬜ Fix cursor/caret movement in bidi context
- ⬜ Fix line wrapping in bidi context
- ⬜ Add BiDi direction info to LineLayout/ShapedRun
- ⬜ Pass bidiLevel to DWRITE_GLYPH_RUN for correct glyph rasterization

## The Data Flow With BiDi (Target State)

```
 "Hello أهلاً World"
       │
       ▼
 Paragraph Direction Detection
 (first strong character → R if Arabic/Hebrew, else L)
       │
       ▼
 DirectWriteState::layout_line(text, font_size, runs, paragraph_dir)
       │
       ▼
 IDWriteTextLayout with SetReadingDirection(RTL or LTR)
       │
       ▼
 Draw() → DrawGlyphRun called per VISUAL run:
   Run 1: "Hello "     bidiLevel=0 (LTR) → glyphs left-to-right
   Run 2: "ًلاهأ"      bidiLevel=1 (RTL) → glyphs right-to-left  
   Run 3: " World"     bidiLevel=0 (LTR) → glyphs left-to-right
       │
       ▼
 ShapedRun now carries bidi_level
 Glyph positions correctly account for RTL advance direction
       │
       ▼
 LineLayout with bidi-aware positional queries
```

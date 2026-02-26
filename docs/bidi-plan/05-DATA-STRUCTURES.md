# 05 — New Data Structures

## Changes to Existing Types

### `ShapedRun` (line_layout.rs)
```rust
// BEFORE:
pub struct ShapedRun {
    pub font_id: FontId,
    pub glyphs: Vec<ShapedGlyph>,
}

// AFTER — add bidi_level:
pub struct ShapedRun {
    pub font_id: FontId,
    pub glyphs: Vec<ShapedGlyph>,
    pub bidi_level: u8,  // NEW: 0=LTR, odd=RTL
}
```

### `LineLayout` (line_layout.rs)
```rust
// AFTER — add paragraph direction:
pub struct LineLayout {
    pub font_size: Pixels,
    pub width: Pixels,
    pub ascent: Pixels,
    pub descent: Pixels,
    pub runs: Vec<ShapedRun>,
    pub len: usize,
    pub paragraph_level: u8,  // NEW: 0 or 1
}
```

### `PlatformTextSystem` trait (platform.rs)
```rust
// BEFORE:
fn layout_line(&self, text: &str, font_size: Pixels, 
               runs: &[FontRun]) -> LineLayout;

// AFTER — no signature change needed!
// Paragraph direction is auto-detected from text content.
// The platform impl detects it internally.
```

### `RendererContext` (direct_write.rs)
```rust
// AFTER — track bidi level per run:
struct RendererContext<'t, 'a, 'b> {
    text_system: &'t mut DirectWriteState,
    index_converter: StringIndexConverter<'a>,
    runs: &'b mut Vec<ShapedRun>,
    width: f32,
    // No new fields needed — bidi_level comes from
    // DWRITE_GLYPH_RUN in DrawGlyphRun callback
}
```

## New Helper Function

```rust
/// Detect paragraph base direction per UAX #9 rules P2/P3.
/// Returns 0 for LTR, 1 for RTL.
fn detect_paragraph_level(text: &str) -> u8 {
    for ch in text.chars() {
        match unicode_bidi_class(ch) {
            L => return 0,
            R | AL => return 1,
            _ => continue, // skip weak/neutral
        }
    }
    0 // default LTR
}
```

For `unicode_bidi_class()`, you have two options:
1. Use `unicode-bidi` crate (already in Cargo.toml)
2. Use ICU Unicode property lookup
3. Or a minimal inline table for just P2/P3 detection

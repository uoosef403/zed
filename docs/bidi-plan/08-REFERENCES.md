# 08 — References

## Specifications
- **UAX #9** (Unicode BiDi Algorithm):
  https://unicode.org/reports/tr9/
- **Unicode BiDi conformance tests**:
  https://unicode.org/Public/UCD/latest/ucd/BidiTest.txt
- **UAX #50** (Vertical text — future extension):
  https://unicode.org/reports/tr50/

## DirectWrite APIs
- `IDWriteTextLayout::SetReadingDirection`:
  https://learn.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwritetextlayout-setreadingdirection
- `DWRITE_READING_DIRECTION` enum:
  `DWRITE_READING_DIRECTION_LEFT_TO_RIGHT = 0`
  `DWRITE_READING_DIRECTION_RIGHT_TO_LEFT = 1`
- `DWRITE_GLYPH_RUN::bidiLevel`:
  https://learn.microsoft.com/en-us/windows/win32/api/dwrite/ns-dwrite-dwrite_glyph_run

## Rust Crates
- `unicode-bidi` (v0.3) — Full UBA implementation in Rust:
  https://crates.io/crates/unicode-bidi
- `unicode-bidi-mirroring` — Bracket mirroring data:
  https://crates.io/crates/unicode-bidi-mirroring

## Implementations to Study
- **Firefox/Servo** `unicode-bidi` crate (by Mozilla):
  https://github.com/servo/unicode-bidi
- **Chromium BiDi** in Blink layout:
  `third_party/blink/renderer/platform/text/bidi_resolver.h`
- **HarfBuzz** built-in bidi (hb_buffer_set_direction):
  https://harfbuzz.github.io/
- **ICU4C** ubidi.h — industry reference:
  https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/ubidi_8h.html

## Papers & Articles
- "Unicode Bidirectional Algorithm Basics" by Aharon Lanin (Google):
  Explains visual vs logical order with diagrams
- "Implementing BiDi: Lessons Learned" (JetBrains blog):
  Practical cursor/selection issues
- W3C "Requirements for Japanese/Arabic Layout":
  https://www.w3.org/TR/alreq/

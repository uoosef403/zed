# 02b — UBA Theory (continued)

## Phase 4: Neutral Type Resolution (Rules N0–N2)

- **N0**: Bracket Pair Algorithm — paired brackets get direction from
  enclosed strong type, or from embedding direction
- **N1**: Neutrals between same-direction → that direction
- **N2**: Remaining neutrals → embedding direction

## Phase 5: Implicit Levels (Rules I1, I2)

```
I1: For chars with even level:
    if type = R  → level += 1
    if type = AN → level += 2
    if type = EN → level += 2

I2: For chars with odd level:
    if type = L  → level += 1
    if type = EN → level += 1
    if type = AN → level += 1
```

## Phase 6: Reordering (Rules L1–L4)

**L1**: Reset trailing whitespace/isolate to paragraph level.

**L2**: The reversal algorithm (most important formula):
```
max_level = max of all levels
for level in (max_level down to 1):
    reverse every contiguous run where char_level >= level
```

**L4**: Mirror characters for RTL (e.g., `(` ↔ `)`, `[` ↔ `]`).

## Why This Matters for GPUI

DirectWrite implements ALL of the above internally via
`IDWriteTextLayout`. We just need to:
1. Tell it the correct paragraph direction
2. Correctly process the reordered glyph runs it gives back
3. Preserve bidi level info for cursor/hit-testing

The `unicode-bidi` Rust crate also implements the full UBA
and can be used for paragraph direction detection independently.

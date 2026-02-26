# 02 — Unicode BiDi Algorithm (UAX #9) Theory

## Core Concept: Embedding Levels

Every character gets an **embedding level** (integer 0–125).
- **Even levels** (0, 2, 4…) = **LTR**
- **Odd levels** (1, 3, 5…) = **RTL**

The higher the level, the deeper the nesting.

## Phase 1: Paragraph Level (Rules P2, P3)

Scan for the **first strong character** (L, AL, R):
- If L → paragraph level = 0 (LTR)
- If R or AL → paragraph level = 1 (RTL)
- If none found → default = 0 (LTR)

```
Formula: paragraph_level = if first_strong ∈ {R, AL} then 1 else 0
```

## Phase 2: Explicit Levels (Rules X1–X8)

Process control characters that explicitly set embedding levels:

| Control | Effect | Rule |
|---------|--------|------|
| LRE (U+202A) | Push level to next even | X3 |
| RLE (U+202B) | Push level to next odd | X2 |
| LRO (U+202D) | Push level + override LTR | X5 |
| RLO (U+202E) | Push level + override RTL | X4 |
| PDF (U+202C) | Pop from stack | X7 |
| LRI (U+2066) | Push isolate, LTR | X5a |
| RLI (U+2067) | Push isolate, RTL | X5b |
| FSI (U+2068) | Push isolate, auto-detect | X5c |
| PDI (U+2069) | Pop isolate | X6a |

**Key formulas:**
```
next_even(level) = (level + 2) & !1    // round up to even
next_odd(level)  = (level + 1) | 1     // round up to odd
max_depth = 125                         // >125 → ignore
```

## Phase 3: Weak Type Resolution (Rules W1–W7)

These rules reclassify "weak" types based on context:

- **W1**: NSM (combining marks) → type of preceding char
- **W2**: EN after AL → AN (European digits become Arabic-Indic in context)
- **W3**: AL → R (Arabic Letter treated as R for remaining rules)
- **W4**: ES between ENs → EN; CS between same numeric type → that type
- **W5**: ET adjacent to EN → EN
- **W6**: Remaining ES, ET, CS → ON (become neutral)
- **W7**: EN after L context → L

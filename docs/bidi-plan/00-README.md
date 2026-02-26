# GPUI Bidirectional (BiDi) Text Support — Implementation Plan

> **Author**: uoosef403  
> **Branch**: `bidi-support`  
> **Date**: 2026-02-26  
> **Goal**: Add full Unicode BiDi Algorithm (UAX #9) support to the GPUI rendering framework,
> passing all 201 torture-test cases in `crates/bidi-torture`.

## Documents in This Folder

| File | Purpose |
|------|---------|
| [01-ARCHITECTURE.md](01-ARCHITECTURE.md) | Current GPUI text pipeline anatomy & where BiDi plugs in |
| [02-UBA-THEORY.md](02-UBA-THEORY.md) | Unicode BiDi Algorithm theory, math, and formulas you must internalize |
| [03-STRATEGY.md](03-STRATEGY.md) | Two-strategy analysis: DirectWrite-native vs. manual UBA |
| [04-IMPLEMENTATION-PHASES.md](04-IMPLEMENTATION-PHASES.md) | Step-by-step implementation phases with tasks, acceptance criteria |
| [05-DATA-STRUCTURES.md](05-DATA-STRUCTURES.md) | New Rust types and data structures you need to design |
| [06-CURSOR-SELECTION.md](06-CURSOR-SELECTION.md) | Cursor, caret, and selection in bidi context — the hard part |
| [07-TESTING-STRATEGY.md](07-TESTING-STRATEGY.md) | How to test progressively and interpret results |
| [08-REFERENCES.md](08-REFERENCES.md) | Papers, specs, codebases, and implementations to study |

## The Big Picture

```
 ┌────────────────────────────────────────────────────────┐
 │  Logical String: "Hello أهلاً World"                   │
 │                                                        │
 │  ① Paragraph Level Detection (P2, P3)                  │
 │  ② Explicit Level Assignment (X1—X8)                   │
 │  ③ Weak Type Resolution (W1—W7)                        │
 │  ④ Neutral Type Resolution (N0—N2)                     │
 │  ⑤ Implicit Level Assignment (I1, I2)                  │
 │  ⑥ Level Run Reordering (L1—L4)                        │
 │                                                        │
 │  Visual String: "Hello ًلاهأ World"                     │
 │                                                        │
 │  ⑦ Shaping (DirectWrite / HarfBuzz)                    │
 │  ⑧ Glyph Positioning with correct x-coords             │
 │  ⑨ Painting                                            │
 └────────────────────────────────────────────────────────┘
```

## Quick Start (For Future Sessions)

1. Read `01-ARCHITECTURE.md` to understand the current code paths
2. Read `02-UBA-THEORY.md` to understand the algorithm mathematics
3. Read `03-STRATEGY.md` to understand which approach we chose and why
4. Follow `04-IMPLEMENTATION-PHASES.md` step by step
5. Use `cargo run -p bidi-torture` to verify progress visually

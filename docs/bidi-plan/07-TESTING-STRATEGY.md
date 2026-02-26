# 07 — Testing Strategy

## Progressive Verification

Run `cargo run -p bidi-torture` after each phase.
The app shows "Actual" vs "Expected" for each test case.

## Phase-by-Phase Checklist

### After Phase 1 (paragraph direction):
- [ ] "مرحبا بالعالم" renders right-to-left
- [ ] "Hello World" unchanged
- [ ] Empty string doesn't crash
- [ ] Single RTL char renders correctly

### After Phase 2 (mixed text):
- [ ] "Hello أهلاً World" — Arabic reversed in middle
- [ ] First-strong detection works (RTL paragraph with LTR island)
- [ ] Punctuation at boundaries resolves correctly

### After Phase 3 (numbers):
- [ ] "السعر 42 دولار" shows 42 not 24
- [ ] Arabic-Indic digits (٠١٢٣) display correctly
- [ ] "$100" keeps dollar sign with number

### After Phase 4-5 (embeddings/isolates):
- [ ] RLO "Hello" → "olleH"
- [ ] Orphan PDF doesn't crash
- [ ] FSI auto-detects direction
- [ ] Isolates prevent spillover

### After Phase 6-7 (brackets/weak):
- [ ] Parens mirror in RTL context
- [ ] Smart quotes work
- [ ] Combining marks stay with base char

### After Phase 8 (line break):
- [ ] Long mixed text wraps correctly
- [ ] Each wrapped line is independently correct

### After Phase 9 (marks/adversarial):
- [ ] LRM/RLM affect layout invisibly
- [ ] Max nesting (125+) doesn't panic
- [ ] Empty input doesn't crash

## Unit Testing

Add Rust unit tests in `direct_write.rs`:
```rust
#[test]
fn test_paragraph_level_detection() {
    assert_eq!(detect_paragraph_level("Hello"), 0);
    assert_eq!(detect_paragraph_level("مرحبا"), 1);
    assert_eq!(detect_paragraph_level("  مرحبا"), 1);
    assert_eq!(detect_paragraph_level("123"), 0);
    assert_eq!(detect_paragraph_level(""), 0);
}
```

## Unicode Conformance

The official Unicode BiDi conformance test file:
`https://unicode.org/Public/UCD/latest/ucd/BidiTest.txt`

This has 770,000+ test cases. DirectWrite passes these — our job
is just to correctly relay the results. Consider downloading and
running a subset as a regression test.

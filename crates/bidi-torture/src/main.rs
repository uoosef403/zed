// ═══════════════════════════════════════════════════════════════════════════
// BiDi Torture Test — A stress test harness for GPUI RTL/BiDi text rendering
// ═══════════════════════════════════════════════════════════════════════════
//
// Renders every BiDi edge-case so that after adding RTL support to our
// forked GPUI crate, we can visually verify correctness.
//
// HOW TO RUN:
//   cd gpui && cargo run -p bidi-torture
// ═══════════════════════════════════════════════════════════════════════════

use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};

mod test_cases;

// ───────────────────── Data model ──────────────────────

/// A single BiDi test case
#[derive(Clone, Debug)]
pub struct BidiTestCase {
    /// Short label for the category
    pub category: &'static str,
    /// Description of what we're testing
    pub description: &'static str,
    /// The text to render
    pub text: &'static str,
    /// Expected visual order (for human comparison)
    pub expected_visual: &'static str,
}

/// Top-level view that holds all the test cases
struct TortureTestView {
    cases: Vec<BidiTestCase>,
}

impl TortureTestView {
    fn new(cases: Vec<BidiTestCase>) -> Self {
        Self { cases }
    }
}

impl Render for TortureTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut root = div()
            .id("scroll-root")
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .overflow_y_scroll()
            .p_4();

        // Title
        root = root.child(
            div()
                .text_xl()
                .text_color(rgb(0xf38ba8))
                .pb_4()
                .child("BiDi / RTL Torture Test — GPUI"),
        );

        // Subtitle explaining current state
        root = root.child(
            div()
                .text_sm()
                .text_color(rgb(0xa6adc8))
                .pb_6()
                .child(
                    "Below are test cases for BiDi text rendering. \
                     GPUI currently renders all text LTR. \
                     After implementing BiDi, the 'Actual' column should match 'Expected'.",
                ),
        );

        let mut current_category = "";
        for case in &self.cases {
            // Category header
            if case.category != current_category {
                current_category = case.category;
                root = root.child(
                    div()
                        .mt_4()
                        .mb_2()
                        .text_lg()
                        .text_color(rgb(0x89b4fa))
                        .border_b_1()
                        .border_color(rgb(0x45475a))
                        .pb_1()
                        .child(SharedString::from(current_category.to_string())),
                );
            }

            // Individual test case
            root = root.child(render_test_case(case));
        }

        root
    }
}

fn render_test_case(case: &BidiTestCase) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .mb_3()
        .p_3()
        .bg(rgb(0x313244))
        .rounded_md()
        .child(
            // Description
            div()
                .text_xs()
                .text_color(rgb(0x9399b2))
                .mb_1()
                .child(SharedString::from(case.description.to_string())),
        )
        .child(
            // The actual BiDi text (what GPUI renders)
            div()
                .flex()
                .flex_row()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x6c7086))
                        .w(px(60.0))
                        .child("Actual:"),
                )
                .child(
                    div()
                        .text_base()
                        .text_color(rgb(0xcdd6f4))
                        .bg(rgb(0x1e1e2e))
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .flex_1()
                        .child(SharedString::from(case.text.to_string())),
                ),
        )
        .child(
            // Expected visual order
            div()
                .flex()
                .flex_row()
                .gap_2()
                .mt_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x6c7086))
                        .w(px(60.0))
                        .child("Expected:"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa6e3a1))
                        .px_2()
                        .py_1()
                        .child(SharedString::from(case.expected_visual.to_string())),
                ),
        )
}

// ───────────────────── Entry point ─────────────────────

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.), px(800.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| TortureTestView::new(test_cases::all_cases()))
            },
        )
        .unwrap();
    });
}

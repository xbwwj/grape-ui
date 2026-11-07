use egpui::{UiContext, button, div, p};

pub struct Counter {
    count: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn render(&mut self, cx: &mut UiContext) {
        div()
            .id("root")
            .child(button("Increment").id("increment").on_click(cx, || {
                self.count += 1;
            }))
            .child(p(self.count.to_string()))
            .build(cx);
    }
}

fn main() {
    let mut cx = UiContext::new();
    let mut counter = Counter::new();

    // Simulate a click from previous-frame hit testing.
    cx.queue_click("increment");

    // Frame 1:
    // Callback executes synchronously during UI conpub struction.
    counter.render(&mut cx);

    println!("Counter state: {}", counter.count);

    // Frame 2:
    // Updated state appears in the retained content tree.
    counter.render(&mut cx);

    println!("Counter state: {}", counter.count);
}

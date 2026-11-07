use egpui::{UiContext, button, div, text};

fn main() {
    let mut cx = UiContext::new();
    let mut counter = Counter::new();

    cx.queue_click("root/increment");

    counter.render(&mut cx);

    println!("Counter state: {}", counter.count);
    counter.render(&mut cx);
    println!("Counter state: {}", counter.count);
}

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn render(&mut self, cx: &mut UiContext) {
        let count = self.count;

        div()
            .id("root")
            .child(button("Increment").id("increment").on_click(|| {
                self.count += 1;
            }))
            .child(text(count.to_string()))
            .build(cx);
    }
}

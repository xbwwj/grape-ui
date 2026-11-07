# egpui

A combination of egui's immediate mode and gpui's syntax.

```rust
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
```

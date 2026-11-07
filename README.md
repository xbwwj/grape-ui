# egpui

A combination of egui's immediate mode and gpui's syntax.

```rust
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
```

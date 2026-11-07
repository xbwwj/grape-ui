use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeNode {
    id: String,
    kind: NodeKind,
    children: Vec<RuntimeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Div,
    Button { label: String },
    Text { content: String },
}

pub struct UiContext {
    // Events received from the previous frame's hit testing.
    clicked: HashSet<String>,

    // The actual retained content tree.
    retained_tree: Option<RuntimeNode>,
}

impl UiContext {
    pub fn new() -> Self {
        Self {
            clicked: HashSet::new(),
            retained_tree: None,
        }
    }

    // Simulate an input event for this example.
    pub fn queue_click(&mut self, id: impl Into<String>) {
        self.clicked.insert(id.into());
    }

    pub fn take_click(&mut self, id: &str) -> bool {
        self.clicked.remove(id)
    }

    // Compare and retain the new tree.
    pub fn reconcile(&mut self, next: RuntimeNode) {
        let changed = self.retained_tree.as_ref() != Some(&next);

        if changed {
            println!("Tree changed; updating retained tree.");
        } else {
            println!("Tree unchanged.");
        }

        self.retained_tree = Some(next);

        // TODO:
        // - Diff nodes by identity.
        // - Preserve interaction state.
        // - Update layout cache.
        // - Schedule paint.
    }
}

// ============================================================
// Declarative Builder API
// ============================================================

// This callback exists only in the temporary Builder.
// It is never stored in RuntimeNode.
pub type ClickHandler<'a> = Box<dyn FnOnce() + 'a>;

pub struct DivBuilder<'a> {
    id: Option<&'static str>,
    children: Vec<Element<'a>>,
}

pub struct ButtonBuilder<'a> {
    id: Option<&'static str>,
    label: String,
    on_click: Option<ClickHandler<'a>>,
}

pub struct TextBuilder {
    content: String,
}

// Temporary declarative element description.
pub enum Element<'a> {
    Div(DivBuilder<'a>),
    Button(ButtonBuilder<'a>),
    Text(TextBuilder),
}

pub trait IntoElement<'a> {
    fn into_element(self) -> Element<'a>;
}

// ------------------------------------------------------------
// Constructors
// ------------------------------------------------------------

pub fn div<'a>() -> DivBuilder<'a> {
    DivBuilder {
        id: None,
        children: Vec::new(),
    }
}

pub fn button<'a>(label: impl Into<String>) -> ButtonBuilder<'a> {
    ButtonBuilder {
        id: None,
        label: label.into(),
        on_click: None,
    }
}

pub fn text(content: impl Into<String>) -> TextBuilder {
    TextBuilder {
        content: content.into(),
    }
}

// ------------------------------------------------------------
// Element conversions
// ------------------------------------------------------------

impl<'a> IntoElement<'a> for DivBuilder<'a> {
    fn into_element(self) -> Element<'a> {
        Element::Div(self)
    }
}

impl<'a> IntoElement<'a> for ButtonBuilder<'a> {
    fn into_element(self) -> Element<'a> {
        Element::Button(self)
    }
}

impl<'a> IntoElement<'a> for TextBuilder {
    fn into_element(self) -> Element<'a> {
        Element::Text(self)
    }
}

// ------------------------------------------------------------
// Div modifiers
// ------------------------------------------------------------

impl<'a> DivBuilder<'a> {
    pub fn id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn child(mut self, child: impl IntoElement<'a>) -> Self {
        self.children.push(child.into_element());
        self
    }

    // Consumes the Builder.
    // The resulting RuntimeNode contains no callbacks.
    pub fn build(self, cx: &mut UiContext) {
        let tree = build_element(Element::Div(self), cx, "", 0);

        cx.reconcile(tree);
    }
}

// ------------------------------------------------------------
// Button modifiers
// ------------------------------------------------------------

impl<'a> ButtonBuilder<'a> {
    pub fn id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    // The callback may borrow local variables or &mut self.
    // No 'static requirement.
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

// ============================================================
// Builder -> Runtime Tree
// ============================================================

pub fn resolve_id(parent: &str, explicit_id: Option<&'static str>, index: usize) -> String {
    let segment = explicit_id
        .map(str::to_owned)
        .unwrap_or_else(|| index.to_string());

    if parent.is_empty() {
        segment
    } else {
        format!("{parent}/{segment}")
    }
}

pub fn build_element<'a>(
    element: Element<'a>,
    cx: &mut UiContext,
    parent_id: &str,
    index: usize,
) -> RuntimeNode {
    match element {
        Element::Div(div) => {
            let id = resolve_id(parent_id, div.id, index);

            let children = div
                .children
                .into_iter()
                .enumerate()
                .map(|(index, child)| build_element(child, cx, &id, index))
                .collect();

            RuntimeNode {
                id,
                kind: NodeKind::Div,
                children,
            }
        }

        Element::Button(mut button) => {
            let id = resolve_id(parent_id, button.id, index);

            // Dispatch the event synchronously.
            // The callback is consumed and dropped here.
            if cx.take_click(&id) {
                if let Some(callback) = button.on_click.take() {
                    callback();
                }
            }

            RuntimeNode {
                id,
                kind: NodeKind::Button {
                    label: button.label,
                },
                children: Vec::new(),
            }
        }

        Element::Text(text) => {
            let id = resolve_id(parent_id, None, index);

            RuntimeNode {
                id,
                kind: NodeKind::Text {
                    content: text.content,
                },
                children: Vec::new(),
            }
        }
    }
}

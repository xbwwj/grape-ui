use std::collections::HashSet;

// ============================================================
// Runtime: retained representation
// ============================================================

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
    // Clicks produced by previous-frame hit testing.
    // Explicit widget IDs are globally unique.
    clicked: HashSet<String>,

    // The retained content tree.
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

    // Consume the click. Each click is dispatched at most once.
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

pub struct DivBuilder {
    id: Option<&'static str>,
    children: Vec<Element>,
}

pub struct ButtonBuilder {
    id: Option<&'static str>,
    label: String,
}

pub struct TextBuilder {
    content: String,
}

// Temporary declarative element description.
// No callbacks or borrowed component state are stored here.
enum Element {
    Div(DivBuilder),
    Button(ButtonBuilder),
    Text(TextBuilder),
}

trait IntoElement {
    fn into_element(self) -> Element;
}

// ------------------------------------------------------------
// Conpub structors
// ------------------------------------------------------------

pub fn div() -> DivBuilder {
    DivBuilder {
        id: None,
        children: Vec::new(),
    }
}

pub fn button(label: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder {
        id: None,
        label: label.into(),
    }
}

pub fn p(content: impl Into<String>) -> TextBuilder {
    TextBuilder {
        content: content.into(),
    }
}

// ------------------------------------------------------------
// Element conversions
// ------------------------------------------------------------

impl IntoElement for DivBuilder {
    fn into_element(self) -> Element {
        Element::Div(self)
    }
}

impl IntoElement for ButtonBuilder {
    fn into_element(self) -> Element {
        Element::Button(self)
    }
}

impl IntoElement for TextBuilder {
    fn into_element(self) -> Element {
        Element::Text(self)
    }
}

// ------------------------------------------------------------
// Div modifiers
// ------------------------------------------------------------

impl DivBuilder {
    pub fn id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    // Consume Builder and create the retained tree.
    pub fn build(self, cx: &mut UiContext) {
        let tree = build_element(Element::Div(self), "", 0);

        cx.reconcile(tree);
    }
}

// ------------------------------------------------------------
// Button modifiers
// ------------------------------------------------------------

impl ButtonBuilder {
    pub fn id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    // Immediate-mode event handling.
    //
    // The callback:
    // - Executes synchronously.
    // - Is never stored in ButtonBuilder.
    // - Is never stored in RuntimeNode.
    // - Can freely borrow local variables and &mut self.
    //
    // An explicit globally unique ID is required because
    // the parent path is not available at this point.
    pub fn on_click<F>(self, cx: &mut UiContext, callback: F) -> Self
    where
        F: FnOnce(),
    {
        let id = self.id.expect("on_click requires an explicit widget ID");

        if cx.take_click(id) {
            callback();
        }

        self
    }
}

// ============================================================
// Builder -> Runtime Tree
// ============================================================

pub fn resolve_id(parent: &str, explicit_id: Option<&'static str>, index: usize) -> String {
    match explicit_id {
        // Explicit IDs are globally unique.
        Some(id) => id.to_owned(),

        // Implicit IDs are positional and parent-scoped.
        None if parent.is_empty() => index.to_string(),

        None => format!("{parent}/{index}"),
    }
}

pub fn build_element(element: Element, parent_id: &str, index: usize) -> RuntimeNode {
    match element {
        Element::Div(div) => {
            let id = resolve_id(parent_id, div.id, index);

            let children = div
                .children
                .into_iter()
                .enumerate()
                .map(|(index, child)| build_element(child, &id, index))
                .collect();

            RuntimeNode {
                id,
                kind: NodeKind::Div,
                children,
            }
        }

        Element::Button(button) => {
            let id = resolve_id(parent_id, button.id, index);

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

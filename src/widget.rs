use crate::{Attribute, Node};
use sauron_vdom::builder::element;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Widget {
    Column,
    Row,
    Button(String),
    Text(String),
    Block(String),
}

pub fn widget<A, C, MSG>(widget: Widget, attrs: A, children: C) -> Node<MSG>
where
    C: AsRef<[Node<MSG>]>,
    A: AsRef<[Attribute<MSG>]>,
    MSG: Clone + Debug + 'static,
{
    element(widget, attrs, children)
}

pub fn column<A, C, MSG>(attrs: A, children: C) -> Node<MSG>
where
    C: AsRef<[Node<MSG>]>,
    A: AsRef<[Attribute<MSG>]>,
    MSG: Clone + Debug + 'static,
{
    widget(Widget::Column, attrs, children)
}

pub fn row<A, C, MSG>(attrs: A, children: C) -> Node<MSG>
where
    C: AsRef<[Node<MSG>]>,
    A: AsRef<[Attribute<MSG>]>,
    MSG: Clone + Debug + 'static,
{
    widget(Widget::Row, attrs, children)
}

pub fn button<A, MSG>(attrs: A, txt: &str) -> Node<MSG>
where
    A: AsRef<[Attribute<MSG>]>,
    MSG: Clone + Debug + 'static,
{
    widget(Widget::Button(txt.to_string()), attrs, [])
}

pub fn text<MSG>(txt: &str) -> Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    widget(Widget::Text(txt.to_string()), [], [])
}

pub fn block<MSG>(title: &str) -> Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    widget(Widget::Block(title.to_string()), [], [])
}

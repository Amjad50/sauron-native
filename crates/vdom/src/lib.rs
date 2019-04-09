#![deny(warnings)]
mod callback;
mod diff;
mod patch;
mod vnode;
mod view;

pub use vnode::builder;

pub use callback::Callback;
pub use diff::diff;
pub use patch::Patch;
pub use vnode::{Element, Node, Text, Value};
pub use vnode::{Event, InputEvent, KeyEvent, MouseEvent};
pub use view::View;

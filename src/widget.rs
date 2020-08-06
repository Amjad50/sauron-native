//! widget module provides a unified abstract representation of the widget
//! which has a mapping to the actual widget in the supported backend
//!
use crate::widget::attribute::style;
use crate::widget::attribute::util::find_value;
use crate::{AttribKey, Attribute, Node, Value};
pub use attribute::event;
use mt_dom::{attr, element};
use std::fmt::Debug;
use stretch::geometry::Size;
use stretch::style::Dimension;
use stretch::style::FlexDirection;
use stretch::style::PositionType;
use stretch::style::Style;

pub mod attribute;
pub(crate) mod layout;

/// TODO: Each widget variant will need to have more details
///  such as attributes, that will be converted to their
///  corresponding target widget of each platform
///
/// Widget definitions
/// This will have a counterparts for each of the supported
/// different platforms
#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
    /// vertical flexbox
    Vbox,
    /// horizontal flexbox
    Hbox,
    /// vertical resizable flexbox
    Vpane,
    /// horizontal resizable flexbox
    Hpane,
    /// a button widget
    Button,
    /// a text label
    Label,
    /// text paragraph
    Paragraph,
    /// text input
    TextInput,
    /// checkbox
    Checkbox,
    /// radio control
    Radio,
    /// image widget
    Image,
    /// svg widget
    Svg,
    /// textarea widget
    TextArea,
    /// an overlay widget
    /// TODO: rename to cardbox
    Overlay,
    /// groupbox
    GroupBox,
    /// headerbar, such as in gtk where it can contains
    /// menu buttons
    HeaderBar,
}

/// a helper function to create widget elements
pub fn widget<MSG>(
    widget: Widget,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    element(widget, attrs, children)
}

/// a vertically oriented flexbox
pub fn column<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Column,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Vbox, attrs, children)
}

/// create a horizontally oriented flexbox
pub fn row<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Row,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Hbox, attrs, children)
}

/// create a vertically oriented resizable flexbox
pub fn vpane<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Column,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Vpane, attrs, children)
}

/// create a horizontally oriented resizable flexbox
pub fn hpane<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Row,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Hpane, attrs, children)
}

/// overlay can be on top of other widgets
pub fn overlay<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    mut children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    children.iter_mut().for_each(|child| {
        child.add_attributes_ref_mut(vec![attr(
            AttribKey::PositionType,
            Value::from(PositionType::Absolute),
        )]);
    });

    attrs.push(style(Style {
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Overlay, attrs, children)
}

/// group widges together will a visible label and border enclosure
pub fn groupbox<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::GroupBox, attrs, children)
}

/// create a button
pub fn button<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Button, attrs, vec![])
}

/// create a text paragraph
pub fn paragraph<MSG>(txt: &str) -> Node<MSG>
where
    MSG: 'static,
{
    widget(
        Widget::Paragraph,
        vec![
            attribute::value(txt.to_string()),
            style(Style {
                size: Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                },
                ..Default::default()
            }),
        ],
        vec![],
    )
}

/// create a text input
pub fn text_input<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::TextInput, attrs, vec![])
}

/// create a checkbox control
pub fn checkbox<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    widget(Widget::Checkbox, attrs, vec![])
}

/// create a radio control
pub fn radio<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    widget(Widget::Radio, attrs, vec![])
}

/// create an image control
pub fn image<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::Image, attrs, vec![])
}

/// create an image control from svg
pub fn svg<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::Svg, attrs, vec![])
}

/// create a text area
pub fn textarea<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::TextArea, attrs, vec![])
}

/// create a text label
pub fn text_label<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::Label, attrs, vec![])
}

pub fn header_bar<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Auto
            },
        },
        ..Default::default()
    }));
    widget(Widget::HeaderBar, attrs, children)
}

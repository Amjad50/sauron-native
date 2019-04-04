use ui_backend::html::attributes::*;
use ui_backend::html::events::*;
use ui_backend::html::*;
use virtual_dom::builder::*;
use virtual_dom::diff;

fn main() {
    let old = div(
        [
            class("some-class"),
            id("some-id"),
            on_click(|_| {
                println!("clicked");
            }),
            attr("data-id", 1),
            on("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        [input([class("client"), r#type("checkbox")], [])],
    );
    let new = div(
        [
            class("some-class2"),
            id("some-id2"),
            on_click(|_| {
                println!("clicked2");
            }),
            attr("data-id", 2),
            on("mouseover", |_| {
                println!("i've been clicked2");
            }),
        ],
        [input([class("client"), r#type("checkbox")], [])],
    );
    println!("{}", old);
    println!("{}", new);
    let patches = diff(&old, &new);
    println!("patches: {:#?}", patches);
}

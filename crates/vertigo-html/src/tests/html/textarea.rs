use crate::html_component;

// Make crate available by its name for html macro
use crate as vertigo_html;

use super::utils::*;

#[test]
fn empty_textarea() {
    let textarea = html_component!("
        <textarea></textarea>
    ");

    assert_empty(&textarea, "textarea");
}

#[test]
fn textarea_with_expression() {
    let textarea = html_component!(r#"
        <textarea>{$ format!("Some {}", "Value") $}</textarea>
    "#);

    assert_eq!(textarea.name, "textarea");

    let text = get_text(&textarea.child[0]);
    assert_eq!(text.value, "Some Value");
}

#[test]
fn div_with_textarea() {
    let div = html_component!("
        <div>
            Label
            <textarea>Some Value</textarea>
        </div>
    ");

    assert_eq!(div.name, "div");
    assert_eq!(div.child.len(), 2);

    let label = get_text(&div.child[0]);
    assert_eq!(label.value, "Label");

    let textarea = get_node(&div.child[1]);
    assert_eq!(textarea.name, "textarea");

    let text = get_text(&textarea.child[0]);
    assert_eq!(text.value, "Some Value");
}

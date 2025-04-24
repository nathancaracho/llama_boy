use serde_json::Value;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct JsonTreeProps {
    pub value: Value,
}

/// Componente que renderiza NÓ da árvore JSON
#[function_component(JsonTreeNode)]
fn json_tree_node(props: &JsonTreeProps) -> Html {
    let open = use_state(|| false);
    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    match &props.value {
        Value::Object(map) => {
            let items = map
                .iter()
                .map(|(k, v)| {
                    html! {
                        <div style="padding-left:1rem;">
                            <strong>{ format!("\"{}\": ", k) }</strong>
                            <JsonTreeNode value={v.clone()} />
                        </div>
                    }
                })
                .collect::<Html>();

            html! {
                <div>
                    <span onclick={toggle.clone()} style="cursor:pointer; user-select:none;">
                        { if *open { "▼ " } else { "▶ " } }
                        { format!("Object({})", map.len()) }
                    </span>
                    { if *open { items } else { html!{} } }
                </div>
            }
        }
        Value::Array(arr) => {
            let items = arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    html! {
                        <div style="padding-left:1rem;">
                            <span style="color: #666;">{ format!("[{}] ", i) }</span>
                            <JsonTreeNode value={v.clone()} />
                        </div>
                    }
                })
                .collect::<Html>();

            html! {
                <div>
                    <span onclick={toggle.clone()} style="cursor:pointer; user-select:none;">
                        { if *open { "▼ " } else { "▶ " } }
                        { format!("Array({})", arr.len()) }
                    </span>
                    { if *open { items } else { html!{} } }
                </div>
            }
        }
        // Valores primitivos ficam “folhas”
        Value::String(s) => html! { <span style="color: #a31515;">{ format!("\"{}\"", s) }</span> },
        Value::Number(n) => html! { <span style="color: #098658;">{ n.to_string() }</span> },
        Value::Bool(b) => html! { <span style="color: #0000ff;">{ b }</span> },
        Value::Null => html! { <span style="color: #808080;">{ "null" }</span> },
    }
}

/// Componente raiz que recebe o JSON e dispara a árvore
#[function_component(JsonTree)]
pub fn json_tree(props: &JsonTreeProps) -> Html {
    html! {
        <div style="font-family: monospace; font-size: 0.9rem;">
            <JsonTreeNode value={props.value.clone()} />
        </div>
    }
}

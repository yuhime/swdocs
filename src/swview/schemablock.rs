use std::rc::Rc;

use crate::{
    types::swagger::{Components, Schema},
    utils::field_type_display,
};

use leptos::prelude::*;

#[component]
pub fn SchemaBlock(
    name: String,
    schema: Schema,
    components: Rc<Components>,
    #[prop(default = 0)] depth: usize,
) -> impl IntoView {
    if depth > 4 {
        return view! { <div class="schema-desc">"…"</div> }.into_any();
    }

    let schema_type = schema
        .schema_type
        .clone()
        .unwrap_or_else(|| "object".into());
    let desc = schema.description.clone().unwrap_or_default();
    let required: std::collections::HashSet<String> = schema.required.iter().cloned().collect();

    let props: Vec<(String, Schema, bool)> = schema
        .properties
        .iter()
        .map(|(k, v)| (k.clone(), v.clone(), required.contains(k)))
        .collect();

    let components_clone = components.clone();

    view! {
        <div class="schema" id=format!("schema-{}", name)>
            <div class="schema-header">
                <span class="schema-name">{name.clone()}</span>
                <span class="schema-type">{schema_type.clone()}</span>
            </div>
            {(!desc.is_empty()).then(|| view! {
                <div class="schema-desc">{desc.clone()}</div>
            })}
            {(!props.is_empty()).then(|| view! {
                <div class="schema-props">
                    {props.into_iter().map(|(pname, pschema, req)| {
                        let ty = field_type_display(&pschema, &components_clone);
                        let pdesc = pschema.description.clone().unwrap_or_default();
                        let nested = if pschema.reference.is_none()
                            && pschema.schema_type.as_deref() == Some("object")
                            && !pschema.properties.is_empty()
                        {
                            Some((pname.clone(), pschema.clone()))
                        } else { None };
                        let comps = components_clone.clone();
                        view! {
                            <div class="schema-prop">
                                <span class="name">
                                    {pname.clone()}
                                    {req.then(|| view! { <span class="req">"*"</span> })}
                                </span>
                                <span class="type">{ty}</span>
                                <span class="desc">{pdesc}</span>
                            </div>
                            {nested.map(|(n, s)| view! {
                                <SchemaBlock name=n schema=s components=comps depth=depth + 1 />
                            })}
                        }
                    }).collect_view()}
                </div>
            })}
        </div>
    }
    .into_any()
}

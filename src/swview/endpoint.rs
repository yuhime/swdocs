use leptos::prelude::*;

use crate::{
    swview::schemablock::SchemaBlock,
    types::swagger::{Components, Operation, Schema},
    utils::resolve_schema_chain,
};
use std::rc::Rc;

#[component]
pub fn Endpoint(
    method: String,
    path: String,
    op: Operation,
    components: Rc<Components>,
) -> impl IntoView {
    let method_upper = method.to_uppercase();
    let method_lower = method.to_lowercase();
    let anchor_id = crate::utils::anchor(&method, &path);
    let path_html = crate::utils::path_to_html(&path);

    let op_id = op.operation_id.clone().unwrap_or_default();
    let summary = op.summary.clone().unwrap_or_default();
    let desc = op.description.clone().unwrap_or_default();
    let tags = op.tags.clone();
    let params = op.parameters.clone();
    let body = op.request_body.clone();
    let responses = op.responses.clone();

    let has_summary = !summary.is_empty();
    let has_desc = !desc.is_empty() && desc != summary;
    let has_tags = !tags.is_empty();
    let has_params = !params.is_empty();
    let has_body = body.is_some();
    let has_responses = !responses.is_empty();

    let param_rows: Vec<(String, String, String, String, bool)> = params
        .iter()
        .map(|p| {
            let name = p.name.clone().unwrap_or_else(|| "?".into());
            let loc = p.location.clone().unwrap_or_else(|| "?".into());
            let ty = p
                .schema
                .as_ref()
                .map(crate::utils::schema_type_str)
                .unwrap_or_else(|| "any".into());
            let d = p.description.clone().unwrap_or_default();
            (name, loc, ty, d, p.required)
        })
        .collect();

    let body_entries: Vec<(String, Option<Schema>, bool)> = body
        .as_ref()
        .map(|b| {
            b.content
                .iter()
                .map(|(mime, media)| {
                    let schema = media
                        .schema
                        .as_ref()
                        .map(|s| resolve_schema_chain(s, &components, &mut Vec::new()));
                    (mime.clone(), schema, b.required)
                })
                .collect()
        })
        .unwrap_or_default();

    let mut resp_sorted: Vec<(String, String)> = responses
        .iter()
        .map(|(k, v)| (k.clone(), v.description.clone().unwrap_or_default()))
        .collect();
    resp_sorted.sort_by(|a, b| a.0.cmp(&b.0));

    view! {
        <div class="endpoint" id=anchor_id.clone()>
            <div class="endpoint-header">
                <span class=format!("method {}", method_lower)>{method_upper.clone()}</span>
                <span class="path" inner_html=path_html></span>
                {(!op_id.is_empty()).then(|| view! {
                    <span class="op-id">{op_id.clone()}</span>
                })}
                <span class="toggle">"▶"</span>
            </div>

            <div class="endpoint-body">
                {has_tags.then(|| view! {
                    <div class="tags">
                        {tags.iter().map(|t| view! { <span class="tag">{t.clone()}</span> }).collect_view()}
                    </div>
                })}

                {has_summary.then(|| view! {
                    <div class="endpoint-desc"><strong>{summary.clone()}</strong></div>
                })}

                {has_desc.then(|| view! {
                    <div class="endpoint-desc">{desc.clone()}</div>
                })}

                {has_params.then(|| view! {
                    <div class="section">
                        <div class="section-title">"Parameters"</div>
                        <table>
                            <thead>
                                <tr>
                                    <th>"Name"</th>
                                    <th>"In"</th>
                                    <th>"Type"</th>
                                    <th>"Description"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {param_rows.iter().map(|(name, loc, ty, d, req)| view! {
                                    <tr>
                                        <td>
                                            <span class="param-name">
                                                {name.clone()}
                                                {(*req).then(|| view! { <span class="required">"*"</span> })}
                                            </span>
                                        </td>
                                        <td><span class=format!("param-in {}", loc)>{loc.clone()}</span></td>
                                        <td><span class="param-type">{ty.clone()}</span></td>
                                        <td class="param-desc">{d.clone()}</td>
                                    </tr>
                                }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                })}

                {has_body.then(|| {
                    let entries = body_entries.clone();
                    view! {
                        <div class="section">
                            <div class="section-title">"Request Body"</div>
                            {entries.into_iter().map(|(mime, schema, required)| {
                                let schm = schema.clone();
                                view! {
                                    <div class="body-media">
                                        <div class="schema-header">
                                            <span class="schema-name">{mime.clone()}</span>
                                            {required.then(|| view! {
                                                <span class="schema-type">"required"</span>
                                            })}
                                        </div>
                                        {schm.map(|s| view! {
                                            <SchemaBlock
                                                name="body".to_string()
                                                schema=s
                                                components=components.clone()
                                                depth=4
                                            />
                                        })}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }
                })}

                {has_responses.then(|| view! {
                    <div class="section">
                        <div class="section-title">"Responses"</div>
                        {resp_sorted.iter().map(|(code, desc)| {
                            let cls = crate::utils::status_class(code);
                            view! {
                                <div class="response">
                                    <span class=format!("code {}", cls)>{code.clone()}</span>
                                    <span class="desc">{desc.clone()}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                })}
            </div>
        </div>
    }
}

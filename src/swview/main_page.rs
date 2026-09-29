use std::rc::Rc;

use leptos::prelude::*;

use crate::{
    CSS, JS,
    swview::{endpoint::Endpoint, sidebar::Sidebar},
    types::swagger::{Components, Operation},
};

#[component]
pub fn MainPage(
    title: String,
    version: String,
    groups: Vec<(String, Vec<(String, String, String)>)>,
    endpoints: Vec<(String, String, Operation)>,
    ignored_tags: Vec<String>,
    skipped: usize,
    components: Rc<Components>,
) -> impl IntoView {
    let total_endpoints = endpoints.len();
    let total_tags = groups.len();
    let has_ignored = !ignored_tags.is_empty();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>{title.clone()} " - API Docs"</title>
                <style inner_html=CSS></style>
            </head>
            <body>
                <div class="layout">
                    <Sidebar
                        title=title.clone()
                        version=version.clone()
                        groups=groups.clone()
                    />

                    <main class="main">
                        <h1 style="margin-bottom:8px;font-size:24px">{title.clone()}</h1>
                        <p style="color:var(--text-muted);margin-bottom:16px">
                            "Version " {version.clone()}
                            " · " {total_endpoints} " endpoints"
                            " · " {total_tags} " tags"
                        </p>

                        {has_ignored.then(|| view! {
                            <p style="color:var(--text-muted);margin-bottom:32px;font-size:12px">
                                "Ignoring tags: "
                                <code>{ignored_tags.join(", ")}</code>
                                " (" {skipped} " endpoints hidden)"
                            </p>
                        })}

                        {groups.iter().map(|(tag, _)| {
                            let tag_name = tag.clone();
                            let tag_clone = tag.clone();
                            let endpoints_for_tag: Vec<_> = endpoints
                                .iter()
                                .filter(|(_, _, op)| {
                                    op.tags.first().map(|t| t == &tag_clone).unwrap_or(false)
                                })
                                .map(|(m, p, op)| (m.clone(), p.clone(), op.clone()))
                                .collect();

                            view! {
                                <h2 style="margin:32px 0 16px;font-size:16px;color:var(--accent);text-transform:uppercase;letter-spacing:1px">
                                    {tag_name}
                                </h2>
                                {endpoints_for_tag.into_iter().map(|(m, p, op)| view! {
                                    <Endpoint method=m path=p op=op components=components.clone() />
                                }).collect_view()}
                            }
                        }).collect_view()}
                    </main>
                </div>

                <script inner_html=JS></script>
            </body>
        </html>
    }
}

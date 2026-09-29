use leptos::prelude::*;

#[component]
pub fn Sidebar(
    title: String,
    version: String,
    groups: Vec<(String, Vec<(String, String, String)>)>,
) -> impl IntoView {
    view! {
        <nav class="sidebar">
            <div class="sidebar-header">
                <h1>
                    {title.clone()}
                    <span class="version">"v"{version.clone()}</span>
                </h1>
            </div>

            <div class="sidebar-search">
                <input type="text" id="search" placeholder="🔍 Search endpoints..."/>
            </div>

            {groups.iter().map(|(tag, items)| {
                let tag = tag.clone();
                let items = items.clone();
                let count = items.len();
                view! {
                    <div class="nav-group">
                        <div class="nav-group-title">
                            <span>{tag.clone()}</span>
                            <span class="count">{count}</span>
                        </div>
                        <div class="nav-items">
                            {items.iter().map(|(m, p, a)| {
                                let m = m.clone();
                                let p = p.clone();
                                let a = a.clone();
                                let badge = m[..m.len().min(4)].to_string();
                                let m_lower = m.to_lowercase();
                                view! {
                                    <a class="nav-item" href=format!("#{}", a)>
                                        <span class=format!("method-badge {}", m_lower)>{badge}</span>
                                        <span>{p.clone()}</span>
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }
            }).collect_view()}
        </nav>
    }
}

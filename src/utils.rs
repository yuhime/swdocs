use crate::types::swagger::{Components, Operation, PathItem, Schema};
use crate::types::swagger::{Parameter, RequestBody, Response};

pub enum ResolvedRef<'a> {
    Schema(&'a Schema),
    Parameter(&'a Parameter),
    RequestBody(&'a RequestBody),
    Response(&'a Response),
}

pub fn resolve_ref<'a>(reference: &str, components: &'a Components) -> Option<ResolvedRef<'a>> {
    let rest = reference.strip_prefix("#/")?;
    let parts: Vec<&str> = rest.split('/').collect();
    if parts.len() != 3 || parts[0] != "components" {
        return None;
    }
    let name = unescape_json_pointer(parts[2]);
    match parts[1] {
        "schemas" => components.schemas.get(&name).map(ResolvedRef::Schema),
        "parameters" => components.parameters.get(&name).map(ResolvedRef::Parameter),
        "requestBodies" => components
            .request_bodies
            .get(&name)
            .map(ResolvedRef::RequestBody),
        "responses" => components.responses.get(&name).map(ResolvedRef::Response),
        _ => None,
    }
}

pub fn resolve_schema_chain(
    schema: &Schema,
    components: &Components,
    visited: &mut Vec<String>,
) -> Schema {
    let Some(reference) = &schema.reference else {
        return schema.clone();
    };

    let name = reference
        .rsplit('/')
        .next()
        .unwrap_or(reference)
        .to_string();

    if visited.contains(&name) {
        return schema.clone();
    }
    visited.push(name.clone());

    let target = if reference.contains("/schemas/") {
        components.schemas.get(&name)
    } else {
        None
    };

    match target {
        Some(resolved) => resolve_schema_chain(resolved, components, visited),
        None => schema.clone(),
    }
}

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn path_to_html(path: &str) -> String {
    let mut out = String::new();
    let mut in_param = false;
    for c in path.chars() {
        match c {
            '{' => {
                out.push_str("<span class=\"param\">{");
                in_param = true;
            }
            '}' => {
                out.push_str("}</span>");
                in_param = false;
            }
            _ => out.push(c),
        }
    }
    let _ = in_param;
    out
}

pub fn resolve_schema_ref<'a>(reference: &str, components: &'a Components) -> Option<&'a Schema> {
    let rest = reference.strip_prefix("#/")?;
    let parts: Vec<&str> = rest.split('/').collect();

    if parts.len() != 3 || parts[0] != "components" {
        return None;
    }

    let section = parts[1];
    let name = unescape_json_pointer(parts[2]);

    match section {
        "schemas" => components.schemas.get(&name),
        _ => None,
    }
}

fn unescape_json_pointer(s: &str) -> String {
    s.replace("~1", "/").replace("~0", "~")
}

pub fn field_type_display(s: &Schema, components: &Components) -> String {
    if let Some(r) = &s.reference {
        let name = r.rsplit('/').next().unwrap_or(r);
        if let Some(resolved) = components.schemas.get(name) {
            if !resolved.enum_values.is_empty() {
                let vals: Vec<String> = resolved
                    .enum_values
                    .iter()
                    .map(|v| match v {
                        serde_json::Value::String(s) => format!("\"{}\"", s),
                        other => other.to_string(),
                    })
                    .collect();
                return format!("{} = {}", name, vals.join(" | "));
            }
            if let Some(t) = &resolved.schema_type {
                return format!("{} ({})", name, t);
            }
            return name.to_string();
        }
        return name.to_string();
    }

    let t = s.schema_type.as_deref().unwrap_or("any");
    if t == "array"
        && let Some(items) = &s.items
    {
        return format!("array<{}>", field_type_display(items, components));
    }
    if !s.enum_values.is_empty() {
        let vals: Vec<String> = s
            .enum_values
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => format!("\"{}\"", s),
                other => other.to_string(),
            })
            .collect();
        return format!("enum({})", vals.join(" | "));
    }
    match &s.format {
        Some(f) => format!("{}({})", t, f),
        None => t.to_string(),
    }
}

pub fn anchor(method: &str, path: &str) -> String {
    let clean: String = path
        .trim_matches('/')
        .chars()
        .map(|c| {
            if c == '/' || c == '{' || c == '}' {
                '-'
            } else {
                c
            }
        })
        .collect();
    let clean = clean.trim_matches('-');
    if clean.is_empty() {
        format!("{}-root", method.to_lowercase())
    } else {
        format!("{}-{}", method.to_lowercase(), clean)
    }
}

pub fn schema_type_str(s: &Schema) -> String {
    if let Some(r) = &s.reference {
        return r.rsplit('/').next().unwrap_or(r).to_string();
    }
    let t = s.schema_type.as_deref().unwrap_or("any");
    if t == "array"
        && let Some(items) = &s.items
    {
        return format!("array<{}>", schema_type_str(items));
    }
    if !s.enum_values.is_empty() {
        return format!("enum({})", t);
    }
    match &s.format {
        Some(f) => format!("{}({})", t, f),
        None => t.to_string(),
    }
}

pub fn status_class(code: &str) -> &'static str {
    match code.chars().next() {
        Some('2') => "s2xx",
        Some('3') => "s3xx",
        Some('4') => "s4xx",
        Some('5') => "s5xx",
        _ => "",
    }
}

pub fn operations_of(item: &PathItem) -> Vec<(&'static str, &Operation)> {
    let mut v = Vec::new();
    if let Some(o) = &item.get {
        v.push(("get", o));
    }
    if let Some(o) = &item.post {
        v.push(("post", o));
    }
    if let Some(o) = &item.put {
        v.push(("put", o));
    }
    if let Some(o) = &item.patch {
        v.push(("patch", o));
    }
    if let Some(o) = &item.delete {
        v.push(("delete", o));
    }
    if let Some(o) = &item.head {
        v.push(("head", o));
    }
    if let Some(o) = &item.options {
        v.push(("options", o));
    }
    if let Some(o) = &item.trace {
        v.push(("trace", o));
    }
    v
}

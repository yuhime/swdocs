use crate::types::swagger::*;

pub fn expand_all(spec: &mut OpenApi) {
    let components = spec.components.clone();

    for (_, item) in spec.paths.iter_mut() {
        expand_parameters(&mut item.parameters, &components);
        for op in item.operations_mut() {
            expand_parameters(&mut op.parameters, &components);
            expand_request_body(&mut op.request_body, &components);
            expand_responses(&mut op.responses, &components);
        }
    }
}

fn expand_parameters(params: &mut Vec<Parameter>, components: &Components) {
    for p in params.iter_mut() {
        if let Some(r) = p.reference.clone()
            && let Some((section, name)) = split_ref(&r)
            && section == "parameters"
            && let Some(resolved) = components.parameters.get(name)
        {
            *p = resolved.clone();
        }
    }
}

fn expand_request_body(body: &mut Option<RequestBody>, components: &Components) {
    if let Some(b) = body
        && let Some(r) = b.reference.clone()
        && let Some((section, name)) = split_ref(&r)
        && section == "requestBodies"
        && let Some(resolved) = components.request_bodies.get(name)
    {
        *b = resolved.clone();
    }
}

fn expand_responses(
    responses: &mut std::collections::HashMap<String, Response>,
    components: &Components,
) {
    for (_, r) in responses.iter_mut() {
        if let Some(rr) = r.reference.clone() {
            if let Some((section, name)) = split_ref(&rr) {
                if section == "responses" {
                    if let Some(resolved) = components.responses.get(name) {
                        *r = resolved.clone();
                    }
                }
            }
        }
    }
}

fn split_ref(reference: &str) -> Option<(&str, &str)> {
    let rest = reference.strip_prefix("#/components/")?;
    let mut parts = rest.splitn(2, '/');
    let section = parts.next()?;
    let name = parts.next()?;
    Some((section, name))
}

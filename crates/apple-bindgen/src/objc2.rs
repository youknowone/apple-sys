//! Post-process bindgen output to use objc2 instead of objc 0.2.
//!
//! bindgen generates ObjC bindings using `objc` 0.2 syntax.
//! This module transforms the output to `objc2` 0.6 syntax.

/// Apply all objc → objc2 transformations to generated binding code.
pub fn migrate(content: &str) -> String {
    let mut result = content.to_string();

    // 1. Replace `use objc::{...};` import line (items may appear in any order)
    result = replace_objc_import(&result);

    // 1b. Convert `pub type BOOL = bool;` to a newtype struct.
    // objc2 intentionally does NOT implement Encode/RefEncode for `bool`.
    // A newtype struct gets Encode impls from `add_struct_encode_impls` below.
    result = result.replace(
        "pub type BOOL = bool;",
        "#[repr(transparent)]\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\npub struct BOOL(pub bool);",
    );

    // 2. Replace type references
    result = result.replace("objc::runtime::Object", "objc2::runtime::AnyObject");
    result = result.replace("objc::runtime::", "objc2::runtime::");

    // 3. Transform Message impls (before renaming objc::Message in where clauses)
    result = transform_message_impls(&result);

    // 4. Replace remaining objc::Message references (in where clauses etc.)
    result = result.replace("objc::Message", "objc2::Message");

    // 5. Replace class!(Name) → objc2::runtime::AnyClass::get(c"Name").unwrap()
    result = replace_class_macro(&result);

    // 6. Transform msg_send! invocations
    result = transform_msg_send(&result);

    // 7. Add RefEncode + Encode impls for all struct types
    result = add_struct_encode_impls(&result);

    result
}

/// Replace `use objc::{...};` import line with `use objc2::msg_send;`.
///
/// bindgen may emit the imports in any order (e.g. `{self, msg_send, sel, sel_impl, class}`
/// vs `{self, class, msg_send, sel, sel_impl}`), so we match any line starting with `use objc::{`.
fn replace_objc_import(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use objc::{") && trimmed.ends_with("};") {
            result.push_str("#[allow(unused_imports)]\nuse objc2::msg_send;\n");
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

/// Transform `unsafe impl objc::Message for Foo {}` → `unsafe impl objc2::Message for Foo {}`.
///
/// RefEncode + Encode impls are added separately by `add_struct_encode_impls` for ALL structs.
fn transform_message_impls(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("unsafe impl objc::Message for ") {
            if let Some(name) = rest.strip_suffix(" {}") {
                result.push_str(&format!("unsafe impl objc2::Message for {name} {{}}\n"));
                continue;
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

/// Add `RefEncode` + `Encode` impls for ALL `pub struct` types.
///
/// This satisfies objc2's msg_send! trait bounds for both pointer targets
/// (`*mut T` requires `T: RefEncode`) and direct arguments (`T: Encode`).
///
/// For ObjC wrapper types (single `id` field like `NSObject(pub id)`),
/// `Encoding::Object` is used. For C/CG structs with fields (e.g. CGRect),
/// `Encoding::Struct(name, &[])` is used as an opaque struct encoding.
/// objc2 treats opaque structs as compatible with any struct of the same name,
/// which passes runtime verification without needing exact field encodings.
fn add_struct_encode_impls(content: &str) -> String {
    let mut names = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let rest_opt = trimmed
            .strip_prefix("pub struct ")
            .or_else(|| trimmed.strip_prefix("pub union "));
        if let Some(rest) = rest_opt {
            if rest.contains('<') {
                continue;
            }
            let name_end = rest
                .find(|c: char| c == '{' || c == '(' || c == ';' || c == ' ')
                .unwrap_or(rest.len());
            let name = rest[..name_end].trim();
            if !name.is_empty() {
                names.push(name.to_string());
            }
        }
    }

    if names.is_empty() {
        return content.to_string();
    }

    // Detect which structs are ObjC wrapper types (single `pub id` field)
    let objc_wrappers: std::collections::HashSet<&str> = {
        let mut set = std::collections::HashSet::new();
        for line in content.lines() {
            let trimmed = line.trim();
            let rest_opt = trimmed
                .strip_prefix("pub struct ")
                .or_else(|| trimmed.strip_prefix("pub union "));
            if let Some(rest) = rest_opt {
                if rest.ends_with("(pub id);") {
                    let name_end = rest.find('(').unwrap_or(rest.len());
                    let name = rest[..name_end].trim();
                    set.insert(name);
                }
            }
        }
        set
    };

    let mut impls = String::from("\n");
    for name in &names {
        let encoding = if objc_wrappers.contains(name.as_str()) {
            // ObjC wrapper type: use Object encoding
            "objc2::encode::Encoding::Object".to_string()
        } else {
            // C struct: use opaque Struct encoding (name only, no fields)
            format!("objc2::encode::Encoding::Struct(\"{name}\", &[])")
        };
        let ref_encoding =
            format!("objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING)");
        impls.push_str(&format!(
            "unsafe impl objc2::encode::RefEncode for {name} {{\n    \
             const ENCODING_REF: objc2::encode::Encoding = {ref_encoding};\n\
             }}\n\
             unsafe impl objc2::encode::Encode for {name} {{\n    \
             const ENCODING: objc2::encode::Encoding = {encoding};\n\
             }}\n"
        ));
    }

    let mut result = content.to_string();
    result.push_str(&impls);
    result
}

/// Replace `class!(ClassName)` / `class ! (ClassName)` with
/// `objc2::runtime::AnyClass::get(c"ClassName").unwrap()`.
fn replace_class_macro(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut remaining = content;
    while let Some(pos) = remaining.find("class") {
        let after_class = &remaining[pos + 5..];
        let mut j = 0;
        while j < after_class.len() && after_class.as_bytes()[j] == b' ' {
            j += 1;
        }
        if j < after_class.len() && after_class.as_bytes()[j] == b'!' {
            j += 1;
            while j < after_class.len() && after_class.as_bytes()[j] == b' ' {
                j += 1;
            }
            if j < after_class.len() && after_class.as_bytes()[j] == b'(' {
                j += 1;
                let args_start = j;
                if let Some(close) = after_class[args_start..].find(')') {
                    let name = after_class[args_start..args_start + close].trim();
                    result.push_str(&remaining[..pos]);
                    result.push_str(&format!(
                        "objc2::runtime::AnyClass::get(c\"{name}\").unwrap()"
                    ));
                    remaining = &after_class[args_start + close + 1..];
                    continue;
                }
            }
        }
        result.push_str(&remaining[..pos + 5]);
        remaining = after_class;
    }
    result.push_str(remaining);
    result
}

/// Transform msg_send! invocations:
/// - Wrap receiver with `&*` for objc2's MessageReceiver trait
/// - Add commas between multi-arg selector parts
fn transform_msg_send(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let bytes = content.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if i + 8 <= bytes.len() && &content[i..i + 8] == "msg_send" {
            let mut j = i + 8;
            while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'!') {
                j += 1;
            }
            if j < bytes.len() && (bytes[j] == b'(' || bytes[j] == b'[') {
                let open = bytes[j];
                let close = if open == b'(' { b')' } else { b']' };

                let mut depth = 1i32;
                let mut end = j + 1;
                while end < bytes.len() && depth > 0 {
                    if bytes[end] == open {
                        depth += 1;
                    }
                    if bytes[end] == close {
                        depth -= 1;
                    }
                    if depth > 0 {
                        end += 1;
                    }
                }

                let body = &content[j + 1..end];

                let mut d = 0i32;
                let mut recv_end = None;
                for (k, &b) in body.as_bytes().iter().enumerate() {
                    match b {
                        b'(' | b'[' => d += 1,
                        b')' | b']' => d -= 1,
                        b',' if d == 0 => {
                            recv_end = Some(k);
                            break;
                        }
                        _ => {}
                    }
                }

                if let Some(re) = recv_end {
                    let receiver = body[..re].trim();
                    let selector = body[re + 1..].trim();
                    let new_receiver = format!("&*{receiver}");
                    let new_selector = insert_selector_commas(selector);

                    result.push_str("msg_send!");
                    result.push(open as char);
                    result.push_str(&new_receiver);
                    result.push_str(", ");
                    result.push_str(&new_selector);
                    result.push(close as char);
                } else {
                    result.push_str("msg_send!");
                    result.push(open as char);
                    result.push_str(body);
                    result.push(close as char);
                }

                i = end + 1;
                continue;
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

/// Insert commas between multi-arg selector parts in a msg_send body.
///
/// `getValue : value size : size` → `getValue : value, size : size`
fn insert_selector_commas(selector: &str) -> String {
    let parts: Vec<&str> = selector.split(" : ").collect();
    if parts.len() <= 2 {
        return selector.to_string();
    }

    let mut result = String::new();
    result.push_str(parts[0]);
    result.push_str(" : ");

    for i in 1..parts.len() - 1 {
        let part = parts[i];
        if let Some(last_space) = part.rfind(' ') {
            let arg = &part[..last_space];
            let next_sel = &part[last_space + 1..];
            result.push_str(arg);
            result.push_str(", ");
            result.push_str(next_sel);
        } else {
            result.push_str(part);
        }
        result.push_str(" : ");
    }

    result.push_str(parts[parts.len() - 1]);
    result
}

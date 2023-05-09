use proc_macro2::TokenStream;
use quote::quote;
use serde_yaml::Value;

pub fn rusty_json(v: &Value) -> TokenStream {
    match v {
        Value::Null => quote! { serde_json::Value::Null },
        Value::Bool(b) => quote! { #b },
        Value::Number(n) => {
            if n.is_f64() {
                let f = n.as_f64().unwrap();
                quote! { #f }
            } else if n.is_i64() {
                let i = n.as_i64().unwrap();
                quote! { #i }
            } else {
                let u = n.as_u64().unwrap();
                quote! { #u }
            }
        }
        Value::String(s) => {
            quote! { #s }
        }
        Value::Mapping(m) => {
            let kvs = m.iter().map(|(k, v)| {
                let k = match k {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    _ => panic!("unsupported key {:?}", k),
                };
                let v = rusty_json(v);
                quote! { #k: #v }
            });
            quote! { { #(#kvs),* } }
        }
        Value::Sequence(s) => {
            let items = s.iter().map(rusty_json);
            quote! { [#(#items),*] }
        }
        _ => panic!("unsupported value {:?}", v),
    }
}

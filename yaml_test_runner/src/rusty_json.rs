use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use serde_yaml::Value;

lazy_static! {
    // replace usages of "$.*" with the captured value
    static ref SET_REGEX: Regex =
        Regex::new(r#"\$(.+)"#).unwrap();

    // replace usages of ${.*} with the captured value
    static ref SET_DELIMITED_REGEX: Regex =
        Regex::new(r#"\$\{([^}]+)\}"#).unwrap();
}

pub fn from_set_value(s: &str) -> TokenStream {
    if s.starts_with('$') {
        match SET_DELIMITED_REGEX
            .captures(s)
            .or_else(|| SET_REGEX.captures(s))
        {
            Some(c) => syn::parse_str(&c[1]).unwrap(),
            None => quote! { #s },
        }
    } else if SET_DELIMITED_REGEX.is_match(s) {
        let mut format_str = s.to_owned();
        let mut args = Vec::<TokenStream>::new();
        for c in SET_DELIMITED_REGEX.captures_iter(s) {
            format_str = format_str.replace(&c[0], "{}");
            args.push(syn::parse_str(&c[1]).unwrap());
        }
        quote! { format!(#format_str, #(#args),*) }
    } else {
        quote! { #s }
    }
}

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
        Value::String(s) => from_set_value(s),
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

#[cfg(test)]
mod test {
    use super::from_set_value;
    use quote::quote;

    #[test]
    fn from_set_value_correctly_interpolates_strings() {
        assert_eq!(
            from_set_value("foo").to_string(),
            quote! { "foo" }.to_string()
        );
        assert_eq!(
            from_set_value("$primary_term").to_string(),
            quote! { primary_term }.to_string()
        );
        assert_eq!(
            from_set_value("${primary_term}").to_string(),
            quote! { primary_term }.to_string()
        );
        assert_eq!(
            from_set_value("yoo${primary_term}foo").to_string(),
            quote! { format!("yoo{}foo", primary_term) }.to_string()
        );
        assert_eq!(
            from_set_value("yoo${primary_term}foo${foo}bar").to_string(),
            quote! { format!("yoo{}foo{}bar", primary_term, foo) }.to_string()
        );
    }
}

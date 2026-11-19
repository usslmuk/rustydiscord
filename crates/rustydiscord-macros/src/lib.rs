use proc_macro::TokenStream;

fn append_ok(s: &str) -> String {
    let t = s.trim_end().to_string();
    let Some(pos) = t.rfind('}') else { return t };
    let mut out = t.clone();
    out.insert_str(pos, ";Ok(())");
    out
}

fn is_fn(s: &str) -> bool {
    s.contains(" fn ")
        || s.trim_start().starts_with("fn ")
        || s.contains("async fn")
        || s.trim_start().starts_with("async fn")
        || s.trim_start().starts_with("pub ")
}

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let s = item.to_string();
    if !is_fn(&s) {
        return append_ok(&s).parse().unwrap_or(item);
    }
    let Some(brace) = s.find('{') else {
        return item;
    };
    let (head, _) = s.split_at(brace);
    if head.contains("->") {
        return append_ok(&s).parse().unwrap_or(item);
    }
    let with_ret = format!("{head} -> Result < (), rustydiscord :: HandlerError > ");
    let rest = &s[brace..];
    let combined = format!("{with_ret}{rest}");
    append_ok(&combined).parse().unwrap_or(item)
}

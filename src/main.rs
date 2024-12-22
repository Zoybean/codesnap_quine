fn main() {
    fn quot(s: &str) -> String {
        format!("r#{0}{s}{0}#", '"')
    }
    fn out(s: &str) {
        println!("fn main() {{{s}}}");
    }
(
    |quine_closure: &str, def: &str| {
        out(&[
            def,
            "(", quine_closure, ")(",
                &quot(quine_closure), ",",
                &quot(def), ",",
            ")"
            ].concat());
        })(r#"
    |quine_closure: &str, def: &str| {
        out(&[
            def,
            "(", quine_closure, ")(",
                &quot(quine_closure), ",",
                &quot(def), ",",
            ")"
            ].concat());
        }"#,r#"
    fn quot(s: &str) -> String {
        format!("r#{0}{s}{0}#", '"')
    }
    fn out(s: &str) {
        println!("fn main() {{{s}}}");
    }
"#,)}


fn quot(s: &str) -> String {
    format!("r#{0}{s}{0}#", '"')
}
fn embed(defs: &str, body: &str) -> String {
    format!("{defs}fn main() {{{body}}}")
}
fn out(s: &str) {
    println!("{s}");
}
fn main() {
    (|quine: &str, def: &str| {
        out(&embed(
            def,
            &[quine, "(", &quot(quine), ", ", &quot(def), ")"].concat(),
        ))
    })(r#"
    (|quine: &str, def: &str| {
        out(&embed(
            def,
            &[quine, "(", &quot(quine), ", ", &quot(def), ")"].concat(),
        ))
    })"#, r#"
fn quot(s: &str) -> String {
    format!("r#{0}{s}{0}#", '"')
}
fn embed(defs: &str, body: &str) -> String {
    format!("{defs}fn main() {{{body}}}")
}
fn out(s: &str) {
    println!("{s}");
}
"#)}

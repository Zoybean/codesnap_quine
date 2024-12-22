fn main() {
    fn quot(q: &str) -> String {
        format!("\"{}\"", q.escape_default())
    }
    fn out(q: &str) {
        println!("fn main () {{ {} }}", q);
    }

    (|quine_closure: &str, quot_def: &str, out_def: &str| {
        let ls = [
            quot_def,
            out_def,
            "(",
            quine_closure,
            ")",
            "(",
            &quot(quine_closure),
            ",",
            &quot(quot_def),
            ",",
            &quot(out_def),
            ",",
            ")",
        ];

        let mut s = String::new();
        for st in ls {
            s += st
        }
        out(&s);
    })(
        "|quine_closure: &str, quot_def: &str, out_def: &str| {
        let ls = [
            quot_def,
            out_def,
            quine_closure,
            \"(\",
            &quot(quine_closure),
            \",\",
            &quot(quot_def),
            \",\",
            &quot(out_def),
            \",\",
            \")\",
        ];

        let mut s = String::new();
        for st in ls {
            s += st
        }
        out(&s);
    }",
        "fn quot(q: &str) -> String { format!(\"\\\"{}\\\"\", q.escape_default()) }",
        "fn out(q: &str) { println!(\"fn main () {{ {} }}\", q); }",
    );
}

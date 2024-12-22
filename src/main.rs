fn main() {
    fn quot(s: &str) -> String {
        format!("r#{0}{s}{0}#", '"')
}
    fn out(q: &str) -> Result<(), Box<dyn Error>> {
        CodeSnap::default()
            .code(
                CodeBuilder::default()
                    .language("rust")
                    .content(format!(
                        "
use std::error::Error;
use codesnap::config::{{CodeBuilder, CodeSnap}};

fn main() -> Result<(), Box<dyn Error>> {{
    {q}
}}
"
                    ))
                    .build()?,
            )
            .build()?
            .create_snapshot()?
            .png_data()?
            .save("./snap.png")?;
        Ok(())
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
    fn out(q: &str) -> Result<(), Box<dyn Error>> {
        CodeSnap::default()
            .code(
                CodeBuilder::default()
                    .language("rust")
                    .content(format!(
                        "
use std::error::Error;
use codesnap::config::{{CodeBuilder, CodeSnap}};

fn main() -> Result<(), Box<dyn Error>> {{
    {q}
}}
"
                    ))
                    .build()?,
            )
            .build()?
            .create_snapshot()?
            .png_data()?
            .save("./snap.png")?;
        Ok(())
    }
"#,)}

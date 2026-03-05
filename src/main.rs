use std::io;

use typst::text::RawElem;

fn escape_md_cell(text: &str) -> String {
    text.replace('|', "\\|")
}

fn main() -> io::Result<()> {
    let langs: Vec<(&'static str, Vec<&'static str>)> = RawElem::languages();

    let mut md = String::from("| Language | Tags |\n|---|---|\n");

    for (name, tags) in langs {
        let tags_joined = tags.join(",");
        println!("{name}\t{tags_joined}");

        md.push_str(&format!(
            "| {} | {} |\n",
            escape_md_cell(name),
            escape_md_cell(&tags_joined)
        ));
    }

    std::fs::write("result.md", md)?;
    Ok(())
}

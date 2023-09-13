use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "flowchart.pest"]
struct Flowchart;

pub fn fishextract(_x: impl std::io::BufRead) -> Result<(), ()> {
    log::info!("fishextract - So good!");

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! explode {
        ($name:ident, $x:expr) => {
            #[test]
            fn $name() {
                let i: usize = $x;
                let p = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join("samples")
                    .join("flowchart")
                    .join(format!("{}.mermaid", i));
                println!("Read {}", p.display());
                let s = std::fs::read_to_string(p).unwrap();
                dbg!(Flowchart::parse(Rule::file, s.as_str())).unwrap();
            }
        };
    }

    explode!(flowchart_t01, 1);
    explode!(flowchart_t02, 2);
    explode!(flowchart_t03, 3);
    explode!(flowchart_t04, 4);
    explode!(flowchart_t05, 5);
    explode!(flowchart_t06, 6);
    explode!(flowchart_t07, 7);
    explode!(flowchart_t08, 8);
    explode!(flowchart_t09, 9);
    explode!(flowchart_t10, 10);
    explode!(flowchart_t11, 11);
    explode!(flowchart_t12, 12);
    explode!(flowchart_t13, 13);
    explode!(flowchart_t14, 14);
    explode!(flowchart_t15, 15);
}

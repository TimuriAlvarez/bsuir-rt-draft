use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "extra/LaTeX.pest"]
struct LaTeXParser;

fn parse(project: &str, path: &str) {
  let file = path.split(project).last().expect(&format!("Failed to get file name for {path}")).replace("/src/", "");
  println!("  => Parsing {file}");
  let content = std::fs::read_to_string(path).expect(&format!("Failed to read file {path}"));
  let parsed = LaTeXParser::parse(Rule::document, &content).expect(&format!("Failed to parse {path}"));
  if parsed.len() != 1 {
    panic!("Invalid document format");
  }
  // TODO: Finish implementation
}

fn project(path: &str) {
  let project = path.replace("/src/manifest.tex", "").split("/").last().expect("Failed to get project name").to_string();
  println!(" -> Generating documentation for {project}...");
  parse(&project, path);
}

pub fn generate(dirs: &[String]) {
  println!("Generating documentation...");
  dirs.into_iter().for_each(|dir: &String| project(&format!("{dir}/manifest.tex")));
}

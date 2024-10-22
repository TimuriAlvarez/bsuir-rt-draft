use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "extra/LaTeX.pest"]
struct LaTeXParser;

type Context<'a> = pest::iterators::Pair<'a, Rule>;

fn parse_comment(comment: Context) {
  match comment.as_rule() {
    Rule::comment_doxygen => comment.into_inner().for_each(|sub: Context| parse_comment(sub)),
    Rule::comment_regular => comment.into_inner().for_each(|sub: Context| parse_comment(sub)),
    Rule::DOXYFN => {},
    Rule::comment_string => {},
    _ => panic!("{comment:?}"),
  }
}
fn parse_command(command: Context) {
  match command.as_rule() {
    Rule::command_impl => command.into_inner().for_each(|sub: Context| parse_command(sub)),
    Rule::comment_regular => command.into_inner().for_each(|sub: Context| parse_command(sub)),
    Rule::CMDNAME => {},
    Rule::command_arg => {},
    _ => panic!("{command:?}"),
  }
}
fn parse_expression(expression: Context) {
  match expression.as_rule() {
    Rule::comment => expression.into_inner().for_each(|comment: Context| parse_comment(comment)),
    Rule::command => expression.into_inner().for_each(|command: Context| parse_command(command)),
    _ => panic!("{expression:?}"),
  }
}
fn parse_document(document: Context) {
  match document.as_rule() {
    Rule::expression => document.into_inner().for_each(|expression: Context| parse_expression(expression)),
    Rule::EOI => {},
    _ => panic!("{document:?}"),
  };
}
fn parse_content(content: Context) {
  match content.as_rule() {
    Rule::document => content.into_inner().for_each(|document: Context| parse_document(document)),
    _ => panic!("{content:?}"),
  };
}
fn parse(project: &str, path: &str) {
  let file = path.split(project).last().expect(&format!("Failed to get file name for {path}")).replace("/src/", "");
  println!("  => Parsing {file}");
  let content = std::fs::read_to_string(path).expect(&format!("Failed to read file {path}"));
  let content = LaTeXParser::parse(Rule::document, &content).expect(&format!("Failed to parse file {path}"));
  content.into_iter().for_each(|content: Context| parse_content(content));
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

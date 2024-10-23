use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "extra/LaTeX.pest"]
struct LaTeXParser;

type Context<'a> = pest::iterators::Pair<'a, Rule>;

struct Doc {
  command_name: String,
  input_vector: Vec::<String>,
}

impl Doc {
  pub fn new() -> Self {
    Self {
      command_name: String::new(),
      input_vector: Vec::<String>::new(),
    }
  }
  fn parse_comment(&self, comment: Context) {
    match comment.as_rule() {
      Rule::comment_doxygen => comment.into_inner().for_each(|sub: Context| self.parse_comment(sub)),
      Rule::comment_regular => comment.into_inner().for_each(|sub: Context| self.parse_comment(sub)),
      Rule::DOXYFN => {},
      Rule::comment_string => {},
      Rule::comment_split => {},
      _ => panic!("{comment:?}"),
    }
  }
  fn process_command_string(&mut self, string: &str) {
    let command = self.command_name.as_str();
    match command {
      "input" => self.input_vector.push(string.to_string()),
      "NewDocumentCommand" => {},
      "usepackage" => {},
      "documentclass" => {},
      _ => eprintln!("[WARNING] Unknown command: {command}"),
    }
  }
  fn process_command_arg(&mut self, arg: Context) {
    match arg.as_rule() {
      Rule::command_left => {},
      Rule::command_right => {},
      Rule::command_left_opt => {},
      Rule::command_right_opt => {},
      Rule::command_arg => {},
      Rule::command_arg_content => arg.into_inner().for_each(|sub: Context| self.process_command_arg(sub)),
      Rule::command_impl => {},
      Rule::command_string => self.process_command_string(arg.as_str()),
      Rule::command_split => {},
      _ => panic!("{arg:?}"),
    }
  }
  fn parse_command(&mut self, command: Context) {
    match command.as_rule() {
      Rule::command_impl => command.into_inner().for_each(|sub: Context| self.parse_command(sub)),
      Rule::comment_regular => command.into_inner().for_each(|sub: Context| self.parse_comment(sub)),
      Rule::command_name => self.command_name = command.as_str().to_string(),
      Rule::command_arg => command.into_inner().for_each(|sub: Context| self.process_command_arg(sub)),
      _ => panic!("{command:?}"),
    }
  }
  fn parse_expression(&mut self, expression: Context) {
    match expression.as_rule() {
      Rule::comment => expression.into_inner().for_each(|comment: Context| self.parse_comment(comment)),
      Rule::command => expression.into_inner().for_each(|command: Context| self.parse_command(command)),
      _ => panic!("{expression:?}"),
    }
  }
  fn parse_document(&mut self, document: Context) {
    match document.as_rule() {
      Rule::expression => document.into_inner().for_each(|expression: Context| self.parse_expression(expression)),
      Rule::EOI => {},
      _ => panic!("{document:?}"),
    };
  }
  fn parse_content(&mut self, content: Context) {
    match content.as_rule() {
      Rule::document => content.into_inner().for_each(|document: Context| self.parse_document(document)),
      _ => panic!("{content:?}"),
    };
  }
  fn parse(&mut self, project: &str, path: &str) {
    let file = path.split(project).last().expect(&format!("Failed to get file name for {path}")).replace("/src/", "");
    println!("  => Parsing {file}");
    let content = std::fs::read_to_string(path).expect(&format!("Failed to read file {path}"));
    let content = LaTeXParser::parse(Rule::document, &content).expect(&format!("Failed to parse file {path}"));
    content.into_iter().for_each(|content: Context| self.parse_content(content));
  }
  pub fn project(&mut self, path: &str) {
    let project = path.replace("/src/manifest.tex", "").split("/").last().expect("Failed to get project name").to_string();
    println!(" -> Generating documentation for {project}...");
    self.parse(&project, path);
    let path = path.replace("/manifest.tex", "");
    let files: Vec::<String> = self.input_vector.clone().into_iter().map(|input: String| format!("{path}/{}.tex", input.split("/src/").last().expect("Failed to get file name"))).collect();
    files.into_iter().for_each(|file: String| self.parse(&project, &file));
    self.input_vector.clear();
  }
}

pub fn generate(dirs: &[String]) {
  println!("Generating documentation...");
  let mut doc: Doc = Doc::new();
  dirs.into_iter().for_each(|dir: &String| doc.project(&format!("{dir}/manifest.tex")));
}

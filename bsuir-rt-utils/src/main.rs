mod commands;

fn main() {
  let args: Vec<String> = std::env::args().collect::<Vec<String>>();
  let command: &str = args[1].as_str();
  let project: &str = args[2].as_str();
  match command {
    "new" => commands::new(project),
    "build" => commands::build(project),
    _ => commands::invalid(command),
  };
}

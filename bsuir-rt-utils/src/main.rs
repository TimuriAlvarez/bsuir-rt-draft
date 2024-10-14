mod commands;

fn main() {
  let args: Vec::<String> = std::env::args().collect::<Vec::<String>>();
  let command: &str = args[1].as_str();
  let project: Option<&str> = args.get(2).map(|x: &String| x.as_str());
  match command {
    "new" => commands::new(project.expect("Project name must be provided")),
    "build" => commands::build(project.expect("Project name must be provided")),
    _ => commands::invalid(command),
  };
}

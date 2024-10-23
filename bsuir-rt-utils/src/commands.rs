mod doc;
mod latex;

const EXIT_CODE: i32 = 0x0C;
const BSUIR_RT: &str = "~/Public/bsuir-rt-draft/bsuir-rt";

const BSUIR_RT_DIRS: [&str; 4] = ["backend", "rules", "service", "frontend"];
const PROJECT_DIRS: [&str; 6] = ["contents", "data", "extras", "images", "pages", "sources"];

fn bsuir_rt() -> String {
  expanduser::expanduser(BSUIR_RT).expect("Failed to expand BSUIR_RT").to_string_lossy().to_string()
}

pub fn doxygen() {
  let bsuir_rt: String = bsuir_rt();
  let dirs: Vec::<String> = BSUIR_RT_DIRS.into_iter().map(|dir: &str| format!("{bsuir_rt}-{dir}/src")).collect();
  doc::generate(&dirs);
}

fn exit(message: &str) -> ! {
  eprintln!("{message}");
  std::process::exit(EXIT_CODE)
}

fn validate(project: &str) {
  if project.is_empty() {
    exit("Project name should not be empty");
  }
  if !project.chars().all(|c: char| c.is_alphanumeric() || c == '-') {
    exit("Project name should contain only alphanumeric characters");
  }
}

fn copy(source: &str, dest: &str) {
  if let Some(error) = dircpy::copy_dir(source, dest).err() {
    exit(&format!("Failed to copy {source} to {dest}: {error}"));
  }
}

fn link(src: &str, dst: &str) {
  symlink::symlink_auto(src, dst).expect(&format!("Failed to create symlink from {src} to {dst}"));
}

pub fn new(project: &str) {
  validate(project);
  if std::path::PathBuf::from(project).exists() {
    exit("Project already exists");
  }
  let bsuir_rt: String = bsuir_rt();
  copy(&format!("{bsuir_rt}-project/src"), project);
  PROJECT_DIRS.into_iter().for_each(|dir: &str| std::fs::create_dir(&format!("{project}/{dir}")).expect(&format!("Failed to create {project}/{dir} directory")));
}

pub fn build(project: &str) {
  validate(project);
  if !std::path::PathBuf::from(project).exists() {
    exit("Project does not exist");
  }
  if !std::path::PathBuf::from(format!("{project}/manifest.tex")).exists() {
    exit("Project manifest does not exist");
  }
  let build_dir: String = format!("{project}/build");
  if std::path::PathBuf::from(&build_dir).exists() {
    std::fs::remove_dir_all(&build_dir).expect("Failed to remove build directory");
  }
  std::fs::create_dir_all(&build_dir).expect("Failed to create build directory");
  let bsuir_rt: String = bsuir_rt();
  BSUIR_RT_DIRS.into_iter().for_each(|dir: &str| link(&format!("{bsuir_rt}-{dir}"), &format!("{build_dir}/{dir}")));
  latex::to_pdf(&project, &build_dir);
  std::fs::copy(format!("{build_dir}/manifest.pdf"), format!("{project}/{project}.pdf")).expect("Failed to copy manifest.pdf to project directory");
  std::fs::remove_dir_all(&build_dir).expect("Failed to remove build directory");
}

pub fn invalid(command: &str) {
  exit(&format!("{command} is not a valid command"));
}

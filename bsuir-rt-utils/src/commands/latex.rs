const PDF_LATEX: &str = "xelatex";
const L: char = '{';
const R: char = '}';

fn impl_path(project: &str) -> String {
  format!("\\def\\rtImplPath{L}{project}{R}")
}

fn include_module_manifest(build_dir: &str, subdir: &str) -> String {
  format!("\\input{L}{build_dir}/{subdir}/src/manifest{R}")
}

fn include_project_manifest(project: &str) -> String {
  format!("\\input{L}{project}/manifest{R}")
}

fn call_impl_manifest() -> String {
  format!("\\rtImplManifest{L}{R}\\undef\\rtImplPath")
}

fn pdf_latex(project: &str, build_dir: &str) {
  let manifest: String = impl_path(project) + &["backend", "rules", "service", "frontend"].into_iter().map(|s| include_module_manifest(build_dir, s)).collect::<Vec<String>>().join("") + &include_project_manifest(project) + &call_impl_manifest();
  let _ = std::process::Command::new(PDF_LATEX)
    .arg(format!("-output-directory={build_dir}"))
    .arg(manifest)
    .output()
    .expect(&format!("Failed to run {PDF_LATEX}"))
    .stdout;
}

fn make_index() {
  // TODO: implement
}

fn bibliographer() {
  // TODO: implement
}

pub fn to_pdf(project: &str, build_dir: &str) {
  pdf_latex(project, build_dir);
  make_index();
  bibliographer();
  pdf_latex(project, build_dir);
}

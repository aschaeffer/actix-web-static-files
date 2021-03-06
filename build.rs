#![doc(html_no_source)]
include!("src/impl.rs");
fn main() {
    resource_dir("./tests").build().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_filename = Path::new(&out_dir).join("generated_mapping.rs");
    generate_resources_mapping("./tests", None, generated_filename).unwrap();
}

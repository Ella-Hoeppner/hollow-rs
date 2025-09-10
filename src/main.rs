use hollow::sketch::Sketch;

fn main() {
  std::env::set_var("RUST_BACKTRACE", "1");
  // hollow::demos::SimpleSketch::new().run();
  // hollow::demos::VertexSketch::new().run();
  hollow::demos::CliffordSketch::new().run();
}

use hollow::sketch::Sketch;

fn main() {
  std::env::set_var("RUST_BACKTRACE", "1");
  hollow::demos::VertexSketch::run();
}

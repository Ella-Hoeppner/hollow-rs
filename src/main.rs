use hollow_rs::app::run_app;

fn main() {
  pollster::block_on(run_app());
}

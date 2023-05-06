mod long_lifetime1;
mod loop_borrow;

fn main() {
  long_lifetime1::run();
  loop_borrow::run();
}
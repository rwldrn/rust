// error-pattern:Declaration of none shadows a tag
import option::*;

fn main() {
  let none: int = 42;
  log(debug, none);
}
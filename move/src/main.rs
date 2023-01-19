fn main() {
  let hello : String = String::from("Hello");
  take(hello); // From take(): Hello WasmEdge!
  // The following will fail since hello is already taken by take() and no longer available here
  // println!("From main(): {}", hello);

  let hello : String = String::from("Hello");
  take(hello.clone()); // From take(): Hello WasmEdge!
  println!("From main(): {}", hello); // From main(): Hello

  let hello : String = String::from("Hello");
  borrow(&hello); // From borrow(): Hello WasmEdge!
  println!("From main(): {}", hello); // From main(): Hello

  let mut hello : String = String::from("Hello");
  borrow_mut(&mut hello); // From borrow_mut(): Hello WasmEdge!
  println!("From main(): {}", hello); // From main(): Hello WasmEdge!
}

fn take (mut s: String) {
  s.push_str(" WasmEdge!");
  println!("From take(): {}", s);
}

fn borrow (s: &String) {
  let mut buf = String::from(s);
  buf.push_str(" WasmEdge!");
  println!("From borrow(): {}", buf);
}

fn borrow_mut (s: &mut String) {
  (*s).push_str(" WasmEdge!");
  println!("From borrow_mut(): {}", s);
}

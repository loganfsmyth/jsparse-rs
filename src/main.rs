extern crate jsparse;


struct Foo {

}

impl Foo {
  fn lock(&mut self) -> FooLock {
    FooLock::new(self)
  }
  fn thing(&mut self) {
    println!("CONTENT");
  }
}

struct FooLock<'a> {
  f: &'a mut Foo,
}

impl<'a> FooLock<'a> {
  fn new(f: &'a mut Foo) -> FooLock {
    println!("(");
    FooLock { f }
  }
}

impl<'a> std::ops::Drop for FooLock<'a> {
  fn drop(&mut self) {
    println!(")");
  }
}

impl<'a> std::ops::Deref for FooLock<'a> {
  type Target = Foo;

  fn deref(&self) -> &Foo {
    self.f
  }
}
impl<'a> std::ops::DerefMut for FooLock<'a> {
  fn deref_mut(&mut self) -> &mut Foo {
    self.f
  }
}


fn main() {
  let mut f = Foo {};
  println!("[");
  {
    let mut f = f.lock();

    f.thing();
  }
  println!("]");
}

/* error */
struct Foo<'a> {
  bar: &'a mut Bar<'a>
}

impl<'a> Foo<'a> {
  pub fn log(self) {
    println!("bar in foo is: {}", self.bar.text);
  }
}

struct Bar<'a> {
  text: &'a str
}

struct Baz<'a> {
  bar: Bar<'a>
}

impl<'a> Baz<'a> {
  pub fn get_foo(&'a mut self) -> Foo<'a> {
    Foo {
      bar: &mut self.bar
    }
  }
}

/* passed */
// struct Foo<'b, 'a: 'b> {
//   bar: &'b mut Bar<'a>
// }

// impl<'b, 'a: 'b> Foo<'b, 'a> {
//   pub fn log(self) {
//     println!("bar in foo is: {}", self.bar.text);
//   }
// }

// struct Bar<'a> {
//   text: &'a str
// }

// struct Baz<'a> {
//   bar: Bar<'a>
// }

// impl<'a> Baz<'a> {
//   pub fn get_foo<'b>(&'b mut self) -> Foo<'b, 'a>
//   where 'a: 'b {
//     Foo {
//       bar: &mut self.bar
//     }
//   }
// }

pub fn run() {
  let mut baz = Baz {
    bar: Bar {
      text: "hello"
    }
  };

  baz.get_foo().log();

  println!("bar in baz is: {}", &baz.bar.text)
}
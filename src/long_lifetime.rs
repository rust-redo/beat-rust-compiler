/* error */
struct A<'a> {
  b: &'a mut B<'a>
}

impl<'a> A<'a> {
  pub fn log(self) {
    println!("b in a is: {}", self.b.s);
  }
}

struct B<'a> {
  s: &'a str
}

struct C<'a> {
  b: B<'a>
}

impl<'a> C<'a> {
  pub fn get_a(&'a mut self) -> A<'a> {
    A {
      b: &mut self.b
    }
  }
}

/* passed */
// struct A<'b, 'a: 'b> {
//   b: &'b mut B<'a>
// }

// impl<'b, 'a: 'b> A<'b, 'a> {
//   pub fn log(self) {
//     println!("b in a is: {}", self.b.s);
//   }
// }

// struct B<'a> {
//   s: &'a str
// }

// struct C<'a> {
//   b: B<'a>
// }

// impl<'a> C<'a> {
//   pub fn get_a<'b>(&'b mut self) -> A<'b, 'a>
//   where 'a: 'b {
//     A {
//       b: &mut self.b
//     }
//   }
// }

pub fn run() {
  let mut c = C {
    b: B {
      s: "hello"
    }
  };

  c.get_a().log();

  println!("c is: {}", &c.b.s)
}
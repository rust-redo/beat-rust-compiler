/* error */
// struct A {
//   a: i32
// }

// impl A {
//   fn one(&mut self) -> &i32 {
//     self.a += 10;
//     &self.a
//   }

//   fn two(&mut self) -> &i32 {
//     loop {
//       let k = self.one();
//       if *k > 50 {
//         return k
//       }
//     }
//   }
// }

/* passed */
struct A {
  a: i32,
}

impl A {
  fn one(&mut self) -> i32 {
      self.a += 10;
      self.a
  }

  fn two(&mut self) -> &i32 {
      let mut k = self.one();
      while k <= 50 {
          k = self.one();
      }
      &self.a
  }
}

pub fn run() {
  let mut a = A { a: 0 };

  a.one();

  a.two();
}
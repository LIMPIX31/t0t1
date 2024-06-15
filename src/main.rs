use toti;

trait Trait {}

#[toti::expand(10)]
macro_rules! vars {
  ($($T:ident),+) => {
    impl<$($T),+> Trait for ($($T,)+) {}
  };
}

fn main() {
    
}
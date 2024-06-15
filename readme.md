# T-Zero, T-One <sub><sub>aka toti</sub></sub>

## Example
```rust
#[toti::expand(10)]
macro_rules! vars {
  ($($T:ident),+) => {
    impl<$($T),+> Trait for ($($T,)+) {}
  };
}
```
Expands into
```rust
impl<T0> Trait for (T0,) {}
impl<T0, T1> Trait for (T0, T1) {}
impl<T0, T1, T2> Trait for (T0, T1, T2) {}
impl<T0, T1, T2, T3> Trait for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3, T4> Trait for (T0, T1, T2, T3, T4) {}
impl<T0, T1, T2, T3, T4, T5> Trait for (T0, T1, T2, T3, T4, T5) {}
impl<T0, T1, T2, T3, T4, T5, T6> Trait for (T0, T1, T2, T3, T4, T5, T6) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Trait for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Trait for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Trait for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {}
```

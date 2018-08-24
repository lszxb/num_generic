#[macro_use]
mod opt;

use opt::*;

fn main() {
    println!("1+2={}", <
            //  <S<Z> as Add<S<S<Z>>>>::Ret
            //  opt!(S<Z>, Add, S<S<Z>>)
             add!(S<Z>, S<S<Z>>)
             as Nat>::how());
    println!("3-2={}", <
            //  <S<S<S<Z>>> as Del<S<S<Z>>>>::Ret
             del!(S<S<S<Z>>>, S<S<Z>>)
             as Nat>::how());
    println!("3*2={}", <
            //  <S<S<S<Z>>> as Mul<S<S<Z>>>>::Ret
             mul!(S<S<S<Z>>>, S<S<Z>>)
             as Nat>::how());
}

pub struct Z;
pub struct S<T>(T);

pub trait Nat {
    fn how() -> u32;
}
impl Nat for Z {
    #[inline]
    fn how() -> u32 {
        0
    }
}
impl<T: Nat> Nat for S<T> {
    #[inline]
    fn how() -> u32 {
        T::how() + 1
    }
}


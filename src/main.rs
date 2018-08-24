fn main() {
    println!("Hello, world!");
    println!("1+2={}", <
             <S<Z> as Add<S<S<Z>>>>::Ret
             as Nat>::how());
    println!("3-2={}", <
             <S<S<S<Z>>> as Del<S<S<Z>>>>::Ret
             as Nat>::how());
}

struct Z;
struct S<T>(T);
trait Nat {
    fn how() -> u32;
}
impl Nat for Z {
    fn how() -> u32 {
        0
    }
}
impl<T: Nat> Nat for S<T> {
    fn how() -> u32 {
        T::how() + 1
    }
}

trait Add<T: Nat> : Nat {
    type Ret;
}
impl<T: Nat> Add<Z> for T {
    type Ret = T;
}
impl<T: Nat, U: Nat> Add<S<T>> for U 
    where S<U>: Add<T>
{
    type Ret = <S<U> as Add<T>>::Ret;
}

trait Del<T: Nat> : Nat {
    type Ret;
}
impl<T: Nat> Del<Z> for T {
    type Ret = T;
}
impl<T: Nat, U: Nat + Del<T>> Del<S<T>> for S<U> {
    type Ret = <U as Del<T>>::Ret;
}


use ::{Nat, S, Z};

macro_rules! opt {
    ($t1:ty, $p:ident,$t2:ty) => {
        <$t1 as $p<$t2>>::Ret
    };
}

macro_rules! add {
    ($t1:ty, $t2:ty) => {
        opt!($t1, Add, $t2)
    };
}

macro_rules! del {
    ($t1:ty, $t2:ty) => {
        opt!($t1, Del, $t2)
    };
}

macro_rules! mul {
    ($t1:ty, $t2:ty) => {
        opt!($t1, Mul, $t2)
    };
}

pub trait Add<T: Nat> : Nat {
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

pub trait Del<T: Nat> : Nat {
    type Ret;
}
impl<T: Nat> Del<Z> for T {
    type Ret = T;
}
impl<T: Nat, U: Nat + Del<T>> Del<S<T>> for S<U> {
    type Ret = <U as Del<T>>::Ret;
}

pub trait Mul<T: Nat> : Nat {
    type Ret;
}
impl<T: Nat> Mul<Z> for T {
    type Ret = Z;
}
impl<T: Nat, U: Nat + Mul<T>> Mul<S<T>> for U
    where <U as Mul<T>>::Ret: Add<U>
{
    type Ret = <<U as Mul<T>>::Ret as Add<U>>::Ret;
}

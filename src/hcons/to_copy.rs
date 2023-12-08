use frunk::{HCons, HNil};

pub trait ToCopy<'a> {
    type Output;

    fn to_copy(&'a self) -> Self::Output;
}

impl<'a> ToCopy<'a> for HNil {
    type Output = HNil;

    #[inline(always)]
    fn to_copy(&'a self) -> Self::Output {
        HNil
    }
}

impl<'a, T, Tail> ToCopy<'a> for HCons<&T, Tail>
where
    T: Copy,
    Tail: ToCopy<'a>,
{
    type Output = HCons<T, <Tail as ToCopy<'a>>::Output>;

    #[inline(always)]
    fn to_copy(&'a self) -> Self::Output {
        HCons {
            head: *self.head,
            tail: (&self.tail).to_copy(),
        }
    }
}

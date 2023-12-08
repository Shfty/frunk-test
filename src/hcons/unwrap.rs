use frunk::{HCons, HNil};

pub trait Unwrap {
    type Output;

    fn unwrap(self) -> Self::Output;
}

impl Unwrap for HNil {
    type Output = HNil;

    #[inline(always)]
    fn unwrap(self) -> Self::Output {
        HNil
    }
}

impl<'a, H, Tail> Unwrap for HCons<Option<H>, Tail>
where
    Tail: Unwrap,
{
    type Output = HCons<H, <Tail as Unwrap>::Output>;

    #[inline(always)]
    fn unwrap(self) -> Self::Output {
        HCons {
            head: self.head.unwrap(),
            tail: self.tail.unwrap(),
        }
    }
}

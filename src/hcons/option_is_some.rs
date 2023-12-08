use frunk::{HCons, HNil};

pub trait OptionIsSome<'a> {
    type Output;

    fn is_some(&'a self) -> Self::Output;
}

impl<'a> OptionIsSome<'a> for HNil {
    type Output = HNil;

    #[inline(always)]
    fn is_some(&'a self) -> Self::Output {
        HNil
    }
}

impl<'a, H, Tail> OptionIsSome<'a> for HCons<&'a Option<H>, Tail>
where
    Tail: OptionIsSome<'a>,
{
    type Output = HCons<bool, <Tail as OptionIsSome<'a>>::Output>;

    #[inline(always)]
    fn is_some(&'a self) -> Self::Output {
        HCons {
            head: self.head.is_some(),
            tail: (&self.tail).is_some(),
        }
    }
}

#[cfg(test)]
mod tests {
    use frunk::hlist;

    use super::*;

    #[test]
    fn map_unwrap() {
        let list = hlist![Some(true), Option::<f32>::None, Some(3)];
        let is_some = list.to_ref().is_some();

        println!("Is Some:\n{:#?}", is_some);
    }
}

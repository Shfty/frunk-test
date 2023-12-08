use frunk::{HCons, HNil};

pub trait ReplaceWith<T>
where
    T: Copy,
{
    type Output;

    fn replace_with(&self, t: T) -> Self::Output;
}

impl<T> ReplaceWith<T> for HNil
where
    T: Copy,
{
    type Output = HNil;

    #[inline(always)]
    fn replace_with(&self, _: T) -> Self::Output {
        HNil
    }
}

impl<H, Tail, T> ReplaceWith<T> for HCons<H, Tail>
where
    Tail: ReplaceWith<T>,
    T: Copy,
{
    type Output = HCons<T, <Tail as ReplaceWith<T>>::Output>;

    #[inline(always)]
    fn replace_with(&self, t: T) -> Self::Output {
        HCons {
            head: t,
            tail: (&self.tail).replace_with(t),
        }
    }
}

#[cfg(test)]
mod tests {
    use frunk::hlist;

    use super::*;

    #[test]
    fn replace_with() {
        let list = hlist![true, 1.0, 3];
        let list = ReplaceWith::replace_with(&list.to_ref(), "banana");

        println!("usize default:\n{:#?}", list);
    }
}

use std::marker::PhantomData;

use frunk::{Func, Poly};

#[derive(Default)]
pub struct MapOption<F>(PhantomData<F>);

#[allow(dead_code)]
impl<F> MapOption<F>
where
    F: Default,
{
    pub fn mapper() -> Poly<Self> {
        Default::default()
    }
}

impl<'a, From, F> Func<&'a Option<&'a From>> for MapOption<F>
where
    From: 'a,
    F: Func<&'a From>,
{
    type Output = Option<<F as Func<&'a From>>::Output>;

    fn call(option: &'a Option<&'a From>) -> Self::Output {
        option.as_ref().map(|val| F::call(val))
    }
}

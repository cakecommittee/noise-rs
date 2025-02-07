use crate::noise_fns::NoiseFn;
use core::marker::PhantomData;

/// Noise function that negates the output value from the source function.
#[derive(Clone, Debug)]
pub struct Negate<T, Source, const DIM: usize>
where
    Source: NoiseFn<T, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    phantom: PhantomData<T>,
}

impl<T, Source, const DIM: usize> Negate<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    pub fn new(source: Source) -> Self {
        Negate {
            source,
            phantom: PhantomData,
        }
    }
}

impl<T, Source, const DIM: usize> NoiseFn<T, DIM> for Negate<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        -self.source.get(point)
    }
}

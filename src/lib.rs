#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

#[macro_export]
macro_rules! iter_compr {
    ($exp:expr ; $($tt:tt)*) => {
        $crate::GeneratorIterator::new(|| {
            iter_compr_internal!($exp; $($tt)*)
        })
    };
}

#[macro_export]
macro_rules! iter_compr_internal {
    ($exp:expr; ) => {
        yield $exp;
    };

    ($exp:expr; let $pat:pat = $exp2:expr) => {
        {
            let $pat = $exp2;
            iter_compr_internal!($exp;)
        }
    };

    ($exp:expr; let $pat:pat = $exp2:expr, $($tt:tt)*) => {
        {
            let $pat = $exp2;
            iter_compr_internal!($exp; $($tt)*)
        }
    };

    ($exp:expr; $pat:pat in $exp2:expr) => {
        for $pat in $exp2 {
            iter_compr_internal!($exp;)
        }
    };

    ($exp:expr; $pat:pat in $exp2:expr, $($tt:tt)*) => {
        for $pat in $exp2 {
            iter_compr_internal!($exp; $($tt)*)
        }
    };

    ($exp:expr; $guard:expr) => {
        if $guard {
            iter_compr_internal!($exp;)
        }
    };

    ($exp:expr; $guard:expr, $($tt:tt)*) => {
        if $guard {
            iter_compr_internal!($exp; $($tt)*)
        }
    };
}

pub struct GeneratorIterator<G> {
    gen: G,
}

impl<G> GeneratorIterator<G> {
    pub fn new(gen: G) -> Self {
        Self { gen }
    }
}

impl<G, T> Iterator for GeneratorIterator<G>
where
    G: Generator<Yield = T, Return = ()>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let GeneratorState::Yielded(v) = unsafe { self.gen.resume() } {
            Some(v)
        } else {
            None
        }
    }
}

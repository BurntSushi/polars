use polars_utils::float::IsFloat;
use polars_utils::ord::{compare_fn_nan_max, compare_fn_nan_min};

pub trait ExtremaNanAware<T> {
    fn min_value_nan_aware(&self) -> Option<&T>;
    fn max_value_nan_aware(&self) -> Option<&T>;
}

impl<T: PartialOrd + IsFloat> ExtremaNanAware<T> for [T] {
    fn min_value_nan_aware(&self) -> Option<&T> {
        self.iter().min_by(|a, b| compare_fn_nan_max(*a, *b))
    }

    fn max_value_nan_aware(&self) -> Option<&T> {
        self.iter().max_by(|a, b| compare_fn_nan_min(*a, *b))
    }
}

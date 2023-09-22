mod zip_outer;

pub(crate) use zip_outer::*;

use crate::prelude::*;

pub fn _join_suffix_name(name: &str, suffix: &str) -> String {
    format!("{name}{suffix}")
}

/// Utility method to finish a join.
#[doc(hidden)]
pub(crate) fn _finish_join(
    mut df_left: DataFrame,
    mut df_right: DataFrame,
    suffix: Option<&str>,
) -> PolarsResult<DataFrame> {
    let mut left_names = PlHashSet::with_capacity(df_left.width());

    df_left.columns.iter().for_each(|series| {
        left_names.insert(series.name());
    });

    let mut rename_strs = Vec::with_capacity(df_right.width());

    df_right.columns.iter().for_each(|series| {
        if left_names.contains(series.name()) {
            rename_strs.push(series.name().to_owned())
        }
    });
    let suffix = suffix.unwrap_or("_right");

    for name in rename_strs {
        df_right.rename(&name, &_join_suffix_name(&name, suffix))?;
    }

    drop(left_names);
    df_left.hstack_mut(&df_right.columns)?;
    Ok(df_left)
}

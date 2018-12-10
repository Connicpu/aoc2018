use std::str::FromStr;

pub fn get_columns<'a>(
    line: &'a str,
    separator: impl FnMut(char) -> bool,
) -> impl Iterator<Item = &'a str> {
    line.split(separator)
        .filter_map(|c| if c.is_empty() { None } else { Some(c) })
}

pub fn parse_columns<'a, T: FromStr + 'a>(
    line: &'a str,
    separator: impl FnMut(char) -> bool + 'a,
) -> impl Iterator<Item = T> + 'a {
    get_columns(line, separator)
        .map(FromStr::from_str)
        .filter_map(|v| v.ok())
}

#[macro_export]
macro_rules! extract_columns {
    ([$($dummy:ident),*]) => {
        |mut cols| Some([
            $({
                let $dummy = ();
                let _ = $dummy;
                cols.next()?
            },)*
        ])
    };
    (($($dummy:ident),*)) => {
        |mut cols| Some((
            $({
                let $dummy = ();
                let _ = $dummy;
                cols.next()?
            },)*
        ))
    };
}

#[macro_export]
macro_rules! collect_once {
    (let data: $ty:ty = $iter:expr) => {{
        use lazy_static::lazy_static;
        lazy_static! {
            static ref DATA: Vec<$ty> = $iter.collect();
        }

        DATA.iter().cloned()
    }};
}

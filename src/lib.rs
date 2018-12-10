use std::str::FromStr;

pub fn get_columns<'a>(
    line: &'a str,
    separator: impl FnMut(char) -> bool,
) -> impl Iterator<Item = &'a str> {
    line.split(separator)
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

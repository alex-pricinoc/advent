use std::any;
use std::path::Path;

pub fn set(s: &str) {
    crate::Timer::start();
    *crate::INPUT.lock().unwrap() = Some(Box::leak(s.into()));
}

pub fn raw_string() -> &'static str {
    crate::INPUT.lock().unwrap().get_or_insert_with(|| {
        let mut args = std::env::args_os();
        let input = args.nth(1).unwrap();
        let infile = Path::new(&input);
        crate::Timer::start();
        Box::leak(
            std::fs::read_to_string(infile)
                .unwrap_or_else(|err| {
                    panic!(
                        "could not read input file '{infile}': {err}",
                        infile = infile.display()
                    )
                })
                .into(),
        )
    })
}

#[must_use]
pub fn string() -> &'static str {
    raw_string().trim_end()
}

pub fn lines() -> impl Iterator<Item = &'static str> {
    string()
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.is_empty())
}

pub fn parse_lines<T: InputItem>() -> impl Iterator<Item = T> {
    lines().map(|line| {
        T::read_part(&mut line.split_whitespace()).unwrap_or_else(|| {
            panic!(
                "line {:?} failed to convert to {}",
                line,
                any::type_name::<T>()
            )
        })
    })
}

#[allow(clippy::module_name_repetitions)]
pub trait InputItem
where
    Self: Sized,
{
    fn read_part(tok: &mut impl Iterator<Item = &'static str>) -> Option<Self>;
}

macro_rules! simple_impl {
    ( $($ty:ty)+ ) => {
        $(
            impl InputItem for $ty {
                fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
                    tok.next()?.trim_matches(&[',', ':'][..]).parse().ok()
                }
            }
        )+
    }
}

simple_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool);

impl<T> InputItem for Vec<T>
where
    T: InputItem,
{
    fn read_part(tok: &mut impl Iterator<Item = &'static str>) -> Option<Self> {
        let mut result = vec![];

        while let Some(item) = T::read_part(tok) {
            result.push(item);
        }

        Some(result)
    }
}

macro_rules! tuple_impl {
    ( $($tys:ident),+ ) => {
        impl<$($tys: InputItem),+> InputItem for ($($tys),+ ,) {
            #[allow(non_snake_case)]
            fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
                let ( $($tys),+, ) = ( $( $tys::read_part(tok) ),+, );

                Some(( $( $tys? ),+, ))
            }
        }
    }
}

tuple_impl!(T);
tuple_impl!(T, U);
tuple_impl!(T, U, V);
tuple_impl!(T, U, V, W);
tuple_impl!(T, U, V, W, Y);
tuple_impl!(T, U, V, W, Y, Z);
tuple_impl!(T, U, V, W, Y, Z, T1);
tuple_impl!(T, U, V, W, Y, Z, T1, T2);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4, T5);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4, T5, T6);

macro_rules! array_impl {
    ( $ty:ident, $n:expr, $($qm:tt)+) => {
        impl<$ty: InputItem> InputItem for [$ty; $n] {
            fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
                Some([$( $ty::read_part(tok) $qm ),+])
            }
        }
    }
}

array_impl!(T, 1, ?);
array_impl!(T, 2, ??);
array_impl!(T, 3, ???);
array_impl!(T, 4, ????);
array_impl!(T, 5, ?????);
array_impl!(T, 6, ??????);
array_impl!(T, 7, ???????);
array_impl!(T, 8, ????????);
array_impl!(T, 9, ?????????);

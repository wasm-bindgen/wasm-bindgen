use std::{ops::Deref, str};

pub trait Decode<'src>: Sized {
    fn decode(data: &mut &'src [u8]) -> Self;

    fn decode_all(mut data: &'src [u8]) -> Self {
        let ret = Self::decode(&mut data);
        assert!(data.is_empty());
        ret
    }
}

pub struct LitOrExpr<'src> {
    str: &'src str,
}

fn get(b: &mut &[u8]) -> u8 {
    let r = b[0];
    *b = &b[1..];
    r
}

impl Deref for LitOrExpr<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.str
    }
}

impl<'src> Decode<'src> for LitOrExpr<'src> {
    fn decode(data: &mut &'src [u8]) -> Self {
        let str = <&'src str>::decode(data);
        Self { str }
    }
}

impl<'src> Decode<'src> for bool {
    fn decode(data: &mut &'src [u8]) -> Self {
        get(data) != 0
    }
}

impl<'src> Decode<'src> for u32 {
    fn decode(data: &mut &'src [u8]) -> Self {
        let mut cur = 0;
        let mut offset = 0;
        loop {
            let byte = get(data);
            cur |= ((byte & 0x7f) as u32) << offset;
            if byte & 0x80 == 0 {
                break cur;
            }
            offset += 7;
        }
    }
}

impl<'src> Decode<'src> for &'src str {
    fn decode(data: &mut &'src [u8]) -> &'src str {
        let n = u32::decode(data);
        let (a, b) = data.split_at(n as usize);
        *data = b;
        let r = str::from_utf8(a).unwrap();
        log::trace!("decoded string {r:?}");
        r
    }
}

impl<'src> Decode<'src> for String {
    fn decode(data: &mut &'src [u8]) -> String {
        <&'src str>::decode(data).to_string()
    }
}

impl<'src, T: Decode<'src>> Decode<'src> for Vec<T> {
    fn decode(data: &mut &'src [u8]) -> Self {
        let n = u32::decode(data);
        let mut v = Vec::with_capacity(n as usize);
        log::trace!("found a list of length {n}");
        for _ in 0..n {
            v.push(Decode::decode(data));
        }
        v
    }
}

impl<'src, T: Decode<'src>> Decode<'src> for Option<T> {
    fn decode(data: &mut &'src [u8]) -> Self {
        match get(data) {
            0 => None,
            1 => Some(Decode::decode(data)),
            _ => unreachable!(),
        }
    }
}

macro_rules! decode_struct {
    ($name:ident ($($lt:tt)*) $($field:ident: $ty:ty,)*) => {
        pub struct $name <$($lt)*> {
            $(pub $field: $ty,)*
        }

        impl <'a> Decode<'a> for $name <$($lt)*> {
            fn decode(_data: &mut &'a [u8]) -> Self {
                log::trace!("start decode `{}`", stringify!($name));
                $name {
                    $($field: Decode::decode(_data),)*
                }
            }
        }
    }
}

macro_rules! decode_enum {
    ($name:ident ($($lt:tt)*) $($fields:tt)*) => (
        pub enum $name <$($lt)*> { $($fields)* }

        impl <'a> Decode<'a> for $name <$($lt)*> {
            fn decode(data: &mut &'a [u8]) -> Self {
                use self::$name::*;
                decode_enum!(@arms data dst (0) () $($fields)*)
            }
        }
    );

    (@arms $data:ident $dst:ident ($cnt:expr) ($($arms:tt)*)) => (
        decode_enum!(@expr match get($data) { $($arms)* _ => unreachable!() })
    );

    (@arms $data:ident $dst:ident ($cnt:expr) ($($arms:tt)*) $name:ident, $($rest:tt)*) => (
        decode_enum!(
            @arms
            $data
            $dst
            ($cnt+1)
            ($($arms)* n if n == $cnt => $name, )
            $($rest)*
        )
    );

    (@arms $data:ident $dst:ident ($cnt:expr) ($($arms:tt)*) $name:ident($t:ty), $($rest:tt)*) => (
        decode_enum!(
            @arms
            $data
            $dst
            ($cnt+1)
            ($($arms)* n if n == $cnt => $name(Decode::decode($data)), )
            $($rest)*
        )
    );

    (@expr $e:expr) => ($e);
}

macro_rules! decode_api {
    () => ();
    (struct $name:ident<'a> { $($fields:tt)* } $($rest:tt)*) => (
        decode_struct!($name ('a) $($fields)*);
        decode_api!($($rest)*);
    );
    (struct $name:ident { $($fields:tt)* } $($rest:tt)*) => (
        decode_struct!($name () $($fields)*);
        decode_api!($($rest)*);
    );
    (enum $name:ident<'a> { $($variants:tt)* } $($rest:tt)*) => (
        decode_enum!($name ('a) $($variants)*);
        decode_api!($($rest)*);
    );
    (enum $name:ident { $($variants:tt)* } $($rest:tt)*) => (
        decode_enum!($name () $($variants)*);
        decode_api!($($rest)*);
    );
}

wasm_bindgen_shared::shared_api!(decode_api);

#![warn(clippy::disallowed_format_specs)]

pub struct Address;

macro_rules! impl_fmt {
    ($($t:ident),*) => {$(
        impl std::fmt::$t for Address {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                unimplemented!()
            }
        }
    )*}
}

impl_fmt!(Display, Debug, LowerHex, UpperHex);

fn main() {
    let x = &&(&&(Address));

    let _ = format!("{}", Address);
    let _ = format!("{:#}", Address);
    let _ = format!("{:?}", Address);
    //~^ ERROR: format trait `Debug` is not allowed for type `Address` according to config
    let _ = format!("{:x}", Address);
    //~^ ERROR: format trait `LowerHex` is not allowed for type `Address` according to config
    let _ = format!("{:#x}", Address);
    //~^ ERROR: format trait `LowerHex` is not allowed for type `Address` according to config
    let _ = format!("{:X}", Address);
    //~^ ERROR: format trait `UpperHex` is not allowed for type `Address` according to config
    let _ = format!("{:#X}", Address);
    //~^ ERROR: format trait `UpperHex` is not allowed for type `Address` according to config

    let _ = format!("{}", x);
    let _ = format!("{:#}", x);
    let _ = format!("{:?}", x);
    //~^ ERROR: format trait `Debug` is not allowed for type `Address` according to config
    let _ = format!("{:x}", x);
    //~^ ERROR: format trait `LowerHex` is not allowed for type `Address` according to config
    let _ = format!("{:#x}", x);
    //~^ ERROR: format trait `LowerHex` is not allowed for type `Address` according to config
    let _ = format!("{:X}", x);
    //~^ ERROR: format trait `UpperHex` is not allowed for type `Address` according to config
    let _ = format!("{:#X}", x);
    //~^ ERROR: format trait `UpperHex` is not allowed for type `Address` according to config

    let _ = format!("{x}");
    let _ = format!("{x:#}");
    let _ = format!("{x:?}");
    //~^ ERROR: format trait `Debug` is not allowed for type `Address` according to config
    let _ = format!("{x:x}");
    //~^ ERROR: format trait `LowerHex` is not allowed for type `Address` according to config
    let _ = format!("{x:#x}");
    //~^ ERROR: format trait `LowerHex` is not allowed for type `Address` according to config
    let _ = format!("{x:X}");
    //~^ ERROR: format trait `UpperHex` is not allowed for type `Address` according to config
    let _ = format!("{x:#X}");
    //~^ ERROR: format trait `UpperHex` is not allowed for type `Address` according to config
}

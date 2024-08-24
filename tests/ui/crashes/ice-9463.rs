fn main() {
    let _x = -1_i32 >> -1;
    let _y = 1u32 >> 10000000000000u32;
    //~^ ERROR: literal out of range for `u32`
    //~| NOTE: the literal `10000000000000u32` does not fit into the type `u32` whose range is
}

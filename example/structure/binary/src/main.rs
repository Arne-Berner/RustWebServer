// any function used, needs this function block or a mod
use crate::test2::test2;


// the other mods need to be declared in main
mod test;
mod test2;
mod test3;

fn main() {
    //this is proper rust syntax for used functions
    test::test1();
    test2();
    test3::usingtest1and2();
    print!("{}", library::libmod::libtest());
    library::uselib();
}



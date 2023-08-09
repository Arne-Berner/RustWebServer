mod submodule1;
mod submodule2;

pub fn test2(){
    print!("this is test2\n");
    submodule1::submodule1();
    submodule2::submodule2();
}

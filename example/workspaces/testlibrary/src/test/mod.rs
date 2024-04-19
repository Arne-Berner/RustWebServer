mod submodule1;
mod submodule2;

pub fn test(){
    print!("this is test\n");
    submodule1::submodule1();
    submodule2::submodule2();
}

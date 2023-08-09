//used mods from this lib are declared here.

use libmod::libtest;

pub fn uselib(){
    libtest();
}

pub mod libmod {
    pub fn libtest() -> String {
        print!("Libtest running!");
        format!("Libstring")
    }
}

#[cfg(test)]
mod tests {
    use crate::libmod::libtest;

    #[test]
    fn test_libtest() {
        let teststring = "Libstring".to_string();
        assert_eq!(teststring, libtest());
    }
}
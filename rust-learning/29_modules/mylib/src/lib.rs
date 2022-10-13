pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod lib_person {
    fn getlibname() {
        println!("my lib person name");
    }

    pub fn getlibage(age:i32) {
        println!("my lib person age {}",age);
    }
}
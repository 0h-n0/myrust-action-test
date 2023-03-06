use num_cpus;

pub fn get() -> usize {
    return num_cpus::get();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("There are {} cpus", get());
        assert_eq!(4, 4);
    }
}

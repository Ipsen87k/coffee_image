#[cfg(test)]
mod tests {
    use rand::{
        distributions::{Alphanumeric, Uniform},
        thread_rng, Rng,
    };
    #[test]
    fn bitwise_not() {
        let mut list = vec![1, 3, 4, 2, -2];
        list.reverse();
        for x in &list {
            println!("{}", x);
        }
    }
    #[test]
    fn random_generate_words() ->Result<(),()> {
        let rand_string: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

        for x in rand_string {
            println!("{}", x);
        }
        Ok(())
    }
}

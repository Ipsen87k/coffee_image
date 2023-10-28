use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

const NAME_LENGTH: usize = 8;

pub fn generate_strings() -> String {
    let mut rng = rand::thread_rng();

    let file_name: String = (0..NAME_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());

            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect();

    file_name
}

use rand::Rng;

#[napi]
pub const L_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyz";
const LU_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[napi]
pub fn secure_rndstr(length: Option<u32>, chars: Option<&str>) -> String {
    let length = length.unwrap_or(32);
    let chars_str = chars.unwrap_or(LU_CHARS);
    let chars_len = chars_str.len();
    let mut rng = rand::thread_rng();
    let mut str_buf = String::with_capacity(length as usize);

    for _ in 0..length {
        let rand = rng.gen_range(0..chars_len);
        let rand_char = chars_str.as_bytes()[rand] as char;
        str_buf.push(rand_char);
    }

    str_buf
}

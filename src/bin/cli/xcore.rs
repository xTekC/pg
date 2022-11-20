use passgen::config::{lower::r_lower, num::r_num, sym::r_sym, upper::r_upper};

pub async fn xcore_main() -> String {
    let s = r_sym().await;
    let l = r_lower().await;
    let n = r_num().await;
    let u = r_upper().await;
    s.to_owned() + l + n + u
}

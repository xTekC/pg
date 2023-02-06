use passgen::xcore::chars::{r_lower, r_num, r_sym, r_upper};

pub async fn xcore_main() -> String {
    let z = "".to_owned();
    let s = r_sym().await;
    let l = r_lower().await;
    let n = r_num().await;
    let u = r_upper().await;
    z + s + l + n + u
}

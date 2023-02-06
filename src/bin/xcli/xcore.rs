use passgen::xcore::chars::{r_lower, r_num, r_sym, r_upper};

pub async fn xcore_main() -> String {
    let z = "".to_owned();
    let l = r_lower().await;
    let u = r_upper().await;
    let n = r_num().await;
    let s = r_sym().await;
    z + l + u + n + s
}

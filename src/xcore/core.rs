/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

pub async fn core_main() -> String {
    let z = "".to_owned();
    let l = r_lower().await;
    let u = r_upper().await;
    let n = r_num().await;
    let s = r_sym().await;
    z + l + u + n + s
}

async fn r_lower() -> &'static str {
    "abcdefghijklmnopqrstuvwxyz"
}

async fn r_upper() -> &'static str {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
}

async fn r_num() -> &'static str {
    "0123456789"
}

async fn r_sym() -> &'static str {
    "`~!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?"
}

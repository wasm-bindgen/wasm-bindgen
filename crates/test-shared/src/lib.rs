#![doc(html_root_url = "https://docs.rs/wasm-bindgen-test-shared/0.2")]
#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};

pub fn coverage_path(env: Option<&str>, pid: u32, tmpdir: &str, module_signature: u64) -> String {
    let env = env.unwrap_or("default_%m_%p.profraw");

    let mut path = String::new();
    let mut chars = env.chars().enumerate().peekable();

    while let Some((index, char)) = chars.next() {
        if char != '%' {
            path.push(char);
            continue;
        }

        if chars.next_if(|(_, c)| *c == 'p').is_some() {
            path.push_str(&pid.to_string())
        } else if chars.next_if(|(_, c)| *c == 'h').is_some() {
            path.push_str("wbgt")
        } else if chars.next_if(|(_, c)| *c == 't').is_some() {
            path.push_str(tmpdir)
        } else {
            let mut last_index = index;

            loop {
                if let Some((index, _)) = chars.next_if(|(_, c)| c.is_ascii_digit()) {
                    last_index = index;
                } else if chars.next_if(|(_, c)| *c == 'm').is_some() {
                    path.push_str(&module_signature.to_string());
                    path.push_str("_0");
                    break;
                } else {
                    path.push_str(&env[index..=last_index]);
                    break;
                }
            }
        }
    }

    path
}

#[test]
fn test() {
    fn asssert<'a>(env: impl Into<Option<&'a str>>, result: &str) {
        assert_eq!(coverage_path(env.into(), 123, "tmp", 456), result);
    }

    asssert(None, "default_456_0_123.profraw");
    asssert("", "");
    asssert("%p", "123");
    asssert("%h", "wbgt");
    asssert("%t", "tmp");
    asssert("%m", "456_0");
    asssert("%0123456789m", "456_0");
    asssert("%", "%");
    asssert("%%", "%%");
    asssert("%a", "%a");
    asssert("%0123456789", "%0123456789");
    asssert("%0123456789p", "%0123456789p");
    asssert("%%p", "%123");
    asssert("%%%p", "%%123");
    asssert("%a%p", "%a123");
    asssert("%0123456789%p", "%0123456789123");
    asssert("%p%", "123%");
    asssert("%p%%", "123%%");
    asssert("%p%a", "123%a");
    asssert("%p%0123456789", "123%0123456789");
    asssert("%p%0123456789p", "123%0123456789p");
    asssert("%m%a", "456_0%a");
    asssert("%m%0123456789", "456_0%0123456789");
    asssert("%m%0123456789p", "456_0%0123456789p");
    asssert("%0123456789m%a", "456_0%a");
    asssert("%0123456789m%0123456789", "456_0%0123456789");
    asssert("%0123456789m%0123456789p", "456_0%0123456789p");
}

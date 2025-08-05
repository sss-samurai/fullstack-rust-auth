// use web_sys::window;

// pub fn set_cookie(name: &str, value: &str) {
//     if let Some(document) = window().and_then(|w| w.document()) {
//         let cookie = format!("{}={}; path=/", name, value);
//         document.set_cookie(&cookie).expect("Failed to set cookie");
//     }
// }

// pub fn get_cookie(name: &str) -> Option<String> {
//     if let Some(document) = window().and_then(|w| w.document()) {
//         let all = document.cookie().ok()?;
//         for cookie in all.split(';') {
//             let cookie = cookie.trim();
//             if cookie.starts_with(&(name.to_owned() + "=")) {
//                 return Some(cookie[(name.len() + 1)..].to_string());
//             }
//         }
//     }
//     None
// }
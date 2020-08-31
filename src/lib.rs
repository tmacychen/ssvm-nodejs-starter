use wasm_bindgen::prelude::*;

#[wasm_bindgen]
//pub fn say(s: &str) -> String {
//  println!("The Rust function say() received {}", s);
//  let r = String::from("hello ");
//  return r + s;
//}
pub fn solve(params: &str) -> String {
    let ps: (f32, f32, f32) = serde_json::from_str(&params).unwrap();
    let discriminant: f32 = (ps.1 * ps.1) - (4. * ps.0 * ps.2);
    let mut solution: (f32, f32) = (0., 0.);
    if discriminant >= 0. {
        solution.0 = (((-1.) * ps.1) + discriminant.sqrt()) / (2. * ps.0);
        solution.1 = (((-1.) * ps.1) - discriminant.sqrt()) / (2. * ps.0);
        return serde_json::to_string(&solution).unwrap();
    } else {
        return String::from("not real numbers");
    }
}

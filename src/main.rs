
fn add(a:i64,b:i32) -> i64 {
    a + b as i64
}

fn main() {
    println!("The Add结果是:{}", add(100i64,1000));
}

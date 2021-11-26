extern crate time;

fn main() {
    let d = time::now();
	println!("Hoje é {}/{}/{}", d.tm_mday
	,d.tm_mon + 1, d.tm_year + 1900);
}

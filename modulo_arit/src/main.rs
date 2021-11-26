mod calculo{
	pub fn somar(a:i32, b:i32) -> i32{
		a+b
	}
	pub fn subtrair(a:i32, b:i32) -> i32{
		a-b
	}
	pub fn dividir(a:i32, b:i32) -> i32{
		a/b
	}
	pub fn multiplicar(a:i32, b:i32) -> i32{
		a*b
	}
}


fn main() {
    println!("Calculadora:");
	println!("Soma 1 + 2 = {}", calculo::somar(1,2));
	println!("Subtração 10 - 2 = {}", calculo::subtrair(10,2));
	println!("Divisão 20 / 2 = {}", calculo::dividir(20,2));
	println!("Multiplicação 10 * 10 = {}", calculo::multiplicar(10,10));
}

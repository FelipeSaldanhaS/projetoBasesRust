fn numero_magico(b:i32) -> i32{
	let c: i32 = 123;
	b+c
}

fn main() {
    println!("ALOCAÇÃO	E GERENCIAMENTO	DE MEMÓRIA");

	println!("\n-----ESCOPO DE VARIAVEIS-----\n");
	let a: i32 = 77;
	
	println!("\nSoma retornada da função: {}",numero_magico(a));
	
}

#[allow(unused_variables)]
#[allow(dead_code)]
mod condicionais {
	pub fn ifcomvariavel(){
		let z = if 10/2 == 5 {
			"10/2 é igual a 5"
		}else{
			"10/2 não é igual a 5"
		};
		println!("{}", z);
	}
	pub fn checa_notas(nota: f32) -> (){
		if nota >= 0.0 && nota < 4.9{
			println!("Não aprovado!");
		} else if nota >= 4.9 && nota < 6.0{
			println!("Exame!");
		} else if nota >= 6.0 {
			println!("Aprovado!");
		}
		else{
			println!("Nota inválida!");
		}
	}
	pub fn checa_notas_match(nota: f32) -> (){
		match nota{
			0.0 ...4.8 => println!("Não aprovado!"),
			4.9 ...5.9 => println!("Exame!"),
			6.0 ...10.0 => println!("Aprovado!"),
			_ => println!("Não aprovado!"),
		}
	}
	pub fn checa_numero(numero:i16) -> (){
		match numero{
			0 => println!("Zero"),
			2 | 3 | 5 | 7 | 11 => println!("Primo"),
			_ => println!("Qualquer número"),
		}
	}
	pub fn checa_tuplas(t:(i32,i32)) -> (){
		match t {
			(0, _x) => println!("Zero no primeiro"),
			(_x, 0) => println!("Zero no segundo"),
			_ => println!("Não tem zero")
		}
	}
	pub fn vogal_ou_consoante(c:char) -> char{
		match c{
			'a' | 'e' |'i' |'o'| 'u' => 'v',
			'A' | 'E' |'I' |'O'| 'U' => 'v',
			_ => 'c'
		}
	}
	pub fn entre_dez_e_um_e_menor_que_cem(a:i32)->(){
		if a>=1 && a<=10 && a<100{
			println!("Está entre um e dez e é menor que 100")
		}
			else if a<100{
				println!("Não está entre 1 e 10 e é menor que 100")
			}
				else{
					println!("Não está entre 1 e 10 e não é menor que 100")
				}
	}
	pub fn entre_dez_e_um_e_menor_que_cem_match(a:i32)->(){
		match a{
			1..=10 => println!("Está entre um e dez e é menor que 100"),
			11..=99 => println!("Não está entre 1 e 10 e é menor que 100"),
			_ => println!("Não está entre 1 e 10 e não é menor que 100")
		}		
	}
}

use condicionais::ifcomvariavel as ifcomvariavel;
use condicionais::checa_notas as checa_notas;
use condicionais::checa_notas_match as checa_notas_match;
use condicionais::checa_numero as checa_numero;
use condicionais::checa_tuplas as checa_tuplas;
use condicionais::vogal_ou_consoante as vogal_ou_consoante;
use condicionais::entre_dez_e_um_e_menor_que_cem as entre_dez_e_um_e_menor_que_cem;
use condicionais::entre_dez_e_um_e_menor_que_cem_match as entre_dez_e_um_e_menor_que_cem_match;

enum Gender{
		Feminino,
		Masculino,
		Outro
	}

fn main() {
    println!("=========================");
	println!("Condicionais em RUST:");
	println!("IFs:");
	ifcomvariavel();
	
	let nota_a:f32 = 0.0;
	let nota_b:f32 = 3.2;
	let nota_c:f32 = 5.1;
	let nota_d:f32 = 8.3;
	
	checa_notas(nota_a);
	checa_notas(nota_b);
	checa_notas(nota_c);
	checa_notas(nota_d);
	println!("=========================");
	println!("Matchs:");
	println!("\nNotas:");
	checa_notas_match(nota_a);
	checa_notas_match(nota_b);
	checa_notas_match(nota_c);
	checa_notas_match(nota_d);
	
	let num_a = 0;
	let num_b = 3;
	let num_c = 8;
	println!("\nNúmero Primo:");
	checa_numero(num_a);
	checa_numero(num_b);
	checa_numero(num_c);
	
	println!("\nGênero - ENUMs:");
	let genero = Gender::Feminino;
	
	match genero{
		Gender::Feminino => println!("Feminino"),
		Gender::Masculino => println!("Masculino"),
		Gender::Outro => println!("Outro"),
	};
	println!("\nZeros - Tuplas:");
	checa_tuplas((0,32));
	checa_tuplas((32,0));
	checa_tuplas((10,10));
	println!("\nConsoantes e Vogais - Vinculação:");
	let name: &'static str = "Felipe";
	let mut vogais:i32 = 0;
	let mut consoantes:i32 = 0;
	for a in name.chars(){
		match vogal_ou_consoante(a){
			r @ 'v' => {
				println!("{}", r);
				vogais+=1
			},
			r @ 'c' => {
				println!("{}", r);
				consoantes+=1
				},
			_ => ()
		}
	}
	println!("Numero de consoantes: {}\nNumero de vogais: {}\n", consoantes, vogais);
	
	println!("=========================");
	println!("IF-ELSE IF- ELSE");
	entre_dez_e_um_e_menor_que_cem(5);
	entre_dez_e_um_e_menor_que_cem(12);
	entre_dez_e_um_e_menor_que_cem(120);
	println!("MATCHs");
	entre_dez_e_um_e_menor_que_cem_match(5);
	entre_dez_e_um_e_menor_que_cem_match(12);
	entre_dez_e_um_e_menor_que_cem_match(120);
	
	
}

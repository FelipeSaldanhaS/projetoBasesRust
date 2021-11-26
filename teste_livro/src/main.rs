fn main() {
	println!("CARACTERE");
	
	let a:char = 'a';
	
	println!("Letra A-> {}", a);
	
   	println!("Maiúsculo -> {}", 'a'.is_uppercase());
	
	println!("Minúsculo -> {}", 'a'.is_lowercase());
	
	println!("Espaço em branco -> {}", 'a'.is_whitespace());
	
	println!("Alfanúmerico -> {}", 'a'.is_alphanumeric());
	
	println!("Númerico -> {}", 'a'.is_numeric());
	
	let nome = "Felipe";
	if nome.eq("Felipe"){println!("{}", nome)}
	
	//===========
	println!("================================");
	println!("BOOLEANO");
	let x:bool = false;
	let y:bool = true;
	
	if !x{
		println!("X é falso");
	}
	if y{
		println!("Y é verdadeiro");
	}
	//Em C, o if pode tirar de um tipo numa condição se é verdadeiro ou falso pelo número 0(falso) ou qualquer outro(verdadeiro).
	//Rust por outro lado não suporta isso veja a seguir:
	
	//let a:i32 = 1;
	/*if a{
		println!("A é igual a 1");
	}
	*/
	//Rust espera numa condicional um valor booleano, se retornar inteiro, não vai compilar o código por mismatched types(tipos imcompatíveis)
	
	//=======
	println!("Verdadeiro E	Falso	é	{}",	true	&&	false);
	println!("Verdadeiro	OU	Falso	é	{}",	true	||	false);
	println!("NÃO	Verdadeiro	é	{}",	!true);
	//=======
	/*
	let a: i32 = 4294967295;
	println!("{}", a);
	*/
	//Caso de sobrecarregar uma variavel do tipo inteiro com 32 bits, 1 para sinal, para usar sem o sinal use u32.
	//======== Vendo como o inteiro sem sinal e com sinal tem valores diferentes.
	println!("================================");
	println!("INTEIROS");
	println!("i8 = {} a {}", i8::min_value(),i8::max_value());
	println!("i16 = {} a {}", i16::min_value(),i16::max_value());
	println!("i32 = {} a {}", i32::min_value(),i32::max_value());
	
	println!("u8 = {} a {}", u8::min_value(),u8::max_value());
	println!("u16 = {} a {}", u16::min_value(),u16::max_value());
	println!("u32 = {} a {}", u32::min_value(),u32::max_value());
	//Funções interessantes de contagem do std
	let h:i8 = 3;
	println!("Uns: {}", h.count_ones());
	println!("Zeros: {}", h.count_zeros());
	//Usando funções de inverter bits
	let a:i8 = 1;
	println!(">>:	{}",	a.rotate_left(7));
	println!(">>:	{}",	a.rotate_right(8));
	println!(">>:	{}",	a.swap_bytes());
	//Usando variaveis decimais de ponto flutuante
	//f32 só mostrará 2 casa decimais, enquanto f64 demonstrará mais que 2.
	let a:f32 = 10232.61;
	println!("================================");
	println!("DECIMAIS");
	println!("{}",	a);
	
	println!("Chão: {}",	a.floor());
	println!("Teto: {}",	a.ceil());
	println!("Arredonda: {}",	a.round());
	println!("Truncado: {}",	a.trunc());
	println!("Fracional: {}",	a.fract());
	
	println!("É finito?: {}",	a.is_finite());
	println!("É infinito?: {}",	a.is_infinite());
	println!("É NaN(não númerico)?: {}",	a.is_nan());
	
	println!("ARRAYS E SLICES");
	let a =  ["Felipe","Rust","Saldanha"];
	println!("Tentando programar em {}, por {} {}.", a[1],a[0],a[2]);
	let a = [0;5];
	println!("Tentando programar em {}, por {} {}.", a[1],a[0],a[2]);
	println!("Número de elementos: {}",a.len());
	/*
	let mut a = [0;5];
	a[1] = "Rust"
	println!("Tentando programar em {}, por {} {}.", a[1],a[0],a[2]);
	Não vai funcionar porque mesmo que agora o vetor seja mutavel, 
	ele é do tipo inteiro pela atribuição de todos seus campos com 0(inteiro), então 
	é preciso declarar seu tipo antes de executar a troca de valores	
	*/
	let mut a =["";5];
	a[0] = "Felipe";
	a[1] = "Rust";
	a[2] = "Saldanha";
	
	println!("Tentando programar em {}, por {} {}.", a[1],a[0],a[2]);
	
	let a = [1,2,3,4,5,6,7,8,9,0];
	let b = &a[..];
	let c = &a[3..5];
	
	println!("{}",a[1]);
	
	println!("a	tem	{}	elementos",	a.len());
	println!("b	tem	{}	elementos",	b.len());
	println!("c	tem	{}	elementos",	c.len());
	
	for x in c.iter(){
		println!("Iterar: {}", x);
	}
	//Iter significa iterar que é repetir a ação, ou seja, o iter() vai entrar em um loop para atingir
	//todos os valores dentro de c.
	println!("================================");
	println!("TUPLAS");
	let a = ('a', "char");
	let b = (32, "integer");
	let c = ("Nome", "Felipe");
	
	let d = (1, "Teste", 'c');
	
	println!("{}",d.1);
	/*
	let d0 = d.0;
	let d1 = d.1;
	let d2 = d.2;
	Possivel atribuir dessa forma os valores da tupla pegando seu indice. 
	*/
	let (d0,d1,d2) = d;
	
	println!("Valores de D: {} {} {}", d0, d1, d2);
	
	let g = (1, "um");
	let h = (2, "dois");
	let j = (3, "tres");
	
	let k = [g,h,j];
	
	println!("Acessando valores da tupla dentro de array: {}",k[1].1);
	
	/*
	let g = (1, 45.44);
	let h = ("Numero","dois");
	let j = ("Letra", 'c');
	
	let k = [g,h,j];
	Gera erro de mismatched types por causa de tipos diferentes dentro de um array, o que não é compativel
	com a estrutura, temos g com dois inteiros, h com duas strings, e j com uma string e um char
	*/
	println!("================================");
	println!("ENUMS");
	enum Gender{
		Feminino,
		Masculino,
		Outro
	}
	
	struct Person{
		name: &'static str,
		gender: Gender
	}
	
	let felipe = Person {
		name:	"Felipe Saldanha",
		gender:	Gender::Masculino
	};
	
	let adelia = Person {
		name:	"Adelia Maria Fontes",
		gender:	Gender::Feminino
	};

}

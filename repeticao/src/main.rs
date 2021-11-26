mod repeticao{
	pub fn enquanto_simples()->(){
		let mut a:i32 = 0;
		while a< 10{
			a+=1;
			println!("Agora A é {}", a);
		}
	}
	pub fn loop_simples() -> (){
		let mut a:i32 = 0;
		loop{
			a+=1;
			println!("Repetindo em loop!");
			if a > 10{break;}
		}
	}
	pub fn for_simples()->(){
		println!("For com range exclusivo:\n");
		for a in 1..11{ println!("Agora A é {}!", a)}
		//No for foi usado um range exclusivo, que pega do primeiro valor
		//exibido e vai até 1 anterior, excluindo o ultimo numero, se for utilizar
		//do inclusivo será necessário usar 1..=11 para a mesma função.
		println!("\nFor com range inclusivo:\n");
		for a in 1..=11{ println!("Agora A é {}!", a)}
	}
	pub fn while_tabuada(a:i32)->(){
		let mut x = 1;
		while x < 11{
			println!("{} X {} = {}", a, x, a * x);
			x += 1;
		}
	}
	pub fn loop_exponencial(a:i32)->(){
		let mut x = 1;
		loop{
			
			println!("{} ^ {} = {}", a, x, i32::pow(a,x));
			
			x+=1;
			if x == 8{break}
		}
	}
	pub fn for_listagem(a:&[&str])->(){
		for x in a.iter(){
			println!("Nome: {}",x)
		}
	}
}

fn main() {
	println!("=========================");
	println!("Laços de repetição em RUST:");
	println!("While:\n");
    repeticao::enquanto_simples();
	println!("Loop:\n");
	repeticao::loop_simples();
	println!("For:\n");
	repeticao::for_simples();
	println!("=========================");
	println!("\nWhile - Tabuada:\n");
	repeticao::while_tabuada(5);
	println!("\nLoop - Potencia:\n");
	repeticao::loop_exponencial(2);
	println!("\nFor - Listagem:\n");
	let a = &["Felipe","Marcela","Diogo","Lisa"];
	repeticao::for_listagem(a);	
}

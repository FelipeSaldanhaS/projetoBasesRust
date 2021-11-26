extern crate time;

fn main() {
	let pointer_to_function: fn()  = print_today;
	let pointer_to_function2: fn()  = exibir_coracao_e_questionar_base;		
	do_the_things(pointer_to_function);
	do_the_things(pointer_to_function2);
	//--------------//
	let x:i32 = 1;
	let y:i32 = 2;
	println!("{} + {} = {}", x, y, sum(x,y));
	//---------------//
	println!("Testando funções do char:");
	println!("{} (a)", 'a'.is_alphabetic());
	println!("{} ( )", ' '.is_whitespace());
	println!("{} ( - )", '-'.is_alphabetic());
}

fn do_the_things(function: fn()){
	function()
}

fn	sum(a:	i32,	b:	i32)	->	i32{
	a+b
}

fn print_today(){
	const THE_1900: i32 = 1900;
	
    let d = time::now();
	let (day, month, year): (i32,i32,i32) = (d.tm_mday, d.tm_mon+ 1, d.tm_year+THE_1900);
	println!("Hoje é {}/{}/{}", day, month, year);
	
    //let _j: i32 = 2;
	//j = 3; causaria um erro já que j é uma variavel que não pode ser mutada.
	//println!("Const THE_1900: {}", THE_1900);
}

fn exibir_coracao_e_questionar_base(){
	let a: char = '\u{2192}';
	let b: char = '9';
	let c: char = '0';
	
	println!("UGA: {}", a);
	
	let repr:String = a.escape_unicode().collect();
	
	println!("{} é um digito? {}",a, a.is_digit(10));
	println!("{} é um binário? {}\n",a, a.is_digit(2));
	
	println!("{} é um digito? {}",b, b.is_digit(10));
	println!("{} é um binário? {}\n",b, b.is_digit(2));
	
	println!("{} é um digito? {}",c, c.is_digit(10));
	println!("{} é um binário? {}\n",c, c.is_digit(2));
	
	println!("{}\n", repr);
		
}



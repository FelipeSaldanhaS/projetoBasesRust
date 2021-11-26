#[derive(Debug,PartialEq)]
struct Admin { username: &'static str}
#[derive(Debug)]
struct Operador { username: &'static str}
#[derive(Debug)]
struct BasicUser { username: &'static str}

trait User{
	fn new(username: &'static str) -> Self;
	
	fn username(&self) -> &'static str;
	
	fn login(&self) -> &'static str;
	
	fn logout(&self) -> &'static str;
	
	fn is_logged_in(&self) -> bool{
		false
	}
}

impl User for Admin {
	fn new(username: &'static str) -> Admin{
		Admin {username: username}
	}
	
	fn username(&self) -> &'static str{
		self.username
	}
	
	fn login(&self) -> &'static str{
		"Usuário do tipo ADMIN entrou no sistema!"
	}
	
	fn logout(&self) -> &'static str{
		"Usuário do tipo ADMIN saiu do sistema!"
	}
		
}

impl User for Operador {
	fn new(username: &'static str) -> Operador{
		Operador {username: username}
	}
	
	fn username(&self) -> &'static str{
		self.username
	}
	
	fn login(&self) -> &'static str{
		"Usuário do tipo OPERADOR entrou no sistema!"
	}
	
	fn logout(&self) -> &'static str{
		"Usuário do tipo OPERADOR saiu do sistema!"
	}
		
}


impl User for BasicUser {
	fn new(username: &'static str) -> BasicUser{
		BasicUser {username: username}
	}
	
	fn username(&self) -> &'static str{
		self.username
	}
	
	fn login(&self) -> &'static str{
		"Usuário do tipo BÁSICO entrou no sistema!"
	}
	
	fn logout(&self) -> &'static str{
		"Usuário do tipo BÁSICO saiu do sistema!"
	}
		
}

enum BookFormat{
	Paperback,
	Hardback,
	Ebook
}

struct Book{
	isbn:i32,
	title: &'static str,
	format: BookFormat
}

impl PartialEq for Book{
	fn eq(&self, other: &Book) -> bool{
		self.isbn == other.isbn
	}
}

use	std::cmp::Ordering;
use	std::ops::Add;


#[derive(PartialOrd, PartialEq, Eq, Clone, Copy)]
struct Person{
	age:i32,
	name: &'static str
}

impl Ord for Person{
	fn cmp(&self, other: &Person) -> Ordering{
		(self.age).cmp(&(other.age))
	}
}

impl Add<i32> for Person{
	type Output = i32;
	
	fn add(self, b: i32)->i32{
		self.age + b
	}
}

fn older(p1: Person, p2: Person){
	if p1 > p2{
		println!("{} tem mais anos de vida do que {}",
		p1.name, p2.name);
	} else{
		println!("{} tem mais anos de vida do que {}",
		p2.name, p1.name)
	}
	
}

#[derive(PartialEq)]
enum Documento{
	RG,
	CPF,
	CNH
}

struct Pessoa{
	nome: &'static str,
	idade: i32,
	identificacao: Documento
}


fn main() {
	println!("Testando Traits e implementações");
    let admin: Admin = User::new("Corleone");
	let admin2: Admin = User::new("Felipe");
	let basic_user: BasicUser = User::new("Felipe");
	let operador: Operador = User::new("Carlos");
	println!("Comparação eq() admin e admin2");
	println!("{}", admin == admin2);
	
	println!("Testando funções das structs implementadas");
	println!("{:?}", admin);
	println!("Bem-vindo usuário {}", admin.username());
	println!("{}", admin.login());
	println!("{}\n", admin.logout());
	
	println!("{:?}", operador);
	println!("Bem-vindo usuário {}", operador.username());
	println!("{}", operador.login());
	println!("{}\n", operador.logout());
	
	println!("{:?}", basic_user);
	println!("Bem-vindo usuário {}", basic_user.username());
	println!("{}", basic_user.login());
	println!("{}\n", basic_user.logout());
	
	println!("{}",admin.is_logged_in());
	println!("{}",operador.is_logged_in());
	println!("{}",basic_user.is_logged_in());

	println!("\nTestando implementações em Traits do std");
	
	let b1 = Book{
		isbn:1234567890,
		title: "O Senhor dos Anéis",
		format: BookFormat::Paperback
	};
	
	let b2 = Book{
		isbn:1234567890,
		title: "O Senhor dos Anéis",
		format: BookFormat::Paperback
	};	
	
	let b3 = Book{
		isbn:1234567810,
		title: "O Hobbit",
		format: BookFormat::Hardback
	};
	
	println!("{}", b1 == b2);
	println!("{}", b2 == b3);
	println!("{}", b1 == b3);
	
	println!("\nTestando Traits e implementações com PartialOrd e Ord");
	
	let p1 = Person{
		age: 6,
		name: "Nicolas"
	};
	
	let p2 = Person{
		age: 31,
		name: "Ana"
	};
	
	let mut p3 = Person{
		age: 40,
		name: "Marcelo"
	};
	
	older(p1,p2);
	older(p2,p3);
	older(p1,p3);
	
	let x = p3 + 10;
	
	println!("Nome de p3: {}", p3.name);
	
	println!("Nova idade de {}: {}", p3.name, x);
		

	
	println!("\nIdentificação:");
	let mut pessoa_1 = Pessoa{
		nome: "Marcos",
		idade: 20,
		identificacao: Documento::CNH
	};
	pessoa_1.nome = "Carlos";
	println!("Nome: {}\nIdade: {}", pessoa_1.nome, pessoa_1.idade);
	
	if pessoa_1.identificacao == Documento::RG{
		println!("Documento: RG");
	}else if pessoa_1.identificacao == Documento::CPF{
		println!("Documento: CPF");
	}else{
		println!("Documento: CNH");
	}
}

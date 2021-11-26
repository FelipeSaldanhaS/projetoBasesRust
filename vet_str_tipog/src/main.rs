extern crate num;
use	std::cmp::Ordering;

#[derive(PartialOrd,	PartialEq,	Eq,	Clone,	Copy,	Debug)]
struct Contato{
	nome: &'static str,
	telefone: &'static str
}

impl Ord for Contato{
	fn cmp(&self, other: &Contato) -> Ordering{
		(self.nome).cmp(&(other.nome))
	}
}

fn	generic_sum<T:	num::Num>(x:	T,	y:	T)	->	T	{
	x.add(y)
	}

fn fale_meu_nome(nome: String){
	println!("Seu nome é {}", nome);
}
//É póssivel alterar o tipo do parametro nome para um String estatico, usando &'static str como a segur
fn fale_meu_nome_str(nome: &'static str){
	println!("Seu nome é {}", nome);
}
//O simbolo '&' significa que pedimos uma variavel emprestada deste escopo, nesse caso nome.
//Isso chama-se borrowing
//O simbolo " ' " diz que estamos criando um liferime, uma variavel emprestada que vai existir no escopo atual vindo de outro escopo externo
//O lifetime é util principalmente para economizar memoria, já que pode pegar o valor de uma variavel sem atribuir a outra.
fn main() {
	//--------------------------------------------------------
	//VETORES
	//--------------------------------------------------------
	
	println!("VETORES: Testando vetores simples:\n");
	let mut vetor = vec![3,2,1];
	vetor.sort();
    println!("Vetor ordenado {:?}", vetor);


	let vetor_int: Vec<i32> = (1..4).collect();
	
	println!("{:?}", vetor_int);
	
	let vetor_zeros = vec![0; 5];
	
	println!("{:?}", vetor_zeros);
	
	//--------------------------------------------------------
	println!("\nVETORES: Testando vetores de estruturas:\n");
	
	let contato_1 = Contato{
		nome: "Zena",
		telefone: "+55(11) 9.1234.5678"
	};
	
	let contato_2 = Contato{
		nome: "Ayla",
		telefone: "+55(11) 9.8765.4321"
	};
	
	let mut agenda = vec![contato_1, contato_2];
	println!("{:?}\n{:?}", agenda[0], agenda[1]);
	
	//Utilizando o push() para inserir um valor no vetor declarado, lembre-se de declarar o vetor como mutável
	println!("\nVETORES: Testando push:\n");
	let contato_3 = Contato{
		nome: "John",
		telefone: "+55(11) 9.1233.7899"
	};
	
	agenda.push(contato_3);
	println!("A agenda com 3 contatos: \n {:?}", agenda);
	
	println!("\nVETORES: Testando pop:\n");
	//Em rust valores devem ser retornados, e como não é possivel simplesmente descartar esses valores, será utilizado o  argumento "_" para "descartar" esse retorno.
	//Lembrando que também é possivel atribuir a uma variavel esse retorno.
	println!("A agenda com 3 contatos: \n {:?}", agenda);
	let _ = agenda.pop();
	println!("A agenda com 2 contatos: \n {:?}", agenda);
	
	//println!("Contato 51: {:?}", agenda[50]);
	//Esse print com o indice 50, onde só temos duas posições, não irá funcionar, é possivel remediar isto com !try.
	
	println!("\nVETORES: iterando:\n");
	//Usando o for
	/*
	for contato in agenda{
		println!("{:?}\n", contato);
	}
	*/
	//Usando iter() e next()
	let mut agenda_iter = agenda.iter();
	println!("{:?}", agenda_iter.next().unwrap());
	println!("{:?}", agenda_iter.next().unwrap());
	
	for iterar in agenda.iter(){
		println!("{:?}", iterar);
	}
	
	println!("\nVETORES: maps e collects:\n");
	
	let nomes = agenda.iter()
		.map(|contato| {contato.nome})
		.collect::<Vec<_>>();

	println!("Nomes: {:?}", nomes);
	
	let telefones = agenda.iter()
		.map(|contato| {contato.telefone})
		.collect::<Vec<_>>();
		
	println!("Telefones: {:?}", telefones);
	
	//Map tem muitas funções, infinitas na verdade, por ser uma fração de codigo rust, é possivel fazer qualquer coisa dentro desse espaço
	//Seja funções aritmeticas, lógicas e por ai vai
	println!("\nVETORES: ordenação:\n");
	
	let contato_4 = Contato{
		nome: "Carlos",
		telefone: "+55(11) 9.8365.4321"
	};
	
	agenda.push(contato_3);
	agenda.push(contato_4);
	
	agenda.sort();
	
	for item in agenda.iter(){println!("{:?}", item);}
	
	
	println!("\nVETORES: First e Last:\n");
	
	println!("Primeiro: {:?}", agenda.first());
	//Pode ser substituido por agenda[0]
	println!("Ultimo: {:?}", agenda.last());
	//Pode ser substituido por agenda[agenda.len() - 1]
	
	//--------------------------------------------------------
	// STRINGS
	//--------------------------------------------------------
	println!("\n===================================================================");
	println!("\nSTRINGS: Declarações:\n");
	
	
	let editora = "L&PM Pocket".to_string();
	let autor = String::from("Arthur Conan Doyle");
	let livro: String = "O Signo dos Quatro".into();
	
	println!("Este é o livro {} da {}, \nescrito por {}.\n", livro, editora, autor);
	
	let vetor_bytes = vec![82, 117, 115, 116];
	let a = String::from_utf8(vetor_bytes).unwrap();
	
	println!("{}", a);
	
	let mut frase = String::from("Este é o livro ");
	//Quando é concatenado string em string, geralmente rust vai esperar uma string estatica, logo usando o endereço dela (operador &).
	frase += &livro;
	frase += " da ";
	frase += &editora;
	frase += ",\nescrito por ";
	frase += &autor;
	//Essa coerção é chamada de Deref, de desreferenciar que significa em termos simples pegar os valores de um ponteiro.
	println!("\nEsta frase foi escrita por concatenação '+=' : {}", frase);
	
	let mut frase = String::from("Este é o livro ");
	frase.push_str(&livro);
	frase.push_str(" da ");
	frase.push_str(&editora);
	frase.push_str(",\nescrito por ");
	frase.push_str(&autor);
	
	println!("\nEsta frase foi escrita por concatenação 'push_str()' : {}", frase);
	
	let mut frase_char = String::from("Hey ");
	frase_char.push('Y');
	frase_char.push('a');
	frase_char.push('a');
	frase_char.push('h');
	frase_char.push('!');
	
	println!("\n{}", frase_char);
	
	let a = String::with_capacity(255);
	let b = String::from("ABC");
	
	println!("a: {} -> {}, logo len() -> {}", a.capacity(), a, a.len());
	println!("b: {} -> {}, logo len() -> {}", b.capacity(), b, b.len());
	//O metodo capacity() é para saber quanto caracteres PODEM ser inseridos nessa string.
	//O metodo len() é para saber o tamanho alocado da String. 
	
	let mut nome_incompleto = String::from("Felipe");
	//Para essas funções terem ação é necessário o argumento "mut".
	nome_incompleto.reserve(15);
	println!("\nTestando o Shrink:\n");
	println!("Nome incompleto capacidade sem o shrink: {}", nome_incompleto.capacity());
	println!("Nome incompleto tamanho sem o shrink: {}", nome_incompleto.len());
	
	nome_incompleto.shrink_to_fit();
	//Shrink vem de encolher, nesse caso, esse metodo vai pegar o espaço adicional alocado e reduzir ele para o tamanho utilizado real.
	println!("Nome incompleto capacidade com o shrink: {}", nome_incompleto.capacity());
	println!("Nome incompleto tamanho com o shrink: {}", nome_incompleto.len());
	
	
	let mut a = String::from("Rust");
	a.reserve(50);
	
	println!("\nTestando funções no String:\n");
	a.push_str(" rules");
	println!("a: {} -> {}", a.capacity(), a);
	
	println!("\nShrink:\n");
	a.shrink_to_fit();
	println!("a: {} -> {}", a.capacity(), a);
	
	println!("\nTruncate:\n");
	a.truncate(4);
	println!("a: {} -> {}", a.capacity(), a);
	
	println!("\nShrink:\n");
	a.shrink_to_fit();
	println!("a: {} -> {}", a.capacity(), a);
	
	println!("\nRemove:\n");
	a.remove(2);
	println!("a: {} -> {}", a.capacity(), a);
	
	println!("\nClear:\n");
	a.clear();
	println!("a: {} -> {}", a.capacity(), a);
	
	println!("\nSTRINGS: Testando remover com pop:\n");
	
	let mut a = String::from("Rust");
	
	for _x in 0..a.len(){
		let ret = a.pop();
		match ret {
			Some(char) => println!("Pop -> {}", char),
			None => println!("Sem mais caracteres..."),
		}
	}
	//Este é um metodo simples, o loop irá apagar todos os elementos da string "a".
	//Quando entrar no loop, é declarado a variavel ret, que o proposito é apagar uma letra de "a".
	//Depois é executado uma estrutura match encima de ret que possui dois casos, caso tenha algum valor(Some, ou seja apagou algo) mostrará a letra apagada.
	//Caso não tenha, terá um None e uma mensagem que não tem mais caracteres.
	println!("Palavra ficou: {}\nde capacidade: {}", a, a.capacity());
	
	println!("\nSTRINGS: Testando iterar com chars e bytes:\n");
	
	let a = String::from("Rust");
	
	for chr in a.chars(){
		println!("Char: {}\n", chr)
	}
	//Iterando char por char
	for bte in a.bytes(){
		println!("Byte: {}\n", bte);
	}
	//Iterando byte por byte
	for(index, charac) in a.chars().enumerate(){
		println!("{} -> {}", index, charac);
	}
	//Iterando por index, junto do valor do caractere, possivel pelo enumerate().
	
	println!("\nSTRINGS: Casting:\n");
	
	let a = String::from("8");
	
	let b = match a.parse::<i8>(){
		Ok(c) => c + 1,
		Err(_d) => 0,
	};
	println!("8 + 1 = {}", b);
	
	println!("\nSTRINGS: Comparando str e String:\n");
	let nome = "Jorge".to_string();
	//Se for declarado apenas com o texto em aspas, vai ser um literal str e não uma instancia de String, o que causará mismatched types
	let nome_estatico = "Carlos";
	fale_meu_nome(nome);
	fale_meu_nome_str(nome_estatico);
	//Existem casos que é interessante trabalhar com o estatico(cenários de pouca memória), mas é sempre preferivel trabalhar com a String porque pode-se utilizar da sua vasta biblioteca
	
	println!("\nSTRINGS: Buscando e dividindo:\n");
	let mut nome = String::from("Felipe Saldanha");
	let mut espaco = nome.find(" ").unwrap_or(0);
	//O find encontra o caractere que deseja encontrar na String.
	
	let primeiro_nome:String = nome.drain(..espaco).collect();
	println!("Espaço em: {}", espaco);
	espaco = nome.find(" ").unwrap_or(0);
	println!("Espaço em: {}", espaco);
	let ultimo_nome:String = nome.drain(espaco..).collect();
	
	println!("Primeiro nome: {}", primeiro_nome);
	println!("Ultimo nome: {}", ultimo_nome);
	println!("Nome original, vazio?: {}", nome.is_empty());

	println!("\n===================================================================");
	println!("\nTIPOS DE DADOS GENERICOS: Funções genericas:\n");
	
	println!("{}", generic_sum::<i32>(6, 2));
	println!("{}", generic_sum::<i16>(3, 6));
	println!("{}", generic_sum::<i8>(1, 2));
	
	println!("{}", generic_sum::<f32>(1.9, 2.21));
}

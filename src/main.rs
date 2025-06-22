use core::num;
use std::io;

fn main() {
    //CHAMA A FUNÇÃO MENU..
    menu_opcoes();
}

fn menu_opcoes() {
    //INICIA O LOOP ATÉ QUE O USUÁRIO DIGITE ZERO PARA SAIR...
    loop {
        //MENU COM OPÇÕES DA CALCULADORA...
        println!("Calculadora em Rust!");
        println!("1 - Soma");
        println!("2 - Subtração");
        println!("3 - Divisão");
        println!("0 - Sair");

        //LEITURA DO VALOR INFORMADO PELO USUÁRIO
        let mut op = String::new();
        io::stdin()
            .read_line(&mut op)
            .expect("Erro ao ler a mensagem!");
        
        //VERIFICAÇÃO DO DADO DIGITADO PELO USUÁRIO
        //CASO O DADO NÃO SEJA UM VALOR DO TIPO INTEIRO(u8) ELE PEDE PARA QUE DIGITE
        //UM NOVO VALOR..
        let opcao: u8 = match op.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite uma opção válida!");
                continue;
            }
        };

        //VERIFICA QUAL OPÇÃO O USUÁRIO ESCOLHEU E CHAVA A FUNÇÃO PARA COLETAR OS DADOS..
        match opcao {
            0 => {
                println!("Encerrando o programa. Até logo!");
                break;
            }

            //CASO O VALOR NÃO SEJA ZERO É CHAMADA A FUNÇÃO coleta_dados()...   
            1 => coleta_dados(1),
            2 => coleta_dados(2),
            3 => coleta_dados(3),
            _ => print!("Digite uma das opções acima!"),
        }
    }
}
//FUNÇÃO coleta_dados(), NELA FEITA TODA A COLETA DE INFORMAÇÕES PARA A CALCULADORA..
fn coleta_dados(opcao: u8) {
    use std::io::{self, Write};

    //LER O PRIMEIRO VALOR...
    print!("Informe o primeiro valor: ");
    io::stdout().flush().unwrap();
    let mut valor1 = String::new();
    io::stdin()
        .read_line(&mut valor1)
        .expect("Erro ao ler o valor digitado!");
    let primeiro_valor: f64 = valor1
        .trim()
        .parse()
        .expect("Por favor digite um valor válido!");
    //LER O SEGUNDO VALOR...
    print!("Informe o segundo valor: ");
    io::stdout().flush().unwrap();
    let mut valor2 = String::new();
    io::stdin()
        .read_line(&mut valor2)
        .expect("Erro ao ler o valor digitado!");
    let segundo_valor: f64 = valor2
        .trim()
        .parse()
        .expect("Por favor digite um valor válido!");

    match opcao {
        1 => println!(
            "Soma de {} por {} = {}",
            primeiro_valor,
            segundo_valor,
            soma(primeiro_valor, segundo_valor)
        ),
        2 => println!(
            "Subtração de {} por {} = {}",
            primeiro_valor,
            segundo_valor,
            subtracao(primeiro_valor, segundo_valor)
        ),
        3 => {
            if segundo_valor == 0.0 {
                // VERIFICA SE O SEGUNDO VALOR É ZERO POIS NÃO PODEMOS DIVIDIR POR ZERO...
                println!("Erro: divisão por zero!");
            } else {
                println!(
                    "Divisão de {} por {} = {}",
                    primeiro_valor,
                    segundo_valor,
                    divisao(primeiro_valor, segundo_valor)
                );
            }
        }
        _ => println!("Opção incorreta!"),
    }
}

//FUNÇÃO SOMA...
fn soma(primeiro_v: f64, segundo_v: f64) -> f64 {
    primeiro_v + segundo_v
}

//FUNÇÃO DE SUBTRAÇÃO
fn subtracao(primeiro_v: f64, segundo_v: f64) -> f64 {
    primeiro_v - segundo_v
}

//FUNÇÃO DE DIVISÃO..
fn divisao(primeiro_v: f64, segundo_v: f64) -> f64 {
    primeiro_v / segundo_v
}

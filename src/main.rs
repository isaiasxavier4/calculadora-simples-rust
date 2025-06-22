use std::io;

fn main() {
   
    //MENU COM OPÇÕES DA CALCULADORA...
    println!("Calculadora em Rus!");
    println!("1 - Soma");
    println!("2 - Subtração");
    println!("3 - Divisão");

    //COMO TODO DADO RECEBIDO EM RUST É UMA STRING, PEGAMOS E COLOCAMOS
    //NA VARIÁVEL OP DO TIPO STRING E VAZIA...
    //A VARIÁVEL É MUTAVEL, OU SEJA PODE SOFRER AUTERAÇÃO NO VALOR...
    let mut op = String::new(); 

    //LER O DADO DIGITADO PELO USUÁRIO E VERIFICA SE ELE DIGITOU UM VALOR VÁLIDO..
    io::stdin().read_line(&mut op).expect("Erro ao ler a mensagem!");
    
    //PASSA O DADO DE STRING PARA INTIERO...
    //SE O USUÁRIO NÃO DIGITAR UM VALOR CORRETO É PEDIDO PARA 
    //QUE COLOQUE UM VALOR VÁLIDO..
    let opcao: u8 = op.trim().parse().expect("Por favor, digite um valor válido!");
    
    //VERIFICA QUAL OPÇÃO O USUÁRIO ESCOLHEU E CHAVA A FUNÇÃO PARA COLETAR OS DADOS..
    match opcao {
        1 => soma(),
        2 => Subtracao(),
        _=> print!("Digite uma das opções acima!"),
    }
}

fn soma(){
    print!("Você escolheu a opção SOMA!");
}

fn Subtracao(){
    print!("Você escolheu a opção SUBTRAÇÃO!");
}
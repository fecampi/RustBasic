// Declaração de uma função chamada "soma"

// Importando o Módulo em Outro Arquivo
mod my_module;
mod modules;

fn main() {
    
    // Variáveis:

    // Declaração de uma variável chamada "numero" do tipo i32 (inteiro de 32 bits)
    let mut numero: i32 = 5; // A palavra-chave "mut" indica que a variável é mutável

    println!("O valor da variável 'numero' é: {}", numero);

    // Modificando o valor da variável
    numero = 10;

    println!("Agora, o valor da variável 'numero' é: {}", numero);

    // Constantes

    // Declaração de uma constante chamada "PI" do tipo f64 (ponto flutuante de 64 bits)
    const PI: f64 = 3.14159;

    println!("O valor da constante 'PI' é: {}", PI);

    // Estruturas de controle de fluxo

    // else if:
    // A estrutura if é usada para avaliar uma condição e executar um bloco de código se a condição for verdadeira.
    // A cláusula else pode ser usada para fornecer um bloco de código a ser executado quando a condição é falsa.
    // A cláusula else if para lidar com várias condições.

    numero = 42;

    if numero > 50 {
        println!("O número é maior que 50.");
    } else if numero == 50 {
        println!("O número é exatamente 50.");
    } else {
        println!("O número é menor que 50.");
    }

    //match:
    // Útil para lidar com várias condições de maneira concisa.

    let dia_da_semana: &str = "quarta";

    match dia_da_semana {
        "segunda" | "terça" | "quarta" | "quinta" | "sexta" => {
            println!("Dia útil!");
        }
        "sábado" | "domingo" => {
            println!("Final de semana!");
        }
        _ => {
            println!("Dia inválido!");
        }
    }

    // Loops (for, while)

    // Loop Infinito (loop):
    // Ele executa indefinidamente até que seja explicitamente interrompido usando break.
    let mut contador: i32 = 0;

    loop {
        println!("Contagem: {}", contador);

        contador += 1;

        if contador == 5 {
            break; // interrompe o loop quando contador atinge 5
        }
    }

    // Loop com Condição (while):
    // Ele verifica a condição antes de cada iteração.
    contador = 0;

    while contador < 5 {
        println!("Contagem: {}", contador);
        contador += 1;
    }

    // Loop For (for):
    // É usado para iterar sobre uma sequência, como um intervalo ou uma coleção.
    for numero in 0..5 {
        println!("Número: {}", numero);
    }

    // Loop For com Iterador (for com iter()):
    // É usado para iterar sobre elementos em uma coleção usando iter().
    let lista: Vec<i32> = vec![1, 2, 3, 4, 5];

    for elemento in lista.iter() {
        println!("Elemento: {}", elemento);
    }

    // Funções:
    let resultado: i32 = my_module::soma(3, 5);
    println!("A soma é: {}", resultado);

    // Closures:
    let soma_closure = |a, b| a + b;
    println!("A soma é: {}", soma_closure(3, 5));

    modules::submodule::execute();  // Chamando a função do submódulo
}

pub fn default_function() {
    // Estruturas de controle de fluxo

    // else if:
    // A estrutura if é usada para avaliar uma condição e executar um bloco de código se a condição for verdadeira.
    // A cláusula else pode ser usada para fornecer um bloco de código a ser executado quando a condição é falsa.
    // A cláusula else if para lidar com várias condições.
    let numero: i32 = 5; // A palavra-chave "mut" indica que a variável é mutável


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
}
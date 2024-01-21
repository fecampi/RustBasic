// Declaração de uma função chamada "soma"

// Importando o Módulo em Outro Arquivo
// main.rs
// Use a palavra-chave `mod` seguida pelo nome do arquivo mod.rs (sem extensão)
mod modules;





fn main() {
    modules::data();
    modules::decision();
    modules::loops();
    let resultado = modules::functions::soma(3, 4);
    println!("Resultado da soma: {}", resultado);
    
    // Closures:
    let soma_closure = |a, b| a + b;
    println!("A soma é: {}", soma_closure(3, 5));
}

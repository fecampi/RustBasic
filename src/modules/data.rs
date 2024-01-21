pub fn default_function() {
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
}
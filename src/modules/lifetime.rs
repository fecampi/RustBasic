/// # Tempo de Vida em Rust
///
/// Tempo de vida refere-se à duração para a qual referências em Rust são válidas.
/// É denotado por apóstrofos (por exemplo, `'a`). Os tempos de vida ajudam a garantir
/// que referências emprestadas sejam sempre válidas e previnem referências penduradas.
///
/// Em funções, os tempos de vida são usados para especificar a relação entre os tempos
/// de vida de referências de entrada e saída, garantindo segurança na manipulação de referências.
///
/// ## Embora seja comum usar 'a, pode escolher outras letras
/// - 'a: usado frequentemente como o primeiro tempo de vida.
/// - 'b: usado como o segundo tempo de vida.
/// -'static: usado para indicar que algo tem tempo de vida estática, ou seja, existe durante toda a execução do programa.

// Função que recebe duas referências com um tempo de vida 'a
fn get_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Função que utiliza anotação de tempo de vida em parâmetro
fn first_word<'a>(s: &'a str) -> &'a str {
    // Implementação
    &s[0..5]  // Exemplo arbitrário, substitua pela lógica real
}


pub fn default_function() {
    let string1 = String::from("Rust");
    let _result;

    {
        let string2 = String::from("Programming");

        // Chamando a função com referências para string1 e string2
        _result = get_longest(&string1, &string2);

        // string2 sai de escopo aqui, e a memória associada a ela é liberada.
        // Portanto, a referência devolvida por get_longest não deve ser usada após esse ponto.
    }

    // O código abaixo resultaria em um erro, pois a referência devolvida por get_longest não é mais válida aqui.
    // println!("A string mais longa é: {}", _result); // Comente essa linha para corrigir o erro.
}

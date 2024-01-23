// - Refere-se à gestão da alocação e liberação de memória.

// Três princípios básicos:
// Esses princípios garantem um gerenciamento seguro e eficiente da memória em Rust.

fn ownership_system_rules() {
    // Regra 1: Cada valor tem uma única variável que o possui.
    let original_value = String::from("Hello");

    // Regra 2: Somente uma variável pode ser a proprietária do valor por vez.
    let _new_variable = original_value;

    // O seguinte código resultaria em um erro, pois original_value perdeu a propriedade.
    // println!("Original: {}", original_value); /

    // Regra 3: Quando a variável proprietária sai de escopo, o valor é liberado da memória.
    // new_variable sai de escopo aqui, e a memória associada ao valor é liberada automaticamente.
}


// Outros exemplos utilizando os princípios


// Transferindo a propriedade
fn transfer_ownership_example() {
    let s1 = String::from("hello");

    // propriedade de s1 -> s2.
    let s2 = s1;

    // erro, pois s1 não é mais válida.
    // println!("s1: {}", s1);

    // Imprimindo "hello" em s2.
    println!("s2: {}", s2);
} 

//  Uma única propriedade por vez
fn only_one_ownership_example() {
    let s3 = String::from("world");

    // propriedade de s3 -> s4.
    let s4 = s3;

    // erro, pois s3 não é mais válida.
    // println!("s3: {}", s3); //

    // Imprimindo "world" em s4.
    println!("s4: {}", s4);
} 

// Transferindo a propriedade para uma função
fn transfer_ownership_to_function(s: String) {
    // A propriedade de s -> s de função.
    println!("Função recebeu: {}", s);
} 
// Passando a propriedade como argumento
fn ownership_passing_example() {
    let s5 = String::from("Rust");

    // Propriedade de s5 transferida para a função.
    transfer_ownership_to_function(s5);

    // Erro, pois s5 não é mais válida.
    // println!("s5: {}", s5); 
}

// Retornando a propriedade de uma função
fn ownership_returning_example() -> String {
    let s6 = String::from("Ownership");
    s6
} 


pub fn default_function() {
    ownership_system_rules();
    transfer_ownership_example();
    only_one_ownership_example();
    ownership_passing_example();
    let returned_string = ownership_returning_example();
    println!("Função retornou: {}", returned_string);
}

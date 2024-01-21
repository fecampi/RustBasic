pub fn default_function() {
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

}
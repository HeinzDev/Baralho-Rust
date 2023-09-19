use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    let naipes = vec!["Copas", "Ouros", "Paus", "Espadas"];
    let numeros = vec![
        "Ás", "Dois", "Três", "Quatro", "Cinco", "Seis", "Sete", "Oito", "Nove", "Dez", "Valete",
        "Dama", "Rei",
    ];

    let mut baralho: Vec<(String, String)> = vec![];

    for naipe in &naipes {
        for numero in &numeros {
            baralho.push((numero.to_string(), naipe.to_string()));
        }
    }

    baralho.shuffle(&mut rng);

    let mao: Vec<(String, String)> = baralho.into_iter().take(5).collect();

    println!("Sua mão:");
    for (numero, naipe) in &mao {
        println!("{} de {}", numero, naipe);
    }
}


use std::io;
use rand::Rng;

fn main() {
    // initialisation des variables
    let mut essai = 10;
    let liste_mots = ["noel", "cadeau", "simple", "parfait", "remplir", "clement", "lumineux", "benefice", "prudent", "violent", "comprendre", "preciser", "delicieux", "aventure", "depenser", "souvenir", "sapin", "bonheur", "magique", "enchante"];
    let index = rand::thread_rng().gen_range(0..liste_mots.len());
    let secret_word = liste_mots[index];

    // création des tableaux
    let mut a_secret = Vec::new();
    let mut a_input = Vec::new();
    let mut a_letter = Vec::new();

    for c in secret_word.chars(){
        a_secret.push(c.to_string());
        a_input.push("_");
    }

    println!("________________________________");
    println!("\n        LE MOT MYSTERE");
    println!("________________________________");
    println!("A toi de jouer, bonne chance !\n");
    println!("Le mot mystère est en {} lettres", secret_word.len());
    println!("{}", a_input.concat());

    loop{
        //Saisie utilisateur
        let mut letter_input = String::new();
        io::stdin().read_line(&mut letter_input).expect("Erreur lors de la saisie");
        let letter_input = letter_input.chars().next().unwrap().to_string();
    
        // Si lettre est exacte l'ajoute au tableau (mot)
        for (i, c) in a_secret.iter().enumerate(){
            if letter_input.trim() == c.to_string() {
                a_input[i] = c;
            }
        } 

        //Vérifie si le mot est trouvé
        if a_secret != a_input {
            if a_secret.contains(&letter_input){
                println!("Bien joué!");
            } else{
                a_letter.push(letter_input); //recensement des lettres ratées
                println!("Raté, il reste {} essais", dessin_pendu(&mut essai));
                if essai == 0 {
                    println!("\n________________________________");
                    println!("\nPerdu! Le mot était \"{}\"", secret_word);
                    println!("________________________________\n");
                    break;
                }
            }
            // Affiche le mot en cours et les lettres ratées déjà saisies
            for c in a_input.iter(){
                print!("{} ", c);
            }
            println!("\n[{}]\nQuelle lettre? ", a_letter.concat());
        } else {
            println!("\n________________________________");
            println!("\nGagné! Le mot était \"{}\"", secret_word);
            println!("________________________________\n");
            break;
        }     
    }
    quitter();   
}

fn quitter(){
    let mut input = String::new();
    println!("Veux-tu relancer le jeu? [O/N]");
    io::stdin().read_line(&mut input).expect("Erreur de lecture de la ligne");
    if input.trim() == "O" || input.trim() == "o"{
        main()
    } else{
        std::process::exit(0);
    }
}

fn dessin_pendu(essai: &mut i32) -> i32{
    *essai = *essai - 1;

    if *essai == 0 {println!(r"   _______________")}
    if *essai <= 1 {println!(r"   |/          |  ")}
    if *essai == 2 {println!(r"   |/             ")}
    if *essai <= 3 {println!(r"   |           O  ")}
    if *essai <= 4 {println!(r"   |          /|\ ")}
    if *essai == 5 {println!(r"   |           |\ ")}
    if *essai <= 6 {println!(r"   |           |  ")}
    if *essai <= 7 {println!(r"   |          / \ ")}
    if *essai == 8 {println!(r"   |            \ ")}
    if *essai <= 9 {println!(r" __|__            ")}

    return *essai;
}
// Exercice 1 : Maximum de deux nombres
fn maximun(a: i32, b:i32) ->i32{
    if a>b  {
        a
    }
    else{
        b 
    }
}
fn main() {
    let result = maximun(10, 20);
    println!("Le maximun est {}", result); 
}


// Exercice 2 : Pair ou impair
fn pair_ou_impair(n: i32) -> &'static str {
    if n %2 == 0{
        &"pair"
    }
    else{
        &"impair"
    }
}

fn main() {
    let result = pair_ou_impair(2);
    println!("Le nombre est {}", result);
}

// Exercice 3 : Calcul de la surface d'un rectangle
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn surface(&self) -> u32 {
        self.largeur *self.hauteur
    }
}

fn main() {
    let rect = Rectangle { largeur: 30, hauteur: 50 };
    println!("Surface du rectangle : {} pixels carrés.", rect.surface());
}


// Exercice 4 : Enum des couleurs de feux de circulation
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn afficher_message(lumiere: TrafficLight) {
    match lumiere {
        TrafficLight::Red => println!("Arrêtez"),
        TrafficLight::Green => println!("Allez"), 
        TrafficLight::Yellow => println!("Préparez-vous"),
        _ => println!(""),
    }
}

fn main() {
    let lumiere = TrafficLight::Yellow;
    afficher_message(lumiere);
}


// Exercice 5 : Gestion des erreurs - Division sécurisée
fn division(a: i32, b: i32) -> Result<i32, String> {
    // Complète ici
    if b == 0 {
        Err(String::from("Division par zéro"))
    }
    else {
        Ok(a/b)
    }
}

fn main() {
    match division(10, 0) {
        Ok(result) => println!("Résultat de la division : {}", result),
        Err(e) => println!("Erreur : {}", e),
    }
}

// Exercice 6 : Propriété et emprunts
fn afficher_chaine(s: &String) {
    println!("Chaîne : {}", s);
}

fn main() {
    let s1 = String::from("Bonjour");
    afficher_chaine(&s1);
    println!("Encore dans main : {}", s1); // doit encore être utilisable
}


// Exercice 7 : Somme des éléments d'une liste
fn somme_elements(liste: Vec<i32>) -> i32 {
    let mut sum = 0;
    let length  = liste.len(); 
    for i in 0..length {
        sum += liste[i]
    }
    sum
}
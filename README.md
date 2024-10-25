Rust Tutorila

# 0. Création d'un projet

=> Creation d'un projet
cargo new mon_projet
cd mon_projet

```
mon_projet/
├── src/
│   └── main.rs
├── Cargo.toml
 ```

=> Compiler le projet 
cargo build


=> Executer le projet
cargo run

1. Variables et types de données

En Rust, les variables sont immuables par défaut, c'est-à-dire qu'elles ne peuvent pas être modifiées après leur déclaration, sauf si tu utilises mut.
Types de données courants :
i32 : Entier 32 bits (par défaut)
f64 : Nombre flottant 64 bits
bool : Booléen (true ou false)
char : Caractère unicode ('a', 'é', etc.)

```
fn main() {
    let x = 5; // variable immuable
    let mut y = 10; // variable mutable
    y += 5;
    println!("x = {}, y = {}", x, y);
}
 ```

2. Fonctions et contrôle de flux
Les fonctions en Rust sont définies avec le mot-clé fn. Voici comment écrire une fonction avec des paramètres et un type de retour :

```
fn main() {
    let result = addition(5, 3);
    println!("Résultat : {}", result);
}
```

```
fn addition(a: i32, b: i32) -> i32 {
    a + b // Retour implicite
}
```

Contrôle de flux :
Rust supporte les conditions avec if, les boucles avec loop, while, et for.

```
fn main() {
    let x = 10;
    if x > 5 {
        println!("x est plus grand que 5");
    } else {
        println!("x est inférieur ou égal à 5");
    }
    
    for i in 0..5 {
        println!("i = {}", i);
    }
}
```

3. Enums et structures (Stucts)

Structs :
Les structures en Rust permettent de regrouper des données sous une forme plus complexe.

```
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Nom : {}, Âge : {}", person.name, person.age);
}
```

Enums :
Les enums permettent de définir un type pouvant avoir plusieurs valeurs distinctes.
```
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;
    match dir {
        Direction::North => println!("Allons vers le nord !"),
        _ => println!("Autre direction."),
    }
}
```

4. Gestion des erreurs
Rust utilise le type Result<T, E> pour gérer les erreurs, ce qui permet une gestion explicite des erreurs au lieu d'utiliser des exceptions.

```
fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division par zéro"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match division(10, 0) {
        Ok(result) => println!("Résultat : {}", result),
        Err(e) => println!("Erreur : {}", e),
    }
}
```

5. Propriétés et emprunts
L'une des particularités de Rust est son système de propriété et d'emprunt qui permet une gestion fine de la mémoire, tout en évitant des erreurs classiques de gestion de pointeurs.

Propriété :
Quand une variable est assignée à une autre, la propriété des données est transférée, et la première variable ne peut plus être utilisée.

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 est déplacé
    // println!("{}", s1); // Erreur : s1 n'est plus valide
    println!("{}", s2);
}
```

Emprunt :

Les emprunts permettent de prêter une référence à une donnée sans en prendre la propriété.

```
fn main() {
    let s1 = String::from("hello");
    print_string(&s1);
    println!("{}", s1); // s1 est toujours valide

}

fn print_string(s: &String) {
    println!("{}", s);
}
```

6. Concurrence 

Rust facilite la gestion de la concurrence grâce à son système de threads sûrs.
La Concurrence en Rust est un aspect puissant et flexible du langage, conçu pour permettre l'exécution parallèle ou simultanée de tâches de manière sécurisée. 

```
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread: {}", i);
        }
    });

    handle.join().unwrap(); // Attendre que le thread se termine
}
```

7. Appel de fonction : 

function.rs
```
// pub signifie que c'est public
pub fn ma_fonction() {
    println!("Salut depuis le module function!");
}
```

main.rs
```
mod function;

fn main() {
    function :: ma_fonction();
}
```


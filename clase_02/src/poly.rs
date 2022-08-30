//! Ejemplos de polimorfismo
#[derive(Debug)]
struct Perro {
    nombre: String,
}

#[derive(Debug)]
struct Vaca {
    nombre: String,
}

// Exijo que el trait PuedeHablar se aplique sobre algo con el trait std::fmt::Debug
trait PuedeHablar: std::fmt::Debug {
    fn hablar(&self);
}

impl PuedeHablar for Perro {
    fn hablar(&self) {
        println!("{} dice 'guau'", self.nombre);
    }
}

impl PuedeHablar for Vaca {
    fn hablar(&self) {
        println!("{} dice 'moo'", self.nombre);
    }
}

fn main() {
    println!("Polimorfismo estático");
    let lola = Vaca {
        nombre: "Lola".to_owned(),
    };
    let negro = Perro {
        nombre: "Negro".to_owned(),
    };
    presentar(&lola);
    presentar(&negro);

    println!("Polimorfismo dinámico");
    let animales: Vec<Box<dyn PuedeHablar>> = vec![
        Box::new(Vaca {
            nombre: "Violeta".to_owned(),
        }),
        Box::new(Perro {
            nombre: "Pancho".to_owned(),
        }),
    ];
    for animal in &animales {
        animal.hablar();
    }
}

// `Presentar()` es polimórfico en *tiempo de compilación*
// Al momento de compilar ya se sabe qué tipo de variable va a recibir la función

fn presentar(animal: &(dyn PuedeHablar)) {
    println!("Se presenta {:?}", animal);
    animal.hablar();
}

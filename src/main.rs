
use rand::Rng;
use arraystring::Error;
use std::collections::HashMap;

fn main() -> Result<(), Error> {

	let mut persons = HashMap::new();

	persons.insert(0,"Zehiael Ramos".to_string());
	persons.insert(1,"Brenda Casillas".to_string());
	persons.insert(2,"Daniel Rocha".to_string());
	persons.insert(3,"Grace Almanza".to_string());
	persons.insert(4,"Monica Rivera".to_string());
	persons.insert(5,"Dante Robles".to_string());
	persons.insert(6,"Oscar Amilkar".to_string());
	persons.insert(7,"Javier Guerra".to_string());
	persons.insert(8,"Andres Gomez".to_string());
	persons.insert(9,"Jesus Yepes".to_string());
	persons.insert(10,"Javier Guerra".to_string());
	persons.insert(11,"Manuel Uribe".to_string());
	persons.insert(12,"Ivan Venegas".to_string());
	persons.insert(13,"Irving Eiimt".to_string());
	persons.insert(14,"Jenry Lievano".to_string());
	persons.insert(15,"Norberto Ruiz".to_string());
	persons.insert(16,"Ruben Bautista".to_string());
	persons.insert(17,"Maurizio Patino".to_string());

    let mut rng = rand::thread_rng();

	let mut shift: [&str; 3] = [""; 3];

	shift[0] = "Primer intento:";
	shift[1] = "Segundo intento:";
	shift[2] = "La persona condenada va ser...";

	assert_eq!(&shift[1..], &shift[1..]);

	for _msg in shift {

		let id: u8 = rng.gen_range(0..17);

    	println!("{:} {:?}", _msg, persons.get(&id));
	}

    Ok(())
}

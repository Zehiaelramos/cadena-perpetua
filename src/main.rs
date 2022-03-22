
use rand::Rng;
use arraystring::Error;
use std::collections::HashMap;

fn main() -> Result<(), Error> {

	let mut players = HashMap::new();

	players.insert(0,"Zehiael Ramos".to_string());
	players.insert(1,"Brenda Casillas".to_string());
	players.insert(2,"Daniel Rocha".to_string());
	players.insert(3,"Grace Almanza".to_string());
	players.insert(4,"Monica Rivera".to_string());
	players.insert(5,"Dante Robles".to_string());
	players.insert(6,"Oscar Amilkar".to_string());
	players.insert(7,"Javier Guerra".to_string());
	players.insert(8,"Andres Gomez".to_string());
	players.insert(9,"Jesus Yepes".to_string());
	players.insert(10,"Javier Guerra".to_string());
	players.insert(11,"Manuel Uribe".to_string());
	players.insert(12,"Ivan Venegas".to_string());
	players.insert(13,"Irving Eiimt".to_string());
	players.insert(14,"Jenry Lievano".to_string());
	players.insert(15,"Norberto Ruiz".to_string());
	players.insert(16,"Ruben Bautista".to_string());
	players.insert(17,"Maurizio Patino".to_string());

    let mut rng = rand::thread_rng();

	let mut shift: [&str; 3] = [""; 3];

	shift[0] = "Primer intento:";
	shift[1] = "Segundo intento:";
	shift[2] = "La persona condenada va ser...";

	assert_eq!(&shift[1..], &shift[1..]);

	for _msg in shift {

		let id: u8 = rng.gen_range(0..17);

    	println!("{} {:?}", _msg, players.get(&id));
	}

    Ok(())
}

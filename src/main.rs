use std::fs::File;
use std::io::Read;
use std::str;
use std::env; //Utilizzato per ottenere gli argomenti da terminale.


struct Registers{ //Tutti i registri. da V0 a V10, da VA a VF
    v_n:[u8;10],
    v_l:[u8;6]
}



fn load_rom(filename: &str) -> Vec<u8> { //Argomenti: nome file, passato per indirizzo a causa della balla sul lifespan. Ritorna il buffer contenente l'intera rom come vettore di u8.
    println!("Carico ROM {}",filename);
    let mut file=File::open(filename).unwrap();//Apre file.
    let mut buffer = Vec::new();//Un array è di dimensione fissata, un vettore è allocato dinamicamente.
    file.read_to_end(&mut buffer).unwrap(); //Carica l'intera ROM nella memoria in un botto. Idea orribile?
    return buffer;
}

fn exec_op(op: u8){
    //Switch case di ogni istruzione. Le istruzioni vengono lette byte per byte in decimale. Guarda su Wikipedia per la lista degli opcode in HEX.
    match op{
        100...116 => print!("MOV VX,"), 
        0...99=> println!("{}",op),
        _ => println!("{}, chisto proprio non u sacc",op), // Istruzione non ancora implementata.
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Prende gli argomenti che vengono passati al programma e li mette in un vettore di stringhe.
    let rom=load_rom(&args[1]); //Prende in considerazione solo il primo argomento come il nome della ROM.
    let reg = Registers {v_n:[0u8;10],v_l:[0u8;6]}; //Inizializza i vari registri sopra dichiarati. Se fa così su rust a quanto pare.
    for c_op in 0..rom.len(){ //Loop principale che esegue ogni istruzione contenuta nel buffer.
        exec_op(rom[c_op]);
    }
}

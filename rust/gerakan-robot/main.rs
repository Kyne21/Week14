use std::io;

fn main() {
    let mut position = (0, 0);
    println!("Posisi awal robot: ({}, {})", position.0, position.1);
    
    loop {
        println!("Masukkan perintah (w: atas, s: bawah, a: kiri, d: kanan, q: keluar):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        let command = input.trim();
        
        match command {
            "w" => position.1 += 1,
            "s" => if position.1 > 0 { position.1 -= 1; },
            "a" => if position.0 > 0 { position.0 -= 1; },
            "d" => position.0 += 1,
            "q" => {
                println!("Keluar. Posisi akhir robot: ({}, {})", position.0, position.1);
                break;
            }
            _ => println!("Perintah tidak valid. Gunakan w, s, a, d, atau q.")
        }
        
        println!("Posisi robot saat ini: ({}, {})", position.0, position.1);
    }
}

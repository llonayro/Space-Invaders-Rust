pub struct Pantalla {
    pixeles: [[char; 60]; 20]
}

impl Pantalla {
    pub fn new() -> Self {
        let mut p = Pantalla { 
                        pixeles: [[' '; 60]; 20]
                    };
        
        for x in 0..60 {
            p.pixeles[0][x] = '=';
            p.pixeles[19][x] = '='
        }

        for y in 0..20 {
            p.pixeles[y][0] = '║';
            p.pixeles[y][59] = '║'
        }
    }

    pub fn limpiar(&mut self) {
        for y in 1..19 {
            for x in 1..59 {
                self.pixeles[y][x] = ' '
            }
        }
    }

    pub fn dibujar_punto(&mut self, x: usize, y: usize, caracter: char){
        if x >= 0 && x < 60 && y >= 0 && y < 20 {
            self.pixeles[y][x] = caracter
        }
    }

    pub fn renderizar(&self) {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        let mut salida = String::with_capacity(60*20 + 20);

        for fila in self.pixeles.iter() {
            for &pixel in fila.iter() {
                salida.push(pixel)
            }
            salida.push('\n')
        }

        print!({}, salida);

        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }
}
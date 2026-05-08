use std::thread;
use std::time::Duration;

mod motor;
mod entidades;
mod interfaz;

use motor::juego::Juego;

fn main() {
    let mut j = Juego::new();
    j.preparar_pantalla();
    j.mostrar();

    let velocidad = 10;
    loop {
        j.actualizar();

        if j.game_over {
            j.terminar();
            break;
        }

        j.preparar_pantalla();
        j.mostrar();

        thread::sleep(Duration::from_millis(velocidad));
    }
}
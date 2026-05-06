mod motor;
mod entidades;
mod interfaz;

use motor::juego::Juego;

fn main() {
    let mut j = Juego::new();
    j.preparar_pantalla();
    j.mostrar();

    j.actualizar();
}

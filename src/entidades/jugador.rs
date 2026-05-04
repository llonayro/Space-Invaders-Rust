struct Jugador {
    vida: u32,
    posicion: Posicion,
    disparos: Vec<Disparo>,
}

impl Jugador {
    fn new() -> Jugador {
        Jugador {
            vida: 3,
            posicion: Posicion::new(30, 18),
            disparos: Vec::new()
        }
    }
}
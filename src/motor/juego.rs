

use crate::entidades::{Posicion, Direccion};

use crate::entidades::jugador::Jugador;
use crate::entidades::enemigo::{Enemigo, TipoEnemigo};
use crate::interfaz::pantalla::Pantalla;



pub struct Juego {
    jugador: Jugador,
    enemigos: Vec<Enemigo>,
    puntuacion: u32,
    game_over: bool,
    pantalla: Pantalla
}

impl Juego {
    pub fn new() -> Self {
        let mut lista_enemigos = Vec::new();
        for y in 1..4 {
            for x in 28..33 {
                let tipo = match y {
                              1 => {
                                TipoEnemigo::Fuerte
                              }

                              2 => {
                                TipoEnemigo::Rapido
                              }

                              3 => {
                                TipoEnemigo::Normal
                              }

                              _ => {
                                TipoEnemigo::Normal
                              }
                           };
                lista_enemigos.push(Enemigo::new(tipo, Posicion::new(x, y)))
            }
        }
        Self{
            jugador: Jugador::new(),
            enemigos: lista_enemigos,
            puntuacion: 0,
            game_over: false,
            pantalla: Pantalla::new(),
        }

    }

    pub fn preparar_pantalla(&mut self){
        for alien in self.enemigos.iter() {
            let c = match alien.tipo {
                        TipoEnemigo::Fuerte => {
                            'Ѫ'
                        }

                        TipoEnemigo::Normal => {
                            '¤'
                        }

                        TipoEnemigo::Rapido => {
                            'Ѧ'
                        }

                    };
            
            self.pantalla.dibujar_punto(alien.posicion.x, alien.posicion.y, c)
        }

        self.pantalla.dibujar_punto(self.jugador.posicion.x, self.jugador.posicion.y, '▲')
    }

    pub fn mostrar(&self) {
        self.pantalla.renderizar()
    }

    pub fn actualizar(&self){
        let mut x_max = 28;
        let mut x_min = 32;
        for alien in self.enemigos.iter() {
            if alien.posicion.x > x_max {
                x_max = alien.posicion.x
            }
            if alien.posicion.x < x_min {
                x_min = alien.posicion.x
            }
        }
        if x_max == 58 && self.enemigos[0].direccion == Direccion::Derecha {

        }
    }
}
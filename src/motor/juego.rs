

use crate::entidades::{Posicion, Direccion};

use crate::entidades::jugador::Jugador;
use crate::entidades::enemigo::{Enemigo, TipoEnemigo};
use crate::interfaz::pantalla::Pantalla;



pub struct Juego {
    jugador: Jugador,
    enemigos: Vec<Enemigo>,
    puntuacion: u32,
    pub game_over: bool,
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
        self.pantalla.limpiar_medio();

        for alien in self.enemigos.iter() {
            if alien.activo {
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
        }

        for disparo in self.jugador.disparos.iter() {
            self.pantalla.dibujar_punto(disparo.posicion.x, disparo.posicion.y, '⁞');
        }

        self.pantalla.dibujar_punto(self.jugador.posicion.x, self.jugador.posicion.y, '▲')
    }

    pub fn mostrar(&self) {
        let mut vidas = String::new();
        for _ in 0..self.jugador.vida {
            vidas.push_str(" ❤️");
        }
        self.pantalla.renderizar(vidas, self.puntuacion)
    }

    pub fn actualizar(&mut self){
        let x_max = self.enemigos.iter().map(|a| a.posicion.x).max().unwrap_or(0);
        let x_min = self.enemigos.iter().map(|a| a.posicion.x).min().unwrap_or(0);

        let en_borde_der = x_max == self.pantalla.pixeles[0].len() - 2;
        let en_borde_izq = x_min == 1;
        
        let direccion_actual = self.enemigos[0].direccion;

        let nueva_direccion = match(en_borde_der, en_borde_izq, direccion_actual){
                                        (true, _, Direccion::Derecha) => {
                                                                            Direccion::Abajo       
                                                                        }
                                        (true, _, Direccion::Abajo) => {
                                                                            Direccion::Izquierda
                                                                        }                                     
                                        
                                        (_, true, Direccion::Izquierda) => {
                                                                                Direccion::Abajo
                                                                            }
                                        (_, true, Direccion::Abajo) => {
                                                                            Direccion::Derecha        
                                                                        }

                                        _ => {
                                                direccion_actual
                                            }                                   
                                    };
        
        for alien in self.enemigos.iter_mut() {
            alien.direccion = nueva_direccion;
            alien.mover();
        }

        if self.enemigos.last().is_some_and(|e| e.posicion.y == 18) {
            self.game_over = true
        }

    }

    pub fn terminar(&mut self) {
        self.pantalla.limpiar_medio();
        let texto = ['G', 'A', 'M', 'E', ' ', 'O', 'V', 'E', 'R'];
        let mut j = 0;
        for i in 25..34 {
            self.pantalla.pixeles[10][i] = texto[j];
            j += 1
        }
        self.pantalla.renderizar_sin_cabecera();
    }
}
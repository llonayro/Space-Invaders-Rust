# 👾 Space Invaders — Rust Edition · UAGRM 🦀

**Estudiante:** Grovert Meneses Zegarra  
**Carrera:** Ingeniería en Sistemas  
**Materia:** Programación I — FICCT  
**Docente:** Msc. Ing. Víctor Hugo Acosta Ortega  
**Facultad:** FICCT — Universidad Autónoma Gabriel René Moreno · Santa Cruz de la Sierra, Bolivia

---

## 📋 Descripción

Este proyecto es una implementación del clásico juego **Space Invaders** desarrollado íntegramente en **Rust** como proyecto práctico para la materia de Programación I. El juego se ejecuta en la terminal y pone a prueba conceptos avanzados de lógica, gestión de memoria y Programación Orientada a Objetos (POO).

## 🚀 Características Implementadas

*   **IA de Enemigos:** Disparos aleatorios con probabilidad dinámica (aumenta cuando el alien está alineado con el jugador).
*   **Gestión de Memoria:** Uso eficiente de referencias mutables y `Borrow Checker` para el manejo de proyectiles y colisiones.
*   **Sistema de Azar:** Implementación mediante la crate `my_random` para eventos aleatorios.
*   **Interfaz de Usuario:** Pantallas de *Game Over*, *Victoria* y controles de salida fluida (`Q`).
*   **Motor de Renderizado:** Sistema de dibujado por caracteres en consola con limpieza de frames.

## 🛠️ Tecnologías y Conceptos de POO

El proyecto aplica los pilares de la programación enseñados en la FICCT:

| Concepto POO | Implementación en este Proyecto |
| :--- | :--- |
| **Clases / Objetos** | `struct Jugador`, `struct Alien`, `struct Disparo` |
| **Encapsulamiento** | Uso de `pub` y módulos para proteger la lógica del juego. |
| **Composición** | `struct Juego` que contiene vectores de `Alien` y `Disparo`. |
| **Constructores** | Métodos asociados `new()` para inicializar entidades. |
| **Estado Mutable** | Métodos `&mut self` para actualizar posiciones y salud. |

## 🕹️ Controles

*   **Izquierda:** Mover nave a la izquierda.
*   **Derecha:** Mover nave a la derecha.
*   **Espacio:** Disparar.
*   **Q:** Salir del juego.

## ⚙️ Requisitos e Instalación

Asegúrate de tener el entorno de Rust configurado:

```bash
# Verificar instalación de Rust
cargo --version
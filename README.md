# 🏴‍☠️ La Bitácora de Jack: Chatbot con Gemini en Rust

¡Ah del barco! Este proyecto es una aplicación web completa que utiliza un servidor (Backend) escrito en **Rust** y una interfaz de usuario (Frontend) en **HTML, CSS y JavaScript**. Conecta a los usuarios con la Inteligencia Artificial de Google (Gemini), configurada mediante _System Instructions_ para actuar como un pirata informático.

---

## ✨ Características Principales

- **Backend ultrarrápido:** Servidor web asíncrono construido en Rust utilizando el framework `axum`.
- **Integración de IA (API REST):** Conexión HTTP directa con la API de Google Gemini (`gemini-2.5-flash`).
- **Memoria Conversacional:** El chat recuerda el contexto de la conversación, manteniendo un historial fluido.
- **Ingeniería de Prompts (Persona):** Uso de _System Instructions_ para inyectar un comportamiento específico a la IA antes de que el usuario hable.
- **Interfaz Temática:** Diseño web inmersivo en CSS (colores de pergamino, madera y botones 3D).

---

## 📂 Estructura del Proyecto

El proyecto sigue una arquitectura clásica de separación entre Cliente y Servidor:

```text
gemini_chat/
├── Cargo.toml          # Gestor de dependencias y configuración de Rust
├── .env                # Variables de entorno (¡NO SUBIR A GITHUB!)
├── src/
│   └── main.rs         # Código fuente del Backend (Servidor web y lógica API)
└── static/
    └── index.html      # Código fuente del Frontend (Interfaz de usuario y lógica JS)
```

🛠️ Requisitos Previos
Para ejecutar este proyecto en tu máquina local, necesitarás:

Rust y Cargo instalados.

Una clave API gratuita de Google AI Studio.

🚀 Instalación y Uso

1. Clonar o descargar el proyecto
   Abre tu terminal y navega hasta la carpeta del proyecto.

2. Configurar la Clave API
   Crea un archivo llamado .env en la raíz del proyecto (al mismo nivel que Cargo.toml) y añade tu clave API de Gemini sin comillas:

GEMINI_API_KEY=tu_clave_api_aqui

3. Ejecutar el Servidor
   Compila y ejecuta el backend en Rust con el siguiente comando:

Bash
cargo run 4. Abrir la Aplicación
Abre tu navegador web favorito y dirígete a:
👉 http://127.0.0.1:8080

🧠 Conceptos Aprendidos en este Proyecto
Durante la creación de esta aplicación, hemos puesto en práctica:

Asincronía y Concurrencia (tokio): Manejo de peticiones web sin bloquear el hilo principal.

Manejo de Errores en Rust (Result, ?, match): Evitando que el programa colapse mediante una gestión limpia de fallos (ej. el puerto en uso o errores 404 de la API).

Serialización y Deserialización JSON (serde): Traducción de estructuras de datos de Rust a un formato legible por la web y viceversa.

Arquitectura Stateless: Entender que el backend no guarda estado; es el navegador (JavaScript) quien almacena y envía el historial de la conversación.

Deprecation (Obsolescencia): Actualización de modelos de IA retirados (1.5-flash a 2.5-flash) y lectura de documentación de terceros.

"No todos los tesoros son de plata y oro, grumete... algunos son líneas de código en Rust." - Jack Sparrow

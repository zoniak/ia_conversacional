use reqwest::Client;
use serde_json::{Value, json}; // Importamos Value para poder tipar nuestro vector
use std::env;
use std::fs;
use std::io::{self, Write}; // <-- NUEVO: Para interactuar con archivos

// Usamos Tokio para permitir funciones asíncronas
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cargar la variable de entorno de forma segura
    dotenvy::dotenv().ok();
    // Modifica la línea donde se lee la clave añadiendo .trim()
    let api_key = env::var("GEMINI_API_KEY")
        .expect("¡Falta la variable GEMINI_API_KEY en el archivo .env!")
        .trim() // <-- NUEVO: Elimina espacios, tabulaciones y saltos de línea ocultos
        .to_string();

    // 2. Configurar el cliente HTTP y la URL (usamos el modelo flash por ser rápido y económico)
    let client = Client::new();
    // AHORA (Modelo actualizado y activo):
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    // Imprimimos los primeros 10 caracteres de la clave y ocultamos el resto por seguridad
    let safe_url = url.replace(&api_key[10..], "********");
    println!(
        "🔍 [DEBUG] La URL exacta que estamos llamando es:\n{}\n",
        safe_url
    );

    println!("========================================");
    println!("Bienvenido al Chat con Gemini en Rust");
    println!("(Escribe 'salir' para terminar el programa)");
    println!("========================================\n");

    // 3. Bucle infinito para mantener el chat abierto
    // NUEVO: Creamos un Vector mutable para almacenar el historial de la conversación.
    let mut history: Vec<Value> = Vec::new();

    loop {
        print!("Tú: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("salir") {
            println!("¡Hasta luego!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // 1. Añadimos el mensaje actual del usuario al historial
        history.push(json!({
            "role": "user",
            "parts": [{"text": input}]
        }));

        // 2. Construimos el cuerpo de la petición enviando TODO el historial
        let body = json!({
            "contents": history
        });

        // 3. Enviamos la petición
        let response = client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            let json_res: Value = response.json().await?;

            // Extraemos la respuesta
            if let Some(text) = json_res["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                let response_text = text.trim();
                println!("Gemini: {}\n", response_text);

                // 4. NUEVO: Añadimos la respuesta de Gemini al historial
                history.push(json!({
                    "role": "model",
                    "parts": [{"text": response_text}]
                }));

                // ==========================================
                // NUEVO: GUARDAR EL HISTORIAL EN UN ARCHIVO
                // ==========================================
                // 1. Convertimos el vector 'history' a un texto JSON formateado (bonito y legible)
                let historial_json = serde_json::to_string_pretty(&history)?;

                // 2. Lo escribimos en un archivo. Si no existe, lo crea. Si existe, lo sobrescribe.
                fs::write("historial_chat.json", historial_json)?;
                // ==========================================

                //Cuando no quiera guardar el historial eliminar las lineas 1. y 2.
            } else {
                println!("Gemini: [No se pudo extraer el texto de la respuesta]\n");
                // Si algo falla al procesar, sacamos el último mensaje del usuario
                // para no romper la alternancia "user" -> "model"
                history.pop();
            }
        } else {
            println!("Error en la petición: {:?}\n", response.status());
            // Si la API da error, retiramos el último mensaje del usuario del historial
            history.pop();
        }
    }

    Ok(())
}

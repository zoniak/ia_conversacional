use reqwest::Client;
use serde_json::json;
use std::env;
use std::io::{self, Write};

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
    loop {
        print!("Tú: ");
        io::stdout().flush()?; // Asegura que el prompt se imprima antes de leer

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Condición de salida
        if input.eq_ignore_ascii_case("salir") {
            println!("¡Hasta luego!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // 4. Construir el cuerpo de la petición (JSON) según la documentación de Gemini
        let body = json!({
            "contents": [{
                "parts": [{"text": input}]
            }]
        });

        // 5. Enviar la petición de forma asíncrona
        let response = client.post(&url).json(&body).send().await?;

        // 6. Procesar la respuesta
        if response.status().is_success() {
            let json_res: serde_json::Value = response.json().await?;

            // Navegamos por el JSON para extraer el texto generado
            if let Some(text) = json_res["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                println!("Gemini: {}\n", text.trim());
            } else {
                println!("Gemini: [No se pudo extraer el texto de la respuesta]\n");
            }
        } else {
            println!("Error en la petición: {:?}\n", response.status());
        } //fin if
    }

    Ok(())
}

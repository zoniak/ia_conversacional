use axum::{
    Json, Router,
    routing::{get, post},
};
use reqwest::Client;
use serde_json::{Value, json};
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // 1. Construimos el "Router" de nuestro servidor web
    let app = Router::new()
        // Servimos la carpeta 'static' (donde está el index.html) en la ruta principal "/"
        .fallback_service(ServeDir::new("static"))
        // Creamos una ruta POST para que el JavaScript envíe los mensajes
        .route("/api/chat", post(manejar_chat));

    // AHORA (por ejemplo, puerto 8080):
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Servidor web iniciado en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Esta función se ejecuta cada vez que JavaScript hace un POST a "/api/chat"
/// Recibe el historial completo desde el navegador en formato JSON.
async fn manejar_chat(Json(historial): Json<Vec<Value>>) -> Json<Value> {
    let api_key = env::var("GEMINI_API_KEY")
        .expect("Falta GEMINI_API_KEY")
        .trim()
        .to_string();
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    // Empaquetamos el historial que nos dio JS y se lo mandamos a Google
    // AHORA: Le inyectamos la personalidad usando "system_instruction"
    let body = json!({
        "system_instruction": {
            "parts": [{
                "text": "Eres un pirata informático del siglo XVIII. Responde a todo usando jerga pirata, llamando 'grumete' al usuario y haciendo referencias al mar, los barcos y el código binario como si fuera el tesoro."
            }]
        },
        "contents": historial
    });

    // Hacemos la petición a Gemini
    let response = client.post(&url).json(&body).send().await;

    // Procesamos la respuesta y la devolvemos a nuestro frontend
    match response {
        Ok(res) if res.status().is_success() => {
            if let Ok(json_res) = res.json::<Value>().await {
                if let Some(text) =
                    json_res["candidates"][0]["content"]["parts"][0]["text"].as_str()
                {
                    // Devolvemos el texto al frontend en formato JSON: { "texto": "..." }
                    return Json(json!({ "texto": text.trim() }));
                }
            }
            Json(json!({ "texto": "Error extrayendo el texto de Gemini." }))
        }
        Ok(res) => Json(json!({ "texto": format!("Error de API: {}", res.status()) })),
        Err(e) => Json(json!({ "texto": format!("Error de conexión: {}", e) })),
    }
}

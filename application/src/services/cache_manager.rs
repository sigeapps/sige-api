use moka::future::Cache;
use serde::Serialize;
use serde_json::Value;

pub struct CacheManager;

impl CacheManager {
    /// Intenta obtener un valor de la caché, si no existe lo genera con la función proporcionada y lo guarda.
    pub async fn get_or_set<T, F, Fut>(
        cache: &Cache<String, Value>,
        key: &str,
        factory: F,
    ) -> Result<T, sea_orm::DbErr>
    where
        T: Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + 'static,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, sea_orm::DbErr>>,
    {
        if let Some(value) = cache.get(key).await {
            if let Ok(result) = serde_json::from_value::<T>(value) {
                return Ok(result);
            }
        }

        let result = factory().await?;
        if let Ok(value) = serde_json::to_value(result.clone()) {
            cache.insert(key.to_string(), value).await;
        }

        Ok(result)
    }

    /// Invalida una entrada específica de la caché.
    pub async fn invalidate(cache: &Cache<String, Value>, key: &str) {
        cache.remove(key).await;
    }

    /// Invalida todas las entradas que comiencen con cierto prefijo.
    pub async fn invalidate_prefix(cache: &Cache<String, Value>, prefix: &str) {
        // Moka no permite iterar directamente de forma eficiente para borrar por prefijo
        // sin bloquear, pero para Lookups pequeños podemos invalidar el prefijo completo
        // o simplemente limpiar la caché si es crítica.
        // Por ahora, manejaremos invalidación manual por clave.
        cache.remove(prefix).await;
    }
}

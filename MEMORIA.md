# Memoria del Proyecto: SIGE API

## Descripción General
El proyecto **SIGE API** (Sistema Integral de Gestión de Empresas) es un backend desarrollado en **Rust** usando **Arquitectura Limpia (Clean Architecture)**. Está diseñado para gestionar operaciones de prevención que incluyen registros de personal, comisiones de servicio, aislamientos temporales, transporte y sus respectivos datos organizacionales.
compilar con cargo build o cargo dev


## Stack Tecnológico 🛠️
- **Lenguaje:** Rust (edición 2021)
- **Framework Web:** Axum
- **ORM / Base de datos:** SeaORM con PostgreSQL
- **Runtime:** Tokio (asíncrono)
- **Serialización:** Serde
- **Documentación de API:** Utoipa (OpenAPI)
- **Log / Trace:** Tracing y Tracing-subscriber
- **Caché Internal:** Moka (Lock-free, en memoria)
- **Compresión:** Gzip / Brotli (vía tower-http)

## Arquitectura y Escalabilidad 🚀
El sistema ha sido optimizado para hardware de alto rendimiento (128GB RAM, 32 Cores, Dual CPU):

1. **Gestión de Conexiones:** Pool de base de datos ampliado a 100 conexiones simultáneas para maximizar el paralelismo de los 32 hilos.
2. **Caché de Capa de Aplicación:** Implementación de `CacheManager` con Moka para automatizar la persistencia en RAM de consultas frecuentes (Lookups), reduciendo la latencia de red y carga en DB.
3. **Optimización de Binario:** Perfil de `release` configurado con LTO (Link Time Optimization), `codegen-units = 1` y eliminación de símbolos (strip) para ejecución ultra-fluida.
4. **Eficiencia de Red:** Compresión automática de respuestas (Gzip/Brotli) para mejorar la velocidad en conexiones lentas a través de Internet.

## Arquitectura (Clean Architecture) 🏗️
El sistema sigue los principios de Clean Architecture agrupando el código en sub-paquetes (workspaces):

1. **`domain` (Dominio):** 
   - Capa más profunda, contiene la lógica de negocio core y las entidades base. No depende de ningún otro módulo del proyecto. Define los repositorios en forma de interfaces.

2. **`application` (Aplicación):** 
   - Contiene los servicios o casos de uso del sistema. Depende de `domain`. Aquí se definen los "Data Transfer Objects" (DTOs) e implementa la orquestación lógica para los flujos.

3. **`infrastructure/web` (Infraestructura Web):** 
   - Capa superficial que utiliza **Axum** para el enrutamiento y servir los endpoints de la API. Depende del módulo `application` y de `domain`. Contiene la serialización y la capa HTTP.

4. **`migration` (Migraciones):** 
   - Contiene los esquemas y migraciones para la base de datos a través de SeaORM.

5. **`src` (Punto de entrada general):**
   - Configura el inicio del servidor, leyendo parámetros como los definidos en `settings.json` o variables de entorno mediante `clap` (Argumentos CLI) y `config` (Gestor de configuración).

## Funcionalidad Base Mapeada 📋
El servidor gestiona:
- Autenticación y Autorización de empresas (Login/Registro).
- Múltiples aspectos de control de Seguridad y Prevención (`/prevention/...`):
  - Registros de personal (Entrada/Salida).
  - Control de Comisiones de Servicio.
  - Aislamientos temporales (con seguimiento de visitas).
  - Asignación de oficiales y recursos de transporte (vehículos).
- Un listado de catálogos o entidades maestras (`/lookup/...`) como Brigadas, Organismos, Divisiones, Jerarquías, entre otros.

## Interacción con el Ecosistema SIGE 🌐
El ecosistema completo se compone de 3 servicios que interactúan de la siguiente manera:
1. **sige-api (Este proyecto):** Actúa como el motor principal expuesto en `192.168.1.99:8443`. Gestiona la base de datos PostgreSQL (`192.168.1.99:5432/sige`).
2. **sige-dashboard:** Consume los endpoints REST de este proyecto para operaciones CRUD.
3. **sige-io:** Escucha inserciones lógicas directamente en la base de datos compartida para notificar al dashboard en tiempo real.

**⚠️ Impacto de Actualizaciones:** 
Este proyecto es la fuente de verdad del esquema de datos. Cualquier migración aquí (`migration/`) impacta directamente la capacidad de **sige-io** para leer eventos y obliga a **sige-dashboard** a actualizar sus tipos de TypeScript.

## Estado Actual
- Análisis de estructura y dependencias completado.
- Configuración de red identificada (`settings.json` apunta a `192.168.1.99`).
- Memoria interna sincronizada con el grafo de conocimiento.
- **Optimización de Alto Rendimiento:** Implementada caché interna, pool de conexiones de alto volumen y perfil de release optimizado para hardware de 128GB RAM.
- **Mejora de Logs (Tracing):** Sistema actualizado para mostrar logs más limpios e identificar automáticamente al usuario en cada consulta SQL y petición HTTP.
- **Compresión Activa:** Respuestas comprimidas automáticamente para fluidez en accesos remotos vía Internet.
- **Sistema de Permisos:** Actualizado el 19/03/2026 para incluir permisos granulares individuales para los módulos de Parque, Operaciones y Análisis, permitiendo una selección independiente en la UI y sincronización con roles administrativos.

---
*Este archivo se mantendrá actualizado conforme realicemos desarrollos e identifiquemos nuevos patrones del proyecto o en sus integraciones.*

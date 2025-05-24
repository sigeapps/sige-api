# SIGE API

**Sistema Integral de Gestión de Empresas - API REST**

SIGE API es un sistema backend desarrollado en Rust para la gestión integral de empresas y operaciones de prevención. El sistema está diseñado para administrar registros de personal, comisiones de servicio, aislamientos temporales, transporte y datos organizacionales.

## 🏗️ Arquitectura

El proyecto implementa una **Arquitectura Limpia (Clean Architecture)** con separación clara de responsabilidades:

```sh
sige-api/
├── src/                     # Punto de entrada de la aplicación
├── domain/                  # Entidades de dominio y lógica de negocio
├── application/             # Servicios de aplicación y casos de uso
├── infrastructure/          # Capa de infraestructura
│   └── web/                # Framework web (Axum)
├── migration/              # Migraciones de base de datos
└── requests/               # Ejemplos de peticiones HTTP
```

### Capas de la Arquitectura

- **Domain**: Entidades de negocio, repositorios y reglas de dominio
- **Application**: Servicios de aplicación, DTOs y casos de uso
- **Infrastructure**: Implementaciones concretas (web, base de datos)
- **Presentation**: Controladores y rutas HTTP

## 🛠️ Tecnologías

| Tecnología | Propósito |
|------------|-----------|
| **Rust** | Lenguaje de programación principal |
| **Axum** | Framework web asíncrono |
| **SeaORM** | ORM para base de datos |
| **PostgreSQL** | Base de datos relacional |
| **Tokio** | Runtime asíncrono |
| **Serde** | Serialización/deserialización JSON |
| **Clap** | Interfaz de línea de comandos |
| **Tracing** | Sistema de logging estructurado |

## 📋 Funcionalidades

### 🔐 Autenticación y Autorización
- Registro de empresas
- Login con sesiones
- Sistema de roles y permisos

### 📝 Gestión de Registros
- **Registros de entrada/salida**: Control de acceso de personal
- **Seguimiento temporal**: Fechas de entrada y salida
- **Información personal**: CI, nombres, organismo, división
- **Observaciones**: Notas adicionales sobre visitas

### 🚨 Comisiones de Servicio
- **Gestión de comisiones**: Creación y seguimiento de operaciones
- **Personal asignado**: Oficiales y responsables
- **Transporte**: Vehículos asignados y estado
- **Seguimiento temporal**: Entrada, salida y estado actual
- **Razones y motivos**: Documentación de causas

### 🔒 Aislamientos Temporales
- **Gestión de aislamientos**: Control de personas en aislamiento
- **Visitas**: Registro de visitas durante el aislamiento
- **Estados**: Seguimiento del estado del aislamiento
- **Fechas**: Control temporal de inicio y fin

### 🚗 Gestión de Transporte
- **Vehículos**: Registro de vehículos disponibles
- **Estados**: Disponible, en uso, mantenimiento
- **Tipos**: Clasificación de vehículos
- **Marcas y modelos**: Información detallada

### 📊 Datos de Consulta (Lookup)
- **Organismos**: Entidades organizacionales
- **Divisiones**: Subdivisiones administrativas
- **Brigadas**: Unidades operativas
- **Cargos**: Posiciones y rangos
- **Jerarquías**: Estructura jerárquica
- **Estados y municipios**: Ubicaciones geográficas

## ⚙️ Instalación y Configuración

### Prerrequisitos

- **Rust** (1.70 o superior)
- **PostgreSQL** (12 o superior)
- **Git**

### 1. Clonar el Repositorio

```bash
git clone <url-del-repositorio>
cd sige-api
```

### 2. Configurar Base de Datos

```bash
# Crear base de datos PostgreSQL
createdb sige_dev

# Configurar variables de entorno (opcional)
export DATABASE_URL="postgresql://sige:sige123@localhost:5432/sige_dev"
```

### 3. Configurar settings.json

```json
{
  "database": {
    "url": "postgresql://sige:sige123@localhost:5432/sige_dev"
  },
  "address": {
    "host": "127.0.0.1",
    "port": 1234
  },
  "logging": {
    "log_level": "debug"
  }
}
```

### 4. Ejecutar Migraciones

```bash
cd migration
cargo run
```

### 5. Compilar y Ejecutar

```bash
# Modo desarrollo
cargo run dev

# Modo producción
cargo run start
```

## 🚀 API Endpoints

### Autenticación

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `POST` | `/register` | Registrar nuevo usuario |
| `POST` | `/login` | Iniciar sesión |

### Registros de Personal

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `GET` | `/prevention/register` | Listar todos los registros |
| `POST` | `/prevention/register` | Crear nuevo registro |
| `GET` | `/prevention/register/{id}` | Obtener registro por ID |
| `PATCH` | `/prevention/register/{id}` | Actualizar salida de registro |

### Comisiones

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `GET` | `/prevention/commission` | Listar comisiones |
| `POST` | `/prevention/commission` | Crear nueva comisión |
| `GET` | `/prevention/commission/{id}` | Obtener comisión por ID |
| `PATCH` | `/prevention/commission/{id}/exit` | Actualizar salida |
| `PATCH` | `/prevention/commission/{id}/status` | Actualizar estado |

### Aislamientos

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `GET` | `/prevention/seclusion` | Listar aislamientos |
| `POST` | `/prevention/seclusion` | Crear nuevo aislamiento |
| `GET` | `/prevention/seclusion/{id}` | Obtener aislamiento por ID |
| `PATCH` | `/prevention/seclusion/{id}` | Actualizar salida |
| `POST` | `/prevention/seclusion/{id}/visit` | Agregar visita |

### Oficiales y Transporte

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `GET` | `/prevention/official` | Listar oficiales |
| `POST` | `/prevention/official` | Crear oficial |
| `GET` | `/prevention/transport` | Listar transportes |
| `POST` | `/prevention/transport` | Crear transporte |

### Datos de Consulta

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `GET/POST` | `/lookup/brigade` | Brigadas |
| `GET/POST` | `/lookup/organism` | Organismos |
| `GET/POST` | `/lookup/division` | Divisiones |
| `GET/POST` | `/lookup/charge` | Cargos |
| `GET/POST` | `/lookup/hierarchy` | Jerarquías |
| `GET/POST` | `/lookup/state` | Estados |
| `GET/POST` | `/lookup/municipality` | Municipios |
| `GET/POST` | `/lookup/transport-type` | Tipos de transporte |
| `GET/POST` | `/lookup/transport-status` | Estados de transporte |
| `GET/POST` | `/lookup/brand` | Marcas |
| `GET/POST` | `/lookup/vehicle-model` | Modelos de vehículos |

## 📖 Ejemplos de Uso

### Registrar Usuario

```bash
curl -X POST http://127.0.0.1:1234/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "usuario",
    "password": "contraseña"
  }'
```

### Crear Registro de Entrada

```bash
curl -X POST http://127.0.0.1:1234/prevention/register \
  -H "Content-Type: application/json" \
  -d '{
    "ci": "12345678",
    "first_name": "Juan",
    "last_name": "Pérez",
    "visit_reason": "Reunión administrativa",
    "is_official": false
  }'
```

### Listar Comisiones

```bash
curl http://127.0.0.1:1234/prevention/commission
```

## 🗄️ Estructura de Base de Datos

### Entidades Principales

- **users**: Usuarios del sistema
- **register**: Registros de entrada/salida
- **commission**: Comisiones de servicio
- **seclusion**: Aislamientos temporales
- **official**: Personal oficial
- **transport**: Vehículos y transporte

### Entidades de Lookup

- **brigade**: Brigadas
- **organism**: Organismos
- **division**: Divisiones
- **charge**: Cargos
- **hierarchy**: Jerarquías
- **state**: Estados
- **municipality**: Municipios

## 🔧 Desarrollo

### Comandos Útiles

```bash
# Compilar
cargo build

# Ejecutar tests
cargo test

# Formato de código
cargo fmt

# Linter
cargo clippy

# Generar nueva migración
cd migration
cargo run -- generate nombre_migracion

# Aplicar migraciones
cd migration
cargo run

# Verificar estado de migraciones
cd migration
cargo run -- status
```

### Estructura de Directorios

```
src/
├── main.rs              # Punto de entrada
├── args.rs              # Argumentos CLI
├── settings.rs          # Configuración
├── lib.rs               # Biblioteca principal
└── commands/            # Comandos CLI
    └── start.rs         # Comando de inicio

domain/src/
├── entities/            # Entidades de dominio
├── repositories/        # Interfaces de repositorio
└── value_objects/       # Objetos de valor

application/src/
├── services/            # Servicios de aplicación
├── dtos/               # Objetos de transferencia
└── types.rs            # Tipos comunes

infrastructure/web/src/
├── routes/             # Definición de rutas
├── controllers/        # Controladores HTTP
├── auth.rs             # Autenticación
└── state.rs            # Estado de aplicación
```

## 📝 Configuración

### Variables de Entorno

- `DATABASE_URL`: URL de conexión a PostgreSQL
- `RUST_LOG`: Nivel de logging (debug, info, warn, error)

### Archivo settings.json

El archivo `settings.json` en la raíz del proyecto contiene la configuración principal:

```json
{
  "database": {
    "url": "postgresql://usuario:contraseña@host:puerto/base_datos"
  },
  "address": {
    "host": "127.0.0.1",
    "port": 1234
  },
  "logging": {
    "log_level": "debug"
  }
}
```

## 🤝 Contribuir

1. Hacer fork del proyecto
2. Crear una rama para la funcionalidad (`git checkout -b feature/nueva-funcionalidad`)
3. Commit de cambios (`git commit -am 'Agregar nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Crear Pull Request

## 📄 Licencia

[Especificar licencia del proyecto]

## 🐛 Reporte de Bugs

Para reportar bugs o solicitar funcionalidades, crear un issue en el repositorio del proyecto.

## 📞 Soporte

Para soporte técnico o consultas sobre el proyecto, contactar al equipo de desarrollo.

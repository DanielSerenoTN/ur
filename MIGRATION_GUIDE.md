# Migración de MySQL a PostgreSQL - Guía de Migración

## 🔄 Cambios Realizados

### 1. Dependencias actualizadas
- Cambiado `diesel` de `mysql` a `postgres` en `Cargo.toml`
- Agregado soporte para `uuid` en Diesel

### 2. Configuración de base de datos
- `src/db/mod.rs`: Migrado de `MysqlConnection` a `PgConnection`
- `diesel.toml`: Configurado para PostgreSQL

### 3. Migraciones adaptadas
- Todas las migraciones en `/migrations` fueron convertidas a sintaxis PostgreSQL
- Creados triggers para `updated_at` automático (reemplaza `ON UPDATE CURRENT_TIMESTAMP`)
- Removidas opciones específicas de MySQL como `ENGINE=InnoDB`

### 4. Docker actualizado
- `docker-compose.yml`: Agregado servicio PostgreSQL
- Configuración completa con volúmenes persistentes

### 5. Repositorios actualizados
- `interactive_maps_repository.rs`: Migrado a `PgConnection`
- `products_repository.rs`: Migrado a `PgConnection` 
- Cambiado `debug_query::<Mysql, _>` a `debug_query::<Pg, _>`

## 🚀 Pasos para completar la migración

### 1. Instalar herramientas de PostgreSQL
```bash
# Instalar diesel_cli para PostgreSQL
cargo install diesel_cli --no-default-features --features postgres
```

### 2. Configurar variables de entorno
Copia `.env.example` a `.env` y configura:
```bash
cp .env.example .env
```

Edita las variables en `.env`:
```bash
# PostgreSQL Configuration
POSTGRES_USER=urvic_user
POSTGRES_PASSWORD=urvic_password  
POSTGRES_DB=urvic_db
DATABASE_URL=postgresql://urvic_user:urvic_password@localhost:5432/urvic_db
```

### 3. Migrar datos (si tienes datos existentes)
```bash
# 1. Exportar datos de MySQL
mysqldump -u root -p urvic_db > backup_mysql.sql

# 2. Convertir dump de MySQL a PostgreSQL (manual o con herramientas)
# Puedes usar pgloader o convertir manualmente

# 3. Importar a PostgreSQL
psql -U urvic_user -d urvic_db -f converted_data.sql
```

### 4. Levantar con Docker
```bash
# Construir y levantar servicios
docker-compose up --build

# O en modo detached
docker-compose up -d --build
```

### 5. Ejecutar migraciones
```bash
# Dentro del contenedor o con PostgreSQL local
diesel migration run

# O si prefieres hacerlo manualmente
psql -U urvic_user -d urvic_db -f migrations/*/up.sql
```

## 🔧 Cambios técnicos importantes

### Tipos de datos convertidos:
- `LONGTEXT` → `TEXT` (PostgreSQL TEXT es ilimitado)
- `DATETIME` → `TIMESTAMP`
- `VARCHAR(n)` → `VARCHAR(n)` (compatible)

### Triggers para updated_at:
PostgreSQL no soporta `ON UPDATE CURRENT_TIMESTAMP`, por lo que se creó:
```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';
```

### Sintaxis SQL actualizada:
- `ALTER TABLE ... CHANGE COLUMN` → `ALTER TABLE ... RENAME COLUMN`
- `ALTER TABLE ... MODIFY COLUMN` → `ALTER TABLE ... ALTER COLUMN`

## 🧪 Testing

### Verificar conexión:
```bash
# Test de conexión a PostgreSQL
psql -U urvic_user -h localhost -d urvic_db -c "SELECT 1;"
```

### Verificar migraciones:
```bash
# Ver tablas creadas
psql -U urvic_user -d urvic_db -c "\dt"

# Ver estructura de tabla
psql -U urvic_user -d urvic_db -c "\d maps_svg"
```

### Test del API:
```bash
# Una vez levantado el servicio
curl http://localhost:20040/swagger-ui/
```

## 📝 Notas importantes

1. **Backup**: Siempre haz backup de tu BD MySQL antes de migrar
2. **Datos**: Los datos existentes necesitan ser migrados manualmente
3. **Performance**: PostgreSQL puede tener diferentes características de performance
4. **Queries**: La mayoría de queries Diesel son compatibles, pero revisa logs por errores

## 🚨 Problemas comunes

### Error de conexión:
- Verifica que PostgreSQL esté ejecutándose
- Confirma credenciales en `.env`
- Revisa que el puerto 5432 esté disponible

### Error en migraciones:
- Ejecuta las migraciones una por una si hay problemas
- Verifica que la función `update_updated_at_column()` se creó correctamente

### Error de compilación:
- Ejecuta `cargo clean` y `cargo build` después de los cambios
- Verifica que no haya imports de `diesel::mysql` restantes

¡La migración está lista! 🎉
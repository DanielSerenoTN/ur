#!/bin/bash

echo "=== INSTALANDO URVIC API ==="

# Verificar que estamos en el directorio correcto
if [ ! -f "urvic-backend" ]; then
    echo "❌ Error: No se encontró el archivo urvic-backend"
    echo "Asegúrate de estar en el directorio correcto"
    exit 1
fi

if [ ! -f ".env" ]; then
    echo "❌ Error: No se encontró el archivo .env"
    exit 1
fi

if [ ! -f "setup-db.sql" ]; then
    echo "❌ Error: No se encontró el archivo setup-db.sql"
    exit 1
fi

# Dar permisos de ejecución al binario
echo "📁 Configurando permisos..."
chmod +x urvic-backend

# Leer credenciales del archivo .env
echo "🔧 Leyendo configuración..."
DB_USER=$(grep "MYSQL_USER" .env | cut -d'=' -f2)
DB_PASS=$(grep "MYSQL_PASSWORD" .env | cut -d'=' -f2)
DB_NAME=$(grep "MYSQL_DB" .env | cut -d'=' -f2)

echo "Usuario de BD: $DB_USER"
echo "Base de datos: $DB_NAME"

# Ejecutar migraciones
echo "🗄️  Ejecutando migraciones de base de datos..."
mysql -u "$DB_USER" -p"$DB_PASS" "$DB_NAME" < setup-db.sql

if [ $? -eq 0 ]; then
    echo "✅ Migraciones ejecutadas exitosamente!"
    
    # Verificar tablas creadas
    echo "📋 Verificando tablas creadas:"
    mysql -u "$DB_USER" -p"$DB_PASS" "$DB_NAME" -e "SHOW TABLES;"
    
    echo ""
    echo "✅ Instalación completada exitosamente!"
    echo ""
    echo "Para iniciar el servidor, ejecuta:"
    echo "   ./start.sh"
    echo ""
    echo "La API estará disponible en:"
    echo "   http://tu-servidor:20090/api/status-colors"
    echo "   http://tu-servidor:20090/swagger-ui/"
else
    echo "❌ Error al ejecutar las migraciones"
    echo "Verifica las credenciales de la base de datos"
    exit 1
fi
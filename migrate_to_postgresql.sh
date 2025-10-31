#!/bin/bash

# Script de migración de MySQL a PostgreSQL para Urvic Backend
# Asegúrate de tener Docker y Docker Compose instalados

set -e

echo "🚀 Iniciando migración de MySQL a PostgreSQL..."

# Verificar si existe .env
if [ ! -f ".env" ]; then
    echo "📋 Copiando .env.example a .env..."
    cp .env.example .env
    echo "⚠️  Por favor edita el archivo .env con tus configuraciones antes de continuar"
    echo "   Presiona Enter cuando hayas configurado el .env..."
    read
fi

echo "🧹 Limpiando builds anteriores..."
cargo clean

echo "📦 Instalando dependencias..."
cargo fetch

echo "🏗️  Compilando proyecto..."
cargo build --release

echo "🐘 Levantando PostgreSQL..."
docker-compose up postgres -d

echo "⏳ Esperando que PostgreSQL esté listo..."
sleep 10

# Verificar si PostgreSQL está funcionando
echo "🔍 Verificando conexión a PostgreSQL..."
until docker-compose exec postgres pg_isready -U ${POSTGRES_USER:-urvic_user} > /dev/null 2>&1; do
    echo "   PostgreSQL aún no está listo, esperando..."
    sleep 2
done

echo "✅ PostgreSQL está funcionando!"

# Instalar diesel_cli si no está instalado
if ! command -v diesel &> /dev/null; then
    echo "🔧 Instalando diesel_cli para PostgreSQL..."
    cargo install diesel_cli --no-default-features --features postgres
fi

echo "📊 Ejecutando migraciones..."
diesel migration run

echo "🏃‍♂️ Levantando aplicación completa..."
docker-compose up --build -d

echo "🎉 ¡Migración completada!"
echo ""
echo "📋 Información importante:"
echo "   - API: http://localhost:${APP_PORT:-20040}"
echo "   - Swagger UI: http://localhost:${APP_PORT:-20040}/swagger-ui/"
echo "   - PostgreSQL: localhost:5432"
echo ""
echo "📝 Para verificar que todo funciona:"
echo "   docker-compose logs -f"
echo ""
echo "🛑 Para detener los servicios:"
echo "   docker-compose down"
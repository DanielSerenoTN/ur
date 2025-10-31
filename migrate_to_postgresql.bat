@echo off
REM Script de migración de MySQL a PostgreSQL para Urvic Backend
REM Asegúrate de tener Docker y Docker Compose instalados

echo 🚀 Iniciando migración de MySQL a PostgreSQL...

REM Verificar si existe .env
if not exist ".env" (
    echo 📋 Copiando .env.example a .env...
    copy .env.example .env
    echo ⚠️  Por favor edita el archivo .env con tus configuraciones antes de continuar
    echo    Presiona Enter cuando hayas configurado el .env...
    pause
)

echo 🧹 Limpiando builds anteriores...
cargo clean

echo 📦 Instalando dependencias...
cargo fetch

echo 🏗️  Compilando proyecto...
cargo build --release

echo 🐘 Levantando PostgreSQL...
docker-compose up postgres -d

echo ⏳ Esperando que PostgreSQL esté listo...
timeout /t 10 /nobreak >nul

echo 🔍 Verificando conexión a PostgreSQL...
:wait_postgres
docker-compose exec postgres pg_isready -U urvic_user >nul 2>&1
if %errorlevel% neq 0 (
    echo    PostgreSQL aún no está listo, esperando...
    timeout /t 2 /nobreak >nul
    goto wait_postgres
)

echo ✅ PostgreSQL está funcionando!

REM Verificar si diesel está instalado
where diesel >nul 2>&1
if %errorlevel% neq 0 (
    echo 🔧 Instalando diesel_cli para PostgreSQL...
    cargo install diesel_cli --no-default-features --features postgres
)

echo 📊 Ejecutando migraciones...
diesel migration run

echo 🏃‍♂️ Levantando aplicación completa...
docker-compose up --build -d

echo 🎉 ¡Migración completada!
echo.
echo 📋 Información importante:
echo    - API: http://localhost:20040
echo    - Swagger UI: http://localhost:20040/swagger-ui/
echo    - PostgreSQL: localhost:5432
echo.
echo 📝 Para verificar que todo funciona:
echo    docker-compose logs -f
echo.
echo 🛑 Para detener los servicios:
echo    docker-compose down

pause
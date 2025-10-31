#!/bin/bash

echo "🚀 INICIANDO URVIC API SERVER"

# Verificar que el binario existe
if [ ! -f "urvic-backend" ]; then
    echo "❌ Error: No se encontró urvic-backend"
    echo "Ejecuta primero: ./install.sh"
    exit 1
fi

# Verificar que el archivo .env existe
if [ ! -f ".env" ]; then
    echo "❌ Error: No se encontró el archivo .env"
    exit 1
fi

# Cargar variables de entorno
echo "⚙️  Cargando configuración..."
export $(cat .env | grep -v '^#' | xargs)

# Verificar que las variables importantes estén configuradas
if [ -z "$APP_PORT" ]; then
    echo "⚠️  APP_PORT no configurado, usando puerto 20090"
    export APP_PORT=20090
fi

if [ -z "$DATABASE_URL" ]; then
    echo "❌ Error: DATABASE_URL no configurado en .env"
    exit 1
fi

# Verificar si ya hay un proceso corriendo
if pgrep -f "urvic-backend" > /dev/null; then
    echo "⚠️  Ya hay una instancia de urvic-backend corriendo"
    echo "Para detenerla ejecuta: pkill urvic-backend"
    read -p "¿Quieres detener la instancia anterior y continuar? [y/N]: " confirm
    if [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]]; then
        pkill urvic-backend
        sleep 2
    else
        echo "Operación cancelada"
        exit 1
    fi
fi

# Iniciar el servidor en background
echo "🔄 Iniciando servidor en puerto $APP_PORT..."
nohup ./urvic-backend > urvic.log 2>&1 &
SERVER_PID=$!

# Dar tiempo para que inicie
sleep 3

# Verificar que se inició correctamente
if ps -p $SERVER_PID > /dev/null; then
    echo "✅ Servidor iniciado exitosamente!"
    echo ""
    echo "📊 Información del servidor:"
    echo "   PID: $SERVER_PID"
    echo "   Puerto: $APP_PORT"
    echo "   Log: urvic.log"
    echo ""
    echo "🌐 Endpoints disponibles:"
    echo "   Status Colors: http://localhost:$APP_PORT/api/status-colors"
    echo "   Swagger UI: http://localhost:$APP_PORT/swagger-ui/"
    echo ""
    echo "📝 Comandos útiles:"
    echo "   Ver logs: tail -f urvic.log"
    echo "   Detener servidor: pkill urvic-backend"
    echo "   Ver procesos: ps aux | grep urvic-backend"
else
    echo "❌ Error al iniciar el servidor"
    echo "Revisa el log para más detalles:"
    echo "   tail -20 urvic.log"
    exit 1
fi
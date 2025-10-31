#!/bin/bash

echo "🛑 DETENIENDO URVIC API SERVER"

# Buscar procesos de urvic-backend
PIDS=$(pgrep -f "urvic-backend")

if [ -z "$PIDS" ]; then
    echo "ℹ️  No se encontraron procesos de urvic-backend ejecutándose"
    exit 0
fi

echo "📋 Procesos encontrados:"
ps aux | grep urvic-backend | grep -v grep

echo ""
echo "🔄 Deteniendo procesos..."

# Intentar detener gracefully primero
for PID in $PIDS; do
    echo "Enviando SIGTERM a proceso $PID..."
    kill $PID
done

# Esperar 5 segundos
sleep 5

# Verificar si aún están corriendo
REMAINING=$(pgrep -f "urvic-backend")

if [ ! -z "$REMAINING" ]; then
    echo "⚠️  Algunos procesos no se detuvieron, forzando cierre..."
    for PID in $REMAINING; do
        echo "Enviando SIGKILL a proceso $PID..."
        kill -9 $PID
    done
    sleep 2
fi

# Verificación final
if pgrep -f "urvic-backend" > /dev/null; then
    echo "❌ Error: Algunos procesos aún están corriendo"
    ps aux | grep urvic-backend | grep -v grep
    exit 1
else
    echo "✅ Todos los procesos de urvic-backend han sido detenidos"
    
    # Mostrar información del log si existe
    if [ -f "urvic.log" ]; then
        echo ""
        echo "📝 Últimas líneas del log:"
        tail -10 urvic.log
    fi
fi
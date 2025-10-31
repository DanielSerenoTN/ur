#!/bin/bash

echo "=== URVIC API - Script de Despliegue ==="
echo ""

# Función para limpiar contenedores existentes
cleanup() {
    echo "Limpiando contenedores existentes..."
    docker stop urvic-backend-manual urvic-postgres-manual 2>/dev/null || true
    docker rm urvic-backend-manual urvic-postgres-manual 2>/dev/null || true
    echo "✅ Limpieza completada"
    echo ""
}

# Función para verificar puertos
check_ports() {
    echo "Verificando puertos..."
    if netstat -tlnp | grep :5435 > /dev/null; then
        echo "Puerto 5435 ya está en uso"
        read -p "¿Continuar de todas formas? (y/n): " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
    
    if netstat -tlnp | grep :20090 > /dev/null; then
        echo "Puerto 20090 ya está en uso"
        read -p "¿Continuar de todas formas? (y/n): " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
    echo "Puertos disponibles"
    echo ""
}

# Función para construir la imagen
build_image() {
    echo "🔨 Construyendo imagen Docker..."
    docker build -t api-urvic_app . || {
        echo "Error construyendo la imagen"
        exit 1
    }
    echo "✅ Imagen construida exitosamente"
    echo ""
}

# Función para ejecutar PostgreSQL
start_postgres() {
    echo "Iniciando PostgreSQL..."
    docker run -d \
        --name urvic-postgres-manual \
        -e POSTGRES_USER=urvic_prod_user \
        -e POSTGRES_PASSWORD=SuperSecurePassword2025 \
        -e POSTGRES_DB=urvic_production \
        -p 5435:5432 \
        postgres:15-alpine || {
        echo "Error iniciando PostgreSQL"
        exit 1
    }
    
    echo "⏳ Esperando que PostgreSQL esté listo..."
    sleep 10
    echo "✅ PostgreSQL iniciado en puerto 5435"
    echo ""
}

# Función para ejecutar la aplicación
start_app() {
    echo "🚀 Iniciando aplicación URVIC..."
    docker run -d \
        --name urvic-backend-manual \
        --link urvic-postgres-manual:postgres \
        -p 20090:20090 \
        --entrypoint ./urvic-backend \
        api-urvic_app || {
        echo "Error iniciando la aplicación"
        exit 1
    }
    echo "Aplicación iniciada en puerto 20090"
    echo ""
}

# Función para verificar el estado
check_status() {
    echo "Estado de los contenedores:"
    docker ps | grep urvic || echo "No hay contenedores urvic ejecutándose"
    echo ""
    
    echo "Verificando conectividad..."
    sleep 5
    if curl -s http://localhost:20090/api/status-colors > /dev/null; then
        echo "API respondiendo correctamente"
        echo "Despliegue exitoso!"
        echo ""
        echo "URLs disponibles:"
        echo "   • API: http://localhost:20090/api/"
        echo "   • Status Colors: http://localhost:20090/api/status-colors"
        echo "   • Swagger UI: http://localhost:20090/swagger-ui/"
    else
        echo "API no responde aún, verificando logs..."
        echo ""
        echo "Logs de la aplicación:"
        docker logs urvic-backend-manual
    fi
}

# Función principal
main() {
    echo "Iniciando despliegue de URVIC API..."
    echo ""
    
    # Verificar que existe .env
    if [ ! -f ".env" ]; then
        echo "Error: archivo .env no encontrado"
        exit 1
    fi
    
    cleanup
    check_ports
    build_image
    start_postgres
    start_app
    check_status
}

# Función para mostrar ayuda
show_help() {
    echo "Uso: $0 [opción]"
    echo ""
    echo "Opciones:"
    echo "  start    - Iniciar todos los servicios (por defecto)"
    echo "  stop     - Parar todos los servicios"
    echo "  restart  - Reiniciar todos los servicios"
    echo "  logs     - Mostrar logs de la aplicación"
    echo "  status   - Mostrar estado de los contenedores"
    echo "  help     - Mostrar esta ayuda"
}

# Función para parar servicios
stop_services() {
    echo "Parando servicios..."
    docker stop urvic-backend-manual urvic-postgres-manual 2>/dev/null || true
    echo "Servicios parados"
}

# Función para mostrar logs
show_logs() {
    echo "Logs de la aplicación:"
    docker logs -f urvic-backend-manual
}

# Función para mostrar estado
show_status() {
    echo "Estado de los contenedores:"
    docker ps | grep urvic || echo "No hay contenedores urvic ejecutándose"
    echo ""
    echo "Probando conectividad:"
    curl -s http://localhost:20090/api/status-colors || echo "API no responde"
}

# Manejo de argumentos
case "${1:-start}" in
    start)
        main
        ;;
    stop)
        stop_services
        ;;
    restart)
        stop_services
        sleep 2
        main
        ;;
    logs)
        show_logs
        ;;
    status)
        show_status
        ;;
    help)
        show_help
        ;;
    *)
        echo "Opción no válida: $1"
        show_help
        exit 1
        ;;
esac
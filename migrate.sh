#!/bin/bash

echo "=== URVIC API - Ejecutor de Migraciones ==="
echo ""

# Función para verificar que PostgreSQL esté corriendo
check_postgres() {
    echo "🔍 Verificando PostgreSQL..."
    if ! docker ps | grep urvic-postgres-manual > /dev/null; then
        echo "PostgreSQL no está ejecutándose"
        echo "Ejecuta primero: ./run.sh start"
        exit 1
    fi
    echo "PostgreSQL está ejecutándose"
    echo ""
}

# Función para esperar que PostgreSQL esté listo
wait_postgres() {
    echo "Esperando que PostgreSQL esté listo..."
    until docker exec urvic-postgres-manual pg_isready -U urvic_prod_user -d urvic_production > /dev/null 2>&1; do
        echo "Esperando PostgreSQL..."
        sleep 2
    done
    echo "PostgreSQL está listo"
    echo ""
}

# Función para ejecutar migraciones con diesel
run_migrations() {
    echo "Ejecutando migraciones con Diesel..."
    
    # Crear un contenedor temporal para ejecutar diesel
    docker run --rm \
        --link urvic-postgres-manual:postgres \
        -v $(pwd):/app \
        -w /app \
        -e DATABASE_URL=postgresql://urvic_prod_user:SuperSecurePassword2025@postgres:5432/urvic_production \
        rust:latest \
        sh -c "
            echo 'Instalando diesel_cli...'
            cargo install diesel_cli --no-default-features --features postgres --quiet
            echo 'Ejecutando migraciones...'
            diesel migration run
        " || {
        echo "Error ejecutando migraciones con Diesel"
        exit 1
    }
    
    echo "Migraciones ejecutadas exitosamente"
    echo ""
}

# Función para ejecutar migraciones manualmente (sin diesel)
run_migrations_manual() {
    echo "Ejecutando migraciones manualmente..."
    
    # Ejecutar cada archivo SQL de migración
    for migration_dir in migrations/*; do
        if [ -d "$migration_dir" ]; then
            migration_name=$(basename "$migration_dir")
            up_file="$migration_dir/up.sql"
            
            if [ -f "$up_file" ]; then
                echo "Ejecutando migración: $migration_name"
                docker exec -i urvic-postgres-manual psql -U urvic_prod_user -d urvic_production < "$up_file" || {
                    echo "Error en migración: $migration_name"
                    exit 1
                }
                echo "Migración completada: $migration_name"
            fi
        fi
    done
    
    echo "Todas las migraciones ejecutadas"
    echo ""
}

# Función para verificar tablas creadas
verify_tables() {
    echo "🔍 Verificando tablas creadas..."
    docker exec urvic-postgres-manual psql -U urvic_prod_user -d urvic_production -c "\dt" || {
        echo "Error verificando tablas"
        exit 1
    }
    echo ""
}

# Función para verificar datos de status_colors
verify_data() {
    echo "Verificando datos de status_colors..."
    docker exec urvic-postgres-manual psql -U urvic_prod_user -d urvic_production -c "SELECT COUNT(*) as total_colors FROM status_colors;" || {
        echo "Error verificando datos"
        exit 1
    }
    echo ""
}

# Función para mostrar ayuda
show_help() {
    echo "Uso: $0 [opción]"
    echo ""
    echo "Opciones:"
    echo "  diesel   - Ejecutar migraciones con Diesel CLI (recomendado)"
    echo "  manual   - Ejecutar migraciones manualmente con psql"
    echo "  verify   - Solo verificar tablas existentes"
    echo "  reset    - Eliminar y recrear la base de datos"
    echo "  help     - Mostrar esta ayuda"
    echo ""
    echo "Ejemplo:"
    echo "  ./migrate.sh diesel"
}

# Función para resetear la base de datos
reset_database() {
    echo "Reseteando base de datos..."
    read -p "¿Estás seguro? Esto eliminará todos los datos (y/n): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operación cancelada"
        exit 1
    fi
    
    # Recrear la base de datos
    docker exec urvic-postgres-manual psql -U urvic_prod_user -d postgres -c "DROP DATABASE IF EXISTS urvic_production;"
    docker exec urvic-postgres-manual psql -U urvic_prod_user -d postgres -c "CREATE DATABASE urvic_production;"
    
    echo "Base de datos reseteada"
    echo "Ejecutando migraciones..."
    run_migrations_manual
}

# Función principal
main() {
    check_postgres
    wait_postgres
    
    case "${1:-diesel}" in
        diesel)
            run_migrations
            verify_tables
            verify_data
            ;;
        manual)
            run_migrations_manual
            verify_tables
            verify_data
            ;;
        verify)
            verify_tables
            verify_data
            ;;
        reset)
            reset_database
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
    
    echo "Proceso completado exitosamente!"
    echo ""
    echo "Próximos pasos:"
    echo "   • Reinicia la aplicación: ./run.sh restart"
    echo "   • Verifica la API: curl http://localhost:3001/api/status-colors"
}

# Ejecutar función principal
main "$@"
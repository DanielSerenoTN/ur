-- Script de configuración de base de datos para URVIC API
-- Crear tabla status_colors
CREATE TABLE IF NOT EXISTS status_colors (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    status VARCHAR(100) NOT NULL UNIQUE,
    hexadecimal VARCHAR(7) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Insertar datos iniciales de status_colors
INSERT IGNORE INTO status_colors (name, status, hexadecimal) VALUES
    ('Disponible', 'Disponible', '#25B52A'),
    ('Apartada', 'Apartada', '#F5C72F'),
    ('Venta', 'Venta', '#C4F0B3'),
    ('Titulación', 'Titulación', '#ADD9FF'),
    ('Escrituración', 'Escrituración', '#168AEF'),
    ('Entrega', 'Entrega', '#5D4FFB'),
    ('Cancelado Apartado', 'Cancelado Apartado', '#C9BA46'),
    ('Casa Muestra', 'Casa Muestra', '#AF38FA'),
    ('Oficina', 'Oficina', '#8A37BE'),
    ('No disponible', 'No disponible', '#FD8989');

-- Crear tabla zoho_code
CREATE TABLE IF NOT EXISTS zoho_code (
    id INT AUTO_INCREMENT PRIMARY KEY,
    code VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired_at TIMESTAMP NULL
);

-- Índice para búsquedas eficientes por código activo
CREATE INDEX IF NOT EXISTS idx_zoho_code_active ON zoho_code (code, expired_at);

-- Crear tabla products
CREATE TABLE IF NOT EXISTS products (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    product_name VARCHAR(255),
    estatus_venta VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Crear tabla maps_svg
CREATE TABLE IF NOT EXISTS maps_svg (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    prefix VARCHAR(50) NOT NULL,
    content LONGTEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Insertar código inicial de Zoho si no existe
-- INSERT IGNORE INTO zoho_code (code) VALUES ('PORTAL_ACCESS_2025');

-- Mostrar tablas creadas
SELECT 'Tablas creadas exitosamente:' AS resultado;
SHOW TABLES;
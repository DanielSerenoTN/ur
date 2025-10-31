CREATE TABLE status_colors (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    status VARCHAR(100) NOT NULL UNIQUE,
    hexadecimal VARCHAR(7) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Insertar datos iniciales
INSERT INTO status_colors (name, status, hexadecimal) VALUES
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

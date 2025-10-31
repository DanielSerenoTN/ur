CREATE TABLE zoho_code (
    id INT AUTO_INCREMENT PRIMARY KEY,
    code VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired_at TIMESTAMP NULL
);

-- Índice para búsquedas eficientes por código activo
CREATE INDEX idx_zoho_code_active ON zoho_code (code, expired_at);

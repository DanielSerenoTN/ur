-- Función para actualizar updated_at automáticamente
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger para la tabla maps_svg
CREATE TRIGGER update_maps_svg_updated_at 
    BEFORE UPDATE ON maps_svg 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
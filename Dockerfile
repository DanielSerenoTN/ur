# Build Stage
FROM centos:7 AS builder

# Actualizar repositorios a vault.centos.org (CentOS 7 es EOL)
RUN sed -i 's/mirrorlist/#mirrorlist/g' /etc/yum.repos.d/CentOS-*.repo && \
    sed -i 's|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g' /etc/yum.repos.d/CentOS-*.repo

# Instalar dependencias del sistema
RUN yum update -y && \
    yum install -y \
    gcc \
    gcc-c++ \
    make \
    openssl-devel \
    perl \
    curl \
    ca-certificates \
    && yum clean all

# Instalar Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Configurar directorio de trabajo
WORKDIR /app

# Copiar archivos de configuración primero (para cache de dependencias)
COPY Cargo.toml Cargo.lock ./

# Crear un proyecto dummy para compilar dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar el código fuente real
COPY . .

# Compilar la aplicación
RUN cargo build --release

# Runtime Stage
FROM centos:7

# Actualizar repositorios a vault.centos.org
RUN sed -i 's/mirrorlist/#mirrorlist/g' /etc/yum.repos.d/CentOS-*.repo && \
    sed -i 's|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g' /etc/yum.repos.d/CentOS-*.repo

# Instalar solo las dependencias necesarias para runtime
RUN yum update -y && \
    yum install -y \
    openssl \
    ca-certificates \
    && yum clean all

# Crear usuario no-root
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copiar el binario compilado desde el builder
COPY --from=builder /app/target/release/urvic-backend .

# Cambiar al usuario no-root
USER appuser

# Exponer el puerto (ajusta según tu aplicación)
EXPOSE 8080

# Comando para ejecutar la aplicación
CMD ["./urvic-backend"]
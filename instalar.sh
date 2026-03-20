#!/bin/bash
set -e

# Colores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Verificar root
if [[ $EUID -ne 0 ]]; then
   print_error "Este script debe ejecutarse como root (sudo)"
   exit 1
fi

print_status "Instalando dependencias necesarias..."
apt update
apt install -y curl wget systemd ufw nano openssl

# Crear usuario de sistema sigepol si no existe
print_status "Verificando usuario de sistema sigepol..."
if ! id "sigepol" &>/dev/null; then
    useradd --system --no-create-home --shell /bin/false sigepol
fi

# Crear estructura de directorios
print_status "Creando estructura de directorios..."
mkdir -p /opt/sige-api/{logs,config}

# Mover archivos subidos manualmente
print_status "Moviendo binario y archivos de configuración..."
mv /home/sigepol/sige-api /opt/sige-api/sige-api
mv /home/sigepol/.env /opt/sige-api/config/.env
mv /home/sigepol/settings.json /opt/sige-api/settings.json

# Configurar permisos
print_status "Configurando permisos y propietarios..."
chown -R sigepol:sigepol /opt/sige-api
chmod +x /opt/sige-api/sige-api
chmod 600 /opt/sige-api/config/.env
chmod 600 /opt/sige-api/settings.json

# Crear servicio systemd robusto
print_status "Creando servicio systemd..."
cat > /etc/systemd/system/sige-api.service << 'EOF'
[Unit]
Description=SIGE API Service
After=network.target

[Service]
Type=exec
User=sigepol
Group=sigepol
WorkingDirectory=/opt/sige-api
EnvironmentFile=/opt/sige-api/config/.env
ExecStart=/opt/sige-api/sige-api start
Restart=on-failure
RestartSec=5
StandardOutput=append:/opt/sige-api/logs/sige-api.log
StandardError=append:/opt/sige-api/logs/sige-api-error.log

[Install]
WantedBy=multi-user.target
EOF

# Habilitar y arrancar el servicio
print_status "Habilitando y arrancando el servicio..."
systemctl daemon-reload
systemctl enable sige-api
systemctl restart sige-api

# Configurar firewall
print_status "Configurando firewall..."
ufw allow 22/tcp
ufw allow 5432/tcp
ufw allow 8443/tcp
ufw --force enable

# Crear script de gestión
print_status "Creando script de gestión /usr/local/bin/sige..."
cat > /usr/local/bin/sige << 'EOF'
#!/bin/bash
case "$1" in
    start)   systemctl start sige-api ;;
    stop)    systemctl stop sige-api ;;
    restart) systemctl restart sige-api ;;
    status)  systemctl status sige-api ;;
    logs)    tail -f /opt/sige-api/logs/sige-api.log ;;
    migrate) cd /opt/sige-api && sudo -u sigepol ./sige-api migrate ;;
    version) /opt/sige-api/sige-api --version ;;
    help)    /opt/sige-api/sige-api --help ;;
    *) echo "Uso: $0 {start|stop|restart|status|logs|migrate|version|help}" ;;
esac
EOF
chmod +x /usr/local/bin/sige

print_status "==============================================="
print_status "✅ INSTALACIÓN COMPLETADA"
print_status "==============================================="
echo
print_status "🔧 COMANDOS DISPONIBLES:"
echo "  sige start      - Iniciar servicio"
echo "  sige stop       - Detener servicio"
echo "  sige restart    - Reiniciar servicio"
echo "  sige status     - Ver estado"
echo "  sige logs       - Ver logs"
echo "  sige migrate    - Ejecutar migraciones"
echo "  sige version    - Ver versión"
echo "  sige help       - Ver ayuda"
echo
print_status "🚀 PRÓXIMOS PASOS:"
echo "1. sige migrate     # Ejecutar migraciones"
echo "2. sige start       # Iniciar servicio"
echo "3. sige status      # Verificar estado"
echo
print_status "🌐 ACCESO: http://<TU-IP>:8443"
print_status "📂 LOGS: /opt/sige-api/logs/"
print_status "⚙️  CONFIG: /opt/sige-api/config/.env"
print_status "💾 EJECUTABLE: /opt/sige-api/sige-api" 
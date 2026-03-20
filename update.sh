#!/bin/bash
set -e

# ================== CONFIGURACIÓN ==================
SERVICE_NAME="sige-api.service"
BINARY_PATH="/opt/sige-api/sige-api"
NEW_BINARY="/home/sigepol/sige-api"

# ================== ACTUALIZACIÓN ==================
echo "[1/5] Verificando binario nuevo..."
if [ ! -f "$NEW_BINARY" ]; then
    echo "ERROR: No se encontró el binario 'sige-api' en /home/sigepol" >&2
    exit 1
fi

echo "[2/5] Deteniendo el servicio $SERVICE_NAME..."
sudo systemctl stop $SERVICE_NAME

echo "[3/5] Reemplazando binario en $BINARY_PATH..."
sudo mv "$NEW_BINARY" "$BINARY_PATH"
sudo chown sigepol:sigepol "$BINARY_PATH"
sudo chmod +x "$BINARY_PATH"

echo "[4/5] Iniciando el servicio $SERVICE_NAME..."
sudo systemctl start $SERVICE_NAME

echo "[5/5] Estado del servicio:"
systemctl status $SERVICE_NAME --no-pager

echo -e "\nActualización completada." 
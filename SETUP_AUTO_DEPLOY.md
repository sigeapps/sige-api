# 🚀 Configuración Auto-Deploy SIGE-API

Sistema simplificado para actualizar automáticamente ambos servidores con cada `git push`.

---

## 📋 Resumen
Cada vez que haces `git push`, GitHub Actions compila y despliega automáticamente en:
- **Servidor Linux**: `cpnbdaet.org.ve` (192.168.11.100:8443)
- **Servidor Windows**: `app.tecnoelectronics.com.ve:8443`

---

## ⚙️ Paso 1: Configurar Servidor Windows

En el servidor Windows (app.tecnoelectronics.com.ve), ejecutar como **Administrador**:

```powershell
# 1. Configurar OpenSSH
.\setup_windows_ssh.ps1

# 2. Configurar servicio SIGE-API
.\setup_windows_service.ps1
```

Esto creará:
- ✅ Usuario `tecno` con contraseña `tecno2025`
- ✅ SSH habilitado en puerto 22
- ✅ Servicio `sige-api` configurado
- ✅ Directorio `D:\SIGE\sige-api\`
- ✅ Directorio `D:\SIGE\sige-api\target\release\`

---

## 🔧 Paso 2: Configurar GitHub Actions

**No necesitas secrets** - Las credenciales están directamente en el workflow:

### Servidor Linux:
- Host: `cpnbdaet.org.ve`
- Usuario: `sigepol`
- Contraseña: `daet25`

### Servidor Windows:
- Host: `app.tecnoelectronics.com.ve`
- Usuario: `tecno`
- Contraseña: `tecno2025`

---

## 🎯 ¡Listo! ¿Cómo funciona?

### 1. **Hacer cambios**
```bash
git add .
git commit -m "Nueva funcionalidad"
git push origin master
```

### 2. **GitHub Actions automáticamente:**
- ✅ Compila para Linux y Windows
- ✅ Crea configuraciones específicas para cada servidor
- ✅ Despliega en servidor Linux (cpnbdaet.org.ve)
- ✅ Despliega en servidor Windows (app.tecnoelectronics.com.ve)
- ✅ Reinicia servicios automáticamente

### 3. **Configuraciones automáticas:**

**Linux** (`/home/sigepol/settings.json`):
```json
{
  "database": {
    "url": "postgresql://postgres:D@et2O25*@192.168.11.100:5432/sigepol"
  },
  "address": {
    "host": "192.168.11.100",
    "port": 8443
  }
}
```

**Windows** (`D:\SIGE\sige-api\settings.json`):
```json
{
  "database": {
    "url": "postgresql://postgres:D@et2025@192.168.1.101:5432/sige"
  },
  "address": {
    "host": "192.168.1.101",
    "port": 8443
  }
}
```

---

## 📊 Verificar Despliegue

### Linux:
```bash
ssh sigepol@cpnbdaet.org.ve
sudo systemctl status sige-api.service
```

### Windows:
```powershell
ssh tecno@app.tecnoelectronics.com.ve
Get-Service -Name sige-api
```

---

## 🚨 Comandos de Emergencia

### Reiniciar servicios manualmente:

**Linux:**
```bash
ssh sigepol@cpnbdaet.org.ve
echo "daet25" | sudo -S systemctl restart sige-api.service
```

**Windows:**
```powershell
ssh tecno@app.tecnoelectronics.com.ve
Restart-Service -Name sige-api
```

### Ver logs de GitHub Actions:
1. Ve a tu repositorio en GitHub
2. Click en **"Actions"**
3. Selecciona el último workflow
4. Revisa los logs si hay errores

---

## ✅ Estructura Final

```
📂 Repositorio
├── 🔄 .github/workflows/auto-deploy.yml    # Workflow de GitHub Actions
├── 🪟 setup_windows_ssh.ps1                # Configurar SSH en Windows
├── 🪟 setup_windows_service.ps1            # Configurar servicio Windows
└── 📖 SETUP_AUTO_DEPLOY.md                 # Esta guía

📂 Servidor Linux (cpnbdaet.org.ve)
├── 🔧 /usr/local/bin/sige-api              # Binario actualizado automáticamente
└── ⚙️ /home/sigepol/settings.json          # Configuración Linux

📂 Servidor Windows (app.tecnoelectronics.com.ve)
├── 📁 D:\SIGE\sige-api\                     # Directorio del proyecto
├── 🔧 D:\SIGE\sige-api\target\release\sige-api.exe  # Binario actualizado automáticamente
└── ⚙️ D:\SIGE\sige-api\settings.json       # Configuración Windows
```

---

## 🎉 ¡Sistema Funcionando!

Ahora cada `git push` actualiza automáticamente ambos servidores:
- ⚡ **Compilación** para Linux y Windows
- 🚀 **Despliegue paralelo** en ambos servidores  
- 🔄 **Reinicio automático** de servicios
- 📊 **Logs detallados** en GitHub Actions

**¡No más actualizaciones manuales!** 🎯 
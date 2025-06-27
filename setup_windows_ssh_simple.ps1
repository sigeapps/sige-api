# Configuracion de OpenSSH para servidor Windows
# Ejecutar como Administrador

Write-Host "Configurando OpenSSH en Windows Server..." -ForegroundColor Green

# Instalar OpenSSH Server
Write-Host "Instalando OpenSSH Server..." -ForegroundColor Cyan
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0

# Iniciar y habilitar servicio SSH
Write-Host "Iniciando servicio SSH..." -ForegroundColor Cyan
Start-Service sshd
Set-Service -Name sshd -StartupType 'Automatic'

# Configurar firewall
Write-Host "Configurando firewall..." -ForegroundColor Cyan
New-NetFirewallRule -Name sshd -DisplayName 'OpenSSH SSH Server' -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort 22 -ErrorAction SilentlyContinue

# Crear usuario tecno si no existe
Write-Host "Configurando usuario tecno..." -ForegroundColor Cyan
try {
    Get-LocalUser -Name "tecno" -ErrorAction Stop
    Write-Host "Usuario tecno ya existe" -ForegroundColor Yellow
} catch {
    $Password = ConvertTo-SecureString "tecno2025" -AsPlainText -Force
    New-LocalUser -Name "tecno" -Password $Password -Description "Usuario para deploy automatico"
    Add-LocalGroupMember -Group "Administrators" -Member "tecno"
    Write-Host "Usuario tecno creado y agregado al grupo Administrators" -ForegroundColor Green
}

# Configurar permisos para el directorio SIGE
Write-Host "Configurando directorios..." -ForegroundColor Cyan
$SigeDir = "D:\SIGE\sige-api"
if (!(Test-Path $SigeDir)) {
    New-Item -ItemType Directory -Path $SigeDir -Force
}

# Dar permisos al usuario tecno
icacls "D:\SIGE\sige-api" /grant "tecno:(OI)(CI)F" /T

# Configurar SSH para permitir conexiones
Write-Host "Configurando SSH..." -ForegroundColor Cyan
$sshd_config = "C:\ProgramData\ssh\sshd_config"
if (Test-Path $sshd_config) {
    # Hacer backup
    Copy-Item $sshd_config "$sshd_config.backup"
    
    # Configuraciones basicas
    (Get-Content $sshd_config) -replace "#PubkeyAuthentication yes", "PubkeyAuthentication yes" | Set-Content $sshd_config
    (Get-Content $sshd_config) -replace "#PasswordAuthentication yes", "PasswordAuthentication yes" | Set-Content $sshd_config
}

# Reiniciar servicio SSH
Write-Host "Reiniciando servicio SSH..." -ForegroundColor Cyan
Restart-Service sshd

# Verificar estado
Write-Host "Verificando configuracion..." -ForegroundColor Cyan
$sshService = Get-Service sshd
Write-Host "Estado SSH: $($sshService.Status)" -ForegroundColor $(if ($sshService.Status -eq "Running") { "Green" } else { "Red" })

# Configuracion SSH completada
Write-Host "Configuracion SSH completada" -ForegroundColor Cyan
Write-Host "Nota: Probar SSH manualmente con: ssh tecno@localhost" -ForegroundColor Yellow

Write-Host ""
Write-Host "Configuracion completada!" -ForegroundColor Green
Write-Host "=============================" -ForegroundColor Green
Write-Host "Informacion de conexion:" -ForegroundColor Cyan
Write-Host "   - Host: app.tecnoelectronics.com.ve"
Write-Host "   - Usuario: tecno"
Write-Host "   - Contraseña: tecno2025"
Write-Host "   - Puerto: 22"
Write-Host ""
Write-Host "Probar conexion desde otra maquina:" -ForegroundColor Cyan
Write-Host "   ssh tecno@app.tecnoelectronics.com.ve"
Write-Host ""
Write-Host "Directorio de instalacion: D:\SIGE\sige-api" -ForegroundColor Cyan 
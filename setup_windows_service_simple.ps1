# Configuracion del servicio SIGE-API para Windows
# Ejecutar como Administrador

Write-Host "Configurando servicio SIGE-API en Windows..." -ForegroundColor Green

$ServiceName = "sige-api"
$ServiceDisplayName = "SIGE-API Service"
$ServiceDescription = "Servicio de la API SIGE para gestion policial"
$BinaryPath = "D:\SIGE\sige-api\target\release\sige-api.exe"
$WorkingDirectory = "D:\SIGE\sige-api"

# Crear directorio si no existe
if (!(Test-Path $WorkingDirectory)) {
    New-Item -ItemType Directory -Path $WorkingDirectory -Force
    Write-Host "Directorio creado: $WorkingDirectory" -ForegroundColor Green
}

# Crear directorio target/release si no existe
$TargetReleaseDir = "D:\SIGE\sige-api\target\release"
if (!(Test-Path $TargetReleaseDir)) {
    New-Item -ItemType Directory -Path $TargetReleaseDir -Force
    Write-Host "Directorio target/release creado: $TargetReleaseDir" -ForegroundColor Green
}

# Detener y eliminar servicio existente si existe
$existingService = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($existingService) {
    Write-Host "Deteniendo servicio existente..." -ForegroundColor Yellow
    Stop-Service -Name $ServiceName -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 2
    
    Write-Host "Eliminando servicio existente..." -ForegroundColor Yellow
    sc.exe delete $ServiceName
    Start-Sleep -Seconds 2
}

# Crear el servicio
Write-Host "Creando servicio..." -ForegroundColor Cyan
$CreateServiceCommand = "sc.exe create `"$ServiceName`" binPath= `"$BinaryPath`" DisplayName= `"$ServiceDisplayName`" start= auto"
Invoke-Expression $CreateServiceCommand

# Configurar descripcion
sc.exe description $ServiceName $ServiceDescription

# Configurar recuperacion del servicio (reiniciar automaticamente en caso de fallo)
Write-Host "Configurando recuperacion automatica..." -ForegroundColor Cyan
sc.exe failure $ServiceName reset= 86400 actions= restart/30000/restart/30000/restart/30000

# Configurar el servicio para que se ejecute en el directorio correcto
Write-Host "Configurando directorio de trabajo..." -ForegroundColor Cyan

# Crear archivo de configuracion de ejemplo
$ConfigContent = @'
{
  "database": {
    "url": "postgresql://postgres:D@et2O25*@app.tecnoelectronics.com.ve:5432/sigepol"
  },
  "address": {
    "host": "app.tecnoelectronics.com.ve",
    "port": 8443
  }
}
'@

$ConfigPath = "$WorkingDirectory\settings.json"
$ConfigContent | Out-File -FilePath $ConfigPath -Encoding UTF8
Write-Host "Configuracion creada: $ConfigPath" -ForegroundColor Green

# Intentar iniciar el servicio (solo si existe el binario)
if (Test-Path $BinaryPath) {
    Write-Host "Iniciando servicio..." -ForegroundColor Cyan
    try {
        Start-Service -Name $ServiceName
        Write-Host "Servicio iniciado exitosamente" -ForegroundColor Green
    } catch {
        Write-Host "Error iniciando servicio: $($_.Exception.Message)" -ForegroundColor Yellow
        Write-Host "El servicio se iniciara automaticamente cuando se despliegue el binario" -ForegroundColor Cyan
    }
} else {
    Write-Host "Binario no encontrado en: $BinaryPath" -ForegroundColor Yellow
    Write-Host "El servicio se iniciara automaticamente cuando se despliegue el binario" -ForegroundColor Cyan
}

# Mostrar estado del servicio
Write-Host "Estado del servicio:" -ForegroundColor Cyan
try {
    $service = Get-Service -Name $ServiceName
    Write-Host "   Nombre: $($service.Name)" -ForegroundColor White
    Write-Host "   Estado: $($service.Status)" -ForegroundColor $(if ($service.Status -eq "Running") { "Green" } else { "Yellow" })
    Write-Host "   Tipo de inicio: $($service.StartType)" -ForegroundColor White
} catch {
    Write-Host "   Error obteniendo estado del servicio" -ForegroundColor Red
}

Write-Host ""
Write-Host "Configuracion del servicio completada!" -ForegroundColor Green
Write-Host "======================================" -ForegroundColor Green
Write-Host "Informacion del servicio:" -ForegroundColor Cyan
Write-Host "   - Nombre: $ServiceName"
Write-Host "   - Binario: $BinaryPath"
Write-Host "   - Configuracion: $ConfigPath"
Write-Host "   - Directorio: $WorkingDirectory"
Write-Host ""
Write-Host "Comandos utiles:" -ForegroundColor Cyan
Write-Host "   - Ver estado: Get-Service -Name $ServiceName"
Write-Host "   - Iniciar: Start-Service -Name $ServiceName"
Write-Host "   - Detener: Stop-Service -Name $ServiceName"
Write-Host "   - Reiniciar: Restart-Service -Name $ServiceName" 
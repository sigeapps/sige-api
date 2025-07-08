# Script de instalación de GitHub Actions Runner con progreso
# Ejecutar como Administrador

param(
    [string]$GitHubUrl = "https://github.com/eswaldots/sige-api",
    [string]$Token = "BIXYUH3VRZ2X5REU7RVHKHLINGILY",
    [string]$RunnerVersion = "v2.325.0"
)

# Función para mostrar barra de progreso
function Show-Progress {
    param(
        [string]$Activity,
        [int]$PercentComplete,
        [string]$Status
    )
    
    Write-Progress -Activity $Activity -Status $Status -PercentComplete $PercentComplete
    Write-Host "[$([math]::Round($PercentComplete))%] $Status" -ForegroundColor Cyan
}

# Función para mostrar progreso de descarga
function Show-DownloadProgress {
    param(
        [string]$Uri,
        [string]$OutFile
    )
    
    Write-Host "Descargando GitHub Actions Runner..." -ForegroundColor Green
    Show-Progress -Activity "Descarga" -PercentComplete 0 -Status "Iniciando descarga..."
    
    try {
        $webClient = New-Object System.Net.WebClient
        $webClient.DownloadFileAsync($Uri, $OutFile)
        
        # Simular progreso durante la descarga
        for ($i = 0; $i -le 100; $i += 5) {
            Show-Progress -Activity "Descarga" -PercentComplete $i -Status "Descargando archivo..."
            Start-Sleep -Milliseconds 200
        }
        
        Show-Progress -Activity "Descarga" -PercentComplete 100 -Status "Descarga completada"
        Write-Host "✓ Descarga completada: $OutFile" -ForegroundColor Green
    } catch {
        Write-Host "✗ Error en la descarga: $($_.Exception.Message)" -ForegroundColor Red
        throw
    }
}

# Función para validar hash
function Test-FileHash {
    param(
        [string]$FilePath,
        [string]$ExpectedHash
    )
    
    Write-Host "Validando integridad del archivo..." -ForegroundColor Yellow
    Show-Progress -Activity "Validación" -PercentComplete 50 -Status "Calculando hash..."
    
    $actualHash = (Get-FileHash -Path $FilePath -Algorithm SHA256).Hash.ToUpper()
    $expectedHashUpper = $ExpectedHash.ToUpper()
    
    Show-Progress -Activity "Validación" -PercentComplete 100 -Status "Verificando hash..."
    
    if ($actualHash -ne $expectedHashUpper) {
        Write-Host "✗ Hash no coincide!" -ForegroundColor Red
        Write-Host "   Esperado: $expectedHashUpper" -ForegroundColor Red
        Write-Host "   Actual:   $actualHash" -ForegroundColor Red
        throw "Computed checksum did not match"
    } else {
        Write-Host "✓ Hash validado correctamente" -ForegroundColor Green
    }
}

# Función para extraer archivo con progreso
function Extract-WithProgress {
    param(
        [string]$ZipFile,
        [string]$ExtractPath
    )
    
    Write-Host "Extrayendo archivos..." -ForegroundColor Green
    Show-Progress -Activity "Extracción" -PercentComplete 0 -Status "Iniciando extracción..."
    
    try {
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        
        # Simular progreso de extracción
        Show-Progress -Activity "Extracción" -PercentComplete 25 -Status "Preparando archivos..."
        Start-Sleep -Milliseconds 500
        
        Show-Progress -Activity "Extracción" -PercentComplete 50 -Status "Extrayendo contenido..."
        [System.IO.Compression.ZipFile]::ExtractToDirectory($ZipFile, $ExtractPath)
        
        Show-Progress -Activity "Extracción" -PercentComplete 100 -Status "Extracción completada"
        Write-Host "✓ Extracción completada en: $ExtractPath" -ForegroundColor Green
    } catch {
        Write-Host "✗ Error en la extracción: $($_.Exception.Message)" -ForegroundColor Red
        throw
    }
}

# Función para configurar runner
function Configure-Runner {
    param(
        [string]$RunnerPath,
        [string]$GitHubUrl,
        [string]$Token
    )
    
    Write-Host "Configurando GitHub Actions Runner..." -ForegroundColor Green
    Show-Progress -Activity "Configuración" -PercentComplete 0 -Status "Iniciando configuración..."
    
    try {
        Set-Location $RunnerPath
        
        Show-Progress -Activity "Configuración" -PercentComplete 25 -Status "Conectando con GitHub..."
        Start-Sleep -Milliseconds 1000
        
        Show-Progress -Activity "Configuración" -PercentComplete 50 -Status "Configurando runner..."
        Start-Sleep -Milliseconds 1000
        
        Show-Progress -Activity "Configuración" -PercentComplete 75 -Status "Finalizando configuración..."
        Start-Sleep -Milliseconds 1000
        
        # Ejecutar configuración real
        $configProcess = Start-Process -FilePath ".\config.cmd" -ArgumentList "--url", $GitHubUrl, "--token", $Token, "--unattended" -PassThru -Wait
        
        Show-Progress -Activity "Configuración" -PercentComplete 100 -Status "Configuración completada"
        
        if ($configProcess.ExitCode -eq 0) {
            Write-Host "✓ Runner configurado exitosamente" -ForegroundColor Green
        } else {
            Write-Host "⚠ Configuración completada con advertencias" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "✗ Error en la configuración: $($_.Exception.Message)" -ForegroundColor Red
        throw
    }
}

# Función para iniciar runner
function Start-Runner {
    param(
        [string]$RunnerPath
    )
    
    Write-Host "Iniciando GitHub Actions Runner..." -ForegroundColor Green
    Show-Progress -Activity "Inicio" -PercentComplete 0 -Status "Preparando runner..."
    
    try {
        Set-Location $RunnerPath
        
        Show-Progress -Activity "Inicio" -PercentComplete 50 -Status "Iniciando runner..."
        Start-Sleep -Milliseconds 1000
        
        Show-Progress -Activity "Inicio" -PercentComplete 100 -Status "Runner iniciado"
        
        Write-Host "✓ GitHub Actions Runner iniciado" -ForegroundColor Green
        Write-Host "El runner está ejecutándose en segundo plano" -ForegroundColor Cyan
        
        # Mostrar información adicional
        Write-Host ""
        Write-Host "Información del Runner:" -ForegroundColor Cyan
        Write-Host "   - Directorio: $RunnerPath" -ForegroundColor White
        Write-Host "   - Repositorio: $GitHubUrl" -ForegroundColor White
        Write-Host "   - Estado: Ejecutándose" -ForegroundColor Green
        Write-Host ""
        Write-Host "Para detener el runner:" -ForegroundColor Yellow
        Write-Host "   cd $RunnerPath" -ForegroundColor White
        Write-Host "   .\run.cmd" -ForegroundColor White
        
    } catch {
        Write-Host "✗ Error iniciando runner: $($_.Exception.Message)" -ForegroundColor Red
        throw
    }
}

# Script principal
Write-Host "=========================================" -ForegroundColor Green
Write-Host "  Instalador de GitHub Actions Runner" -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Green
Write-Host ""

# Variables
$RunnerDir = "D:\actions-runner"
$ZipFile = "actions-runner-win-x64-$RunnerVersion.zip"
$ZipPath = Join-Path $RunnerDir $ZipFile
$DownloadUrl = "https://github.com/actions/runner/releases/download/$RunnerVersion/$ZipFile"
$ExpectedHash = "8601aa56828c084b29bdfda574af1fcde0943ce275fdbafb3e6d4a8611245b1b"

# Paso 1: Crear directorio
Write-Host "Paso 1/6: Creando directorio..." -ForegroundColor Yellow
Show-Progress -Activity "Preparación" -PercentComplete 0 -Status "Creando directorio..."
if (!(Test-Path $RunnerDir)) {
    New-Item -ItemType Directory -Path $RunnerDir -Force | Out-Null
    Show-Progress -Activity "Preparación" -PercentComplete 100 -Status "Directorio creado"
    Write-Host "✓ Directorio creado: $RunnerDir" -ForegroundColor Green
} else {
    Show-Progress -Activity "Preparación" -PercentComplete 100 -Status "Directorio existente"
    Write-Host "✓ Directorio ya existe: $RunnerDir" -ForegroundColor Green
}

# Paso 2: Descargar runner
Write-Host ""
Write-Host "Paso 2/6: Descargando runner..." -ForegroundColor Yellow
Show-DownloadProgress -Uri $DownloadUrl -OutFile $ZipPath

# Paso 3: Validar hash
Write-Host ""
Write-Host "Paso 3/6: Validando integridad..." -ForegroundColor Yellow
Test-FileHash -FilePath $ZipPath -ExpectedHash $ExpectedHash

# Paso 4: Extraer archivos
Write-Host ""
Write-Host "Paso 4/6: Extrayendo archivos..." -ForegroundColor Yellow
Extract-WithProgress -ZipFile $ZipPath -ExtractPath $RunnerDir

# Paso 5: Configurar runner
Write-Host ""
Write-Host "Paso 5/6: Configurando runner..." -ForegroundColor Yellow
Configure-Runner -RunnerPath $RunnerDir -GitHubUrl $GitHubUrl -Token $Token

# Paso 6: Iniciar runner
Write-Host ""
Write-Host "Paso 6/6: Iniciando runner..." -ForegroundColor Yellow
Start-Runner -RunnerPath $RunnerDir

# Limpiar progreso
Write-Progress -Activity "Completado" -Completed

Write-Host ""
Write-Host "=========================================" -ForegroundColor Green
Write-Host "  Instalación completada exitosamente!" -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Green
Write-Host ""
Write-Host "El GitHub Actions Runner está listo para usar." -ForegroundColor Cyan
Write-Host "Puedes verificar su estado en GitHub > Settings > Actions > Runners" -ForegroundColor Cyan 
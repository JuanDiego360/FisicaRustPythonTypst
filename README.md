# 🌌 Física Computacional con Rust, Python y Typst

> Repositorio modular de ejercicios y simulaciones de física resueltos de forma analítica, numérica y gráfica mediante una arquitectura políglota de máximo rendimiento.

[![GitHub Repository](https://img.shields.io/badge/GitHub-JuanDiego360%2FFisicaRustPythonTyspt-181717?style=flat-square&logo=github)](https://github.com/JuanDiego360/FisicaRustPythonTyspt)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.12%2B-blue?style=flat-square&logo=python)](https://www.python.org/)
[![Typst](https://img.shields.io/badge/Typst-0.15%2B-239dad?style=flat-square)](https://typst.app/)
[![Mise](https://img.shields.io/badge/Mise-Task_Runner-black?style=flat-square)](https://mise.jdx.dev/)

---

## 🎯 Filosofía del Repositorio

El propósito de este repositorio es albergar múltiples proyectos y ejercicios de física (electromagnetismo, mecánica clásica, ondas, termodinámica, gravitación, etc.), aplicando una división de responsabilidades donde **cada lenguaje y herramienta realiza la tarea en la que es insuperable**:

```
[datos.txt / Entradas]
          │
          ▼
   🦀 RUST (Cálculo Vectorial y Numérico) ──> [output/calculo.json]
          │
          ▼
   🐍 PYTHON (Graficación Científica 2D / 3D) ──> [output/grafica.png]
          │
          ▼
   📄 TYPST (Compilación del Informe Paso a Paso) ──> [output/reporte.pdf]
          ▲
          │
   🛠️ MISE (Orquestador que coordina todo con: mise run all)
```

1. **🦀 Rust (Motor Numérico Ultra Rápido):**
   - Se encarga del parseo de datos de entrada, álgebra vectorial, resolución de ecuaciones físicas con precisión en coma flotante `f64` y exportación estructurada a formato JSON en sub-milisegundos.
2. **🐍 Python + Matplotlib + Numpy (`uv`):**
   - Se encarga del trazado y visualización científica (campos vectoriales, líneas de flujo _streamplot_, distribuciones de cargas, proyecciones espaciales 3D). Gestionado de forma instantánea y reproducible mediante `uv` (sin necesidad de crear ni activar entornos virtuales manualmente).
3. **📄 Typst (Generación de Documentos Académicos):**
   - El estándar moderno para tipografía científica que compila en milisegundos. Lee directamente los datos generados y monta un informe en PDF con planteamiento, marco teórico, desglose algebraico paso a paso para cada elemento, ecuaciones matemáticas impecables, tablas comparativas y la gráfica final embebida.
4. **🛠️ Mise (Gestor de Herramientas y Tareas):**
   - Unifica el flujo completo en tareas declarativas (`mise.toml`). Modificas el archivo de datos del ejercicio, ejecutas `mise run all` y obtienes el documento final resuelto.

---

## 📦 Instalación de Herramientas

Para ejecutar cualquiera de los proyectos del repositorio, requieres tener instalados: **Rust**, **Python**, **uv**, **Typst** y **Mise**. A continuación se detalla la instalación paso a paso según tu distribución:

### 🔵 Arch Linux / CachyOS / Manjaro

En Arch Linux y derivados como CachyOS, todos los paquetes se encuentran en los repositorios oficiales y en AUR:

```bash
# 1. Actualizar el sistema e instalar herramientas base
sudo pacman -Syu base-devel git curl wget

# 2. Instalar Rust, Python, uv y Typst desde repositorios oficiales
sudo pacman -S rust cargo python uv typst

# 3. Instalar Mise
sudo pacman -S mise

# (Opcional) Si usas un helper de AUR como paru o yay:
# yay -S mise-bin
```

---

### 🟠 Debian / Ubuntu / Linux Mint

En Debian y derivadas basadas en `apt`, herramientas como `cargo` **no vienen preinstaladas** en el sistema ni se incluyen dentro del metapaquete `build-essential`. Debes instalarlas explícitamente:

```bash
# 1. Actualizar paquetes e instalar dependencias básicas del sistema
sudo apt update && sudo apt install -y curl wget git build-essential

# 2. Instalar Cargo y Rust:
# -------------------------------------------------------------------------
# • OPCIÓN A (Directo desde los repositorios de Debian con APT):
sudo apt install -y cargo rustc

# • OPCIÓN B (Oficial vía rustup, recomendada para tener la última versión estable):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
# Añadir al perfil para que esté disponible en cada nueva terminal:
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
# -------------------------------------------------------------------------

# 3. Instalar Python y uv (el gestor ultra rápido de paquetes de Python)
sudo apt install -y python3 python3-pip
curl -LsSf https://astral.sh/uv/install.sh | sh
source "$HOME/.local/bin/env" 2>/dev/null || export PATH="$HOME/.local/bin:$PATH"

# 4. Instalar Typst (binario precompilado oficial)
curl -fsSL https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz | tar -xJ
sudo mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/
rm -rf typst-x86_64-unknown-linux-musl

# 5. Instalar Mise (Task Runner)
sudo install -dm 755 /etc/apt/keyrings
wget -qO - https://mise.jdx.dev/gpg-key.pub | gpg --dearmor | sudo tee /etc/apt/keyrings/mise-archive-keyring.gpg 1> /dev/null
echo "deb [signed-by=/etc/apt/keyrings/mise-archive-keyring.gpg arch=amd64] https://mise.jdx.dev/deb stable main" | sudo tee /etc/apt/sources.list.d/mise.list
sudo apt update && sudo apt install -y mise
```

---

### 🔍 Verificación de Instalación

Ejecuta el siguiente comando para verificar que todas las herramientas responden correctamente:

```bash
mise --version && cargo --version && python3 --version && uv --version && typst --version
```

---

## 📁 Estructura del Repositorio

```
FisicaRustPythonTyspt/
├── README.md                          # Documentación general del repositorio
├── .gitignore                         # Exclusiones de Git (target, venv, output)
│
├── CalculoCampoElectrostaticos/       # ⚡ PROYECTO 1: Campo Electrostático 2D / 3D
│   ├── datos.txt                      # Entrada editable por el usuario
│   ├── mise.toml                      # Tareas automatizadas de Mise
│   ├── Cargo.toml                     # Dependencias de Rust (serde, serde_json)
│   ├── src/
│   │   └── main.rs                    # Motor de cálculo físico vectorial en Rust
│   ├── scripts/
│   │   └── plot_field.py              # Script de graficación científica en Python
│   ├── informe.typ                    # Plantilla de informe paso a paso en Typst
│   └── README.md                      # Documentación específica del proyecto
│
└── (Próximos proyectos...)            # 🚀 Cinemática, Termodinámica, Circuitos, etc.
```

---

## 🚀 Proyectos Disponibles

### 1. [Calculador de Campo Electrostático 2D / 3D](CalculoCampoElectrostaticos/)

- **Descripción:** Calcula el vector de campo electrostático $\vec{E}$ generado por cualquier número de cargas puntuales sobre un punto arbitrario $P$, aplicando la Ley de Coulomb y el Principio de Superposición.
- **Comportamiento adaptativo:**
  - Si todas las coordenadas $z = 0$, opera en **Modo 2D** (gráfica en plano $XY$, líneas de flujo _streamlines_ de fondo, polaridad con halos y ángulo $\theta$).
  - Si alguna coordenada $z \neq 0$, conmuta a **Modo 3D** (perspectiva espacial con proyecciones ortogonales al piso y vectores en 3D).
- **Cómo usarlo:**

  ```bash
  cd CalculoCampoElectrostaticos
  # Edita datos.txt con tus cargas y punto P
  mise run all
  ```

  El reporte PDF completo se genera en `CalculoCampoElectrostaticos/output/reporte_campo_electrico.pdf`.

---

---

## ⚡ ¿Cómo usar el binario ya construido sin recompilar Rust cada vez?

Una de las grandes ventajas de esta arquitectura es que **no necesitas recompilar el código de Rust cada vez que cambias los datos del ejercicio**:

1. **Si solo modificas `datos.txt` (cambias cargas o el punto $P$):**
   ```bash
   mise run all
   # o si solo quieres ver los números en la terminal:
   mise run calc
   ```
   Mise detectará que el binario `target/release/electrostatic_calc` ya existe y **lo ejecutará directamente en microsegundos**, omitiendo cualquier llamada innecesaria a Cargo.

2. **Si modificas el código fuente en Rust (`src/main.rs`) y quieres recompilarlo:**
   ```bash
   mise run build
   ```
   Compilará de nuevo el motor en modo optimizado `--release`.

3. **Si quieres forzar una recompilación totalmente limpia desde cero:**
   ```bash
   mise run rebuild
   ```

---

## 🛠️ Comandos de Mise por Proyecto

Dentro de la carpeta de cualquier proyecto, dispones de los siguientes comandos de control:

| Comando | Acción |
| :--- | :--- |
| `mise run all` | Ejecuta el pipeline completo usando el binario existente (sin recompilar si ya está listo). |
| `mise run calc` | Ejecuta únicamente los cálculos con el binario ya construido (solo compila la primera vez si no existe). |
| `mise run build` | **Compila o recompila el motor de Rust** en modo `--release` (usar solo si editas archivos en `src/`). |
| `mise run rebuild` | Limpia la caché y fuerza la **recompilación desde cero** de Rust (`cargo clean && cargo build --release`). |
| `mise run plot` | Genera o actualiza la gráfica con Python (2D/3D). |
| `mise run report` | Compila el informe técnico en PDF con Typst. |
| `mise run open` | Abre el PDF generado en el visor predeterminado del sistema. |
| `mise run clean` | Limpia los binarios y archivos temporales (`target/` y `output/`). |

---

## 👤 Autor

- **Juan Diego** - [GitHub @JuanDiego360](https://github.com/JuanDiego360)
- Repositorio: [`JuanDiego360/FisicaRustPythonTyspt`](https://github.com/JuanDiego360/FisicaRustPythonTyspt)

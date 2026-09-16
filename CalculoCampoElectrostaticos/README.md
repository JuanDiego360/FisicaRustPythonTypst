# ⚡ Calculador de Campo Electrostático 2D / 3D

Proyecto del ecosistema **FisicaRustPythonTyspt** para calcular y simular el campo electrostático $\vec{E}$ generado por cualquier número de cargas puntuales sobre un punto arbitrario $P$.

---

## 🚀 Flujo de Trabajo y Comandos Mise

### 1. Edición de Datos
Modifica el archivo [`datos.txt`](datos.txt) con tus cargas y el punto de interés $P$.

### 2. Ejecutar con el Binario ya Compilado (Sin recompilar Rust)
Si solo cambiaste `datos.txt`, ejecuta:
```bash
mise run all
```
Mise detectará automáticamente que el binario de Rust `target/release/electrostatic_calc` ya existe y **lo ejecutará directamente sin recompilar**, generando el PDF en `output/reporte_campo_electrico.pdf`.

Si solo quieres ver los resultados numéricos en la consola:
```bash
mise run calc
```

### 3. ¿Cómo recompilar si modificas el código de Rust?
Si modificas el código fuente en `src/main.rs`:
```bash
# Compila los cambios en modo release optimizado
mise run build

# O para forzar una recompilación limpia desde cero
mise run rebuild
```

---

## 🛠️ Tabla de Comandos Mise

| Comando | Descripción |
| :--- | :--- |
| `mise run all` | Ejecuta todo el flujo usando el binario existente (sin recompilar si ya está construido). |
| `mise run calc` | Realiza el cálculo físico vectorial (solo compila la primera vez si no existe el binario). |
| `mise run build` | Compila o recompila el motor en Rust (`cargo build --release`). |
| `mise run rebuild` | Limpia caché y recompila todo Rust desde cero (`cargo clean && cargo build --release`). |
| `mise run plot` | Genera o actualiza la gráfica con Python (2D/3D). |
| `mise run report` | Compila el informe técnico en PDF con Typst. |
| `mise run open` | Abre el PDF resultante en tu visor del sistema. |
| `mise run clean` | Elimina las carpetas temporales `output/` y `target/`. |

---

## 📋 Formato de `datos.txt`

### Modo 2D Automático (z = 0)
```txt
PUNTO: 0.0  0.2  0.0

CARGAS:
#   x        y        z        q (C)        nombre
  -0.2      0.0      0.0      3.0e-6        q1
   0.2      0.0      0.0     -3.0e-6        q2
   0.0     -0.2      0.0      1.5e-6        q3
```

### Modo 3D Automático (cualquier z != 0)
```txt
PUNTO: 0.1  0.2  0.3

CARGAS:
#   x        y        z        q (C)        nombre
  -0.2      0.1      0.0      2.5e-6        q1
   0.2     -0.1      0.2     -3.0e-6        q2
   0.0      0.3     -0.1      1.8e-6        q3
   0.1      0.0      0.4     -1.0e-6        q4
```

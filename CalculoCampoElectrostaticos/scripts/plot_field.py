#!/usr/bin/env python3
"""
Visualizador de Campo Electrostático (2D y 3D)
Genera representaciones gráficas de alta resolución para el informe Typst.
"""

import sys
import json
import os
import math
import numpy as np
import matplotlib.pyplot as plt

def format_sci_math(val, unit="\\mathrm{N/C}"):
    """Formatea valores en formato LaTeX / Mathtext seguro."""
    if abs(val) < 1e-15:
        return f"0 \\; {unit}" if unit else "0"
    exp = int(math.floor(math.log10(abs(val))))
    mantissa = val / (10 ** exp)
    if -1 <= exp <= 2:
        return f"{val:.2f} \\; {unit}" if unit else f"{val:.2f}"
    return f"{mantissa:.2f} \\times 10^{{{exp}}} \\; {unit}" if unit else f"{mantissa:.2f} \\times 10^{{{exp}}}"

def plot_2d(data, output_path):
    fig, ax = plt.subplots(figsize=(10, 8.5), dpi=300)
    
    # Paleta de colores sobria y elegante
    c_pos = "#d90429"   # Rojo rubí para positivas
    c_neg = "#1d3557"   # Azul marino profundo para negativas
    c_point = "#e76f51" # Naranja para punto P
    c_total = "#2a9d8f" # Verde azulado para E_total
    palette = ["#9d4edd", "#f39c12", "#0077b6", "#e85d04", "#4361ee", "#7209b7"]
    
    point_p = data["point_p"]
    px, py = point_p["x"], point_p["y"]
    charges = data["charges"]
    steps = data["steps"]
    e_tot = data["e_total_vec"]
    e_tot_mag = data["e_total_mag"]
    
    # Determinar límites de la gráfica
    all_x = [px] + [c["pos"]["x"] for c in charges]
    all_y = [py] + [c["pos"]["y"] for c in charges]
    
    x_min, x_max = min(all_x), max(all_x)
    y_min, y_max = min(all_y), max(all_y)
    dx = max(x_max - x_min, 0.2)
    dy = max(y_max - y_min, 0.2)
    span = max(dx, dy)
    
    pad = span * 0.45
    x_lim = (min(all_x) - pad, max(all_x) + pad)
    y_lim = (min(all_y) - pad, max(all_y) + pad)
    
    # Campo de fondo con líneas de flujo sutiles (streamplot)
    grid_size = 100
    gx = np.linspace(x_lim[0], x_lim[1], grid_size)
    gy = np.linspace(y_lim[0], y_lim[1], grid_size)
    GX, GY = np.meshgrid(gx, gy)
    
    Ex_grid = np.zeros_like(GX)
    Ey_grid = np.zeros_like(GY)
    k_e = 8.9875517923e9
    
    for c in charges:
        cx, cy = c["pos"]["x"], c["pos"]["y"]
        q = c["q"]
        rx = GX - cx
        ry = GY - cy
        r_sq = rx**2 + ry**2
        r_sq = np.maximum(r_sq, (span * 0.05)**2)
        r = np.sqrt(r_sq)
        factor = k_e * q / (r**3)
        Ex_grid += factor * rx
        Ey_grid += factor * ry
        
    speed = np.sqrt(Ex_grid**2 + Ey_grid**2)
    lw = 0.7 * (speed / (speed.max() + 1e-12))**0.25
    ax.streamplot(
        GX, GY, Ex_grid, Ey_grid,
        color="#dce1e7",
        linewidth=lw,
        density=1.0,
        arrowsize=0.6,
        arrowstyle='->'
    )

    # Líneas de acción desde cada carga a P
    for c in charges:
        cx, cy = c["pos"]["x"], c["pos"]["y"]
        ax.plot([cx, px], [cy, py], linestyle=":", color="#94a3b8", linewidth=1.2, zorder=2)

    # Escala para vectores en el punto P
    target_arrow_len = span * 0.35
    max_e = max([s["e_mag"] for s in steps] + [e_tot_mag, 1.0])
    arrow_scale = target_arrow_len / max_e
    
    # Graficar vectores de campo individuales E_i en P
    for i, step in enumerate(steps):
        col = palette[i % len(palette)]
        ev = step["e_vec"]
        vx = ev["x"] * arrow_scale
        vy = ev["y"] * arrow_scale
        ch_name = step["charge"]["name"]
        
        ax.annotate(
            "", xy=(px + vx, py + vy), xytext=(px, py),
            arrowprops=dict(
                arrowstyle="-|>",
                color=col,
                lw=2.2,
                mutation_scale=14,
            ),
            zorder=6
        )
        v_len = math.hypot(vx, vy)
        if v_len > 1e-12:
            ux, uy = vx / v_len, vy / v_len
            lbl_x = px + vx + ux * 0.05 * span
            lbl_y = py + vy + uy * 0.05 * span
        else:
            lbl_x, lbl_y = px + 0.03 * span, py + 0.03 * span

        ax.text(
            lbl_x, lbl_y,
            f"$\\vec{{E}}_{{{ch_name}}}$",
            color=col,
            fontsize=11.5,
            fontweight="bold",
            ha="center", va="center",
            bbox=dict(boxstyle="round,pad=0.2", facecolor="white", edgecolor=col, alpha=0.9, lw=1.0),
            zorder=7
        )

    # Graficar vector total E_total en P
    tot_vx = e_tot["x"] * arrow_scale
    tot_vy = e_tot["y"] * arrow_scale
    ax.annotate(
        "", xy=(px + tot_vx, py + tot_vy), xytext=(px, py),
        arrowprops=dict(
            arrowstyle="-|>",
            color=c_total,
            lw=3.4,
            mutation_scale=20,
        ),
        zorder=8
    )
    tot_len = math.hypot(tot_vx, tot_vy)
    if tot_len > 1e-12:
        ux, uy = tot_vx / tot_len, tot_vy / tot_len
        tot_lbl_x = px + tot_vx + ux * 0.06 * span
        tot_lbl_y = py + tot_vy + uy * 0.06 * span
    else:
        tot_lbl_x, tot_lbl_y = px + 0.05 * span, py + 0.05 * span

    ax.text(
        tot_lbl_x, tot_lbl_y,
        "$\\vec{E}_{\\mathrm{total}}$",
        color=c_total,
        fontsize=13,
        fontweight="bold",
        ha="center", va="center",
        bbox=dict(boxstyle="round,pad=0.25", facecolor="#e8f5e9", edgecolor=c_total, alpha=0.95, lw=1.5),
        zorder=9
    )

    # Graficar Cargas
    for c in charges:
        cx, cy = c["pos"]["x"], c["pos"]["y"]
        q = c["q"]
        is_positive = q > 0
        color = c_pos if is_positive else c_neg
        sign_char = "+" if is_positive else "−"
        
        # Halo exterior
        ax.scatter(cx, cy, s=450, color=color, alpha=0.22, zorder=4)
        # Núcleo
        ax.scatter(cx, cy, s=260, color=color, edgecolors="white", linewidths=2.0, zorder=5)
        # Signo + o -
        ax.text(cx, cy, sign_char, color="white", fontsize=14, fontweight="bold", ha="center", va="center", zorder=6)
        
        # Etiqueta de la carga
        q_micro = c["q_micro"]
        lbl = f"${c['name']}$: {q_micro:+.2f} $\\mu\\mathrm{{C}}$"
        ax.annotate(
            lbl,
            xy=(cx, cy),
            xytext=(0, -22 if is_positive else 20),
            textcoords="offset points",
            ha="center",
            fontsize=10,
            fontweight="bold",
            color=color,
            bbox=dict(boxstyle="round,pad=0.2", facecolor="white", edgecolor="#ced4da", alpha=0.9),
            zorder=7
        )

    # Graficar Punto P
    ax.scatter(px, py, s=200, color=c_point, edgecolors="#1f2937", linewidths=2.0, marker="o", zorder=8)
    ax.annotate(
        f"Punto $P$\n$({px:.2f}, {py:.2f})$ m",
        xy=(px, py),
        xytext=(-50, -32),
        textcoords="offset points",
        fontsize=10,
        fontweight="bold",
        color="#1f2937",
        bbox=dict(boxstyle="round,pad=0.25", facecolor="#fffbeb", edgecolor=c_point, alpha=0.95, lw=1.2),
        arrowprops=dict(arrowstyle="->", color="#b45309", lw=1.2),
        zorder=9
    )

    # Configuración de ejes y estética
    ax.set_aspect("equal", adjustable="box")
    ax.set_xlim(x_lim)
    ax.set_ylim(y_lim)
    ax.grid(True, linestyle="--", alpha=0.5, color="#cbd5e1")
    ax.axhline(0, color="#94a3b8", linewidth=0.8, alpha=0.7)
    ax.axvline(0, color="#94a3b8", linewidth=0.8, alpha=0.7)
    
    ax.set_xlabel("Coordenada $X$ [m]", fontsize=12, labelpad=8)
    ax.set_ylabel("Coordenada $Y$ [m]", fontsize=12, labelpad=8)
    
    angle_val = data.get("angle_2d_deg")
    ang_text = f", \\; \\theta = {angle_val:.1f}^\\circ" if angle_val is not None else ""
    ax.set_title(
        f"Distribución Electrostática y Campo Resultante en $P$ (Modo 2D)\n"
        f"$|\\vec{{E}}_{{\\mathrm{{total}}}}| = {format_sci_math(e_tot_mag)}{ang_text}$",
        fontsize=12.5,
        pad=14,
        fontweight="bold",
        color="#0f172a"
    )

    # Caja informativa
    info_lines = [
        f"$\\mathbf{{E}}_{{\\mathrm{{total}}}} = ({format_sci_math(e_tot['x'], '')})\\hat{{\\mathbf{{i}}}} + ({format_sci_math(e_tot['y'], '')})\\hat{{\\mathbf{{j}}}}$ N/C",
        f"$|\\mathbf{{E}}_{{\\mathrm{{total}}}}| = {format_sci_math(e_tot_mag)}$",
    ]
    if angle_val is not None:
        info_lines.append(f"$\\theta = {angle_val:.2f}^\\circ$ (respecto a $+X$)")
        
    ax.text(
        0.02, 0.98, "\n".join(info_lines),
        transform=ax.transAxes,
        fontsize=9.5,
        va="top",
        ha="left",
        bbox=dict(boxstyle="round,pad=0.5", facecolor="#f8fafc", edgecolor="#cbd5e1", alpha=0.92, lw=1.2),
        zorder=10
    )

    plt.tight_layout()
    plt.savefig(output_path, dpi=300)
    plt.close()
    print(f"📊 Gráfica 2D guardada exitosamente en '{output_path}'")


def plot_3d(data, output_path):
    fig = plt.figure(figsize=(11, 9), dpi=300)
    ax = fig.add_subplot(111, projection="3d")
    
    c_pos = "#d90429"
    c_neg = "#1d3557"
    c_point = "#e76f51"
    c_total = "#2a9d8f"
    palette = ["#9d4edd", "#f39c12", "#0077b6", "#e85d04", "#4361ee", "#7209b7"]

    point_p = data["point_p"]
    px, py, pz = point_p["x"], point_p["y"], point_p["z"]
    charges = data["charges"]
    steps = data["steps"]
    e_tot = data["e_total_vec"]
    e_tot_mag = data["e_total_mag"]

    all_x = [px] + [c["pos"]["x"] for c in charges]
    all_y = [py] + [c["pos"]["y"] for c in charges]
    all_z = [pz] + [c["pos"]["z"] for c in charges]

    span_x = max(all_x) - min(all_x)
    span_y = max(all_y) - min(all_y)
    span_z = max(all_z) - min(all_z)
    span = max(span_x, span_y, span_z, 0.2)
    
    target_arrow_len = span * 0.4
    max_e = max([s["e_mag"] for s in steps] + [e_tot_mag, 1.0])
    arrow_scale = target_arrow_len / max_e

    # Cargas y líneas guía al plano z=min
    z_floor = min(all_z) - span * 0.1
    for c in charges:
        cx, cy, cz = c["pos"]["x"], c["pos"]["y"], c["pos"]["z"]
        q = c["q"]
        is_pos = q > 0
        col = c_pos if is_pos else c_neg
        
        # Línea de proyección al piso
        ax.plot([cx, cx], [cy, cy], [z_floor, cz], ":", color="#94a3b8", lw=1)
        # Marcador de carga
        ax.scatter([cx], [cy], [cz], s=200, color=col, edgecolors="white", lw=1.5, alpha=0.9)
        ax.text(cx, cy, cz + span*0.04, f"{c['name']} ({c['q_micro']:+.2f} µC)", color=col, fontsize=9.5, fontweight="bold")

    # Líneas discontinuas entre cargas y P
    for c in charges:
        cx, cy, cz = c["pos"]["x"], c["pos"]["y"], c["pos"]["z"]
        ax.plot([cx, px], [cy, py], [cz, pz], ":", color="#94a3b8", lw=1.1, alpha=0.7)

    # Punto P
    ax.scatter([px], [py], [pz], s=220, color=c_point, edgecolors="#1f2937", lw=1.8, marker="o")
    ax.text(px, py, pz + span*0.05, f"P ({px:.2f}, {py:.2f}, {pz:.2f})", color="#1f2937", fontsize=10, fontweight="bold")

    # Vectores individuales Ei
    for i, step in enumerate(steps):
        col = palette[i % len(palette)]
        ev = step["e_vec"]
        vx, vy, vz = ev["x"] * arrow_scale, ev["y"] * arrow_scale, ev["z"] * arrow_scale
        ax.quiver(px, py, pz, vx, vy, vz, color=col, lw=2.0, arrow_length_ratio=0.2, normalize=False)
        ax.text(px + vx, py + vy, pz + vz, f"E_{step['charge']['name']}", color=col, fontsize=10, fontweight="bold")

    # Vector total E_total
    tot_vx = e_tot["x"] * arrow_scale
    tot_vy = e_tot["y"] * arrow_scale
    tot_vz = e_tot["z"] * arrow_scale
    ax.quiver(px, py, pz, tot_vx, tot_vy, tot_vz, color=c_total, lw=3.5, arrow_length_ratio=0.18, normalize=False)
    ax.text(px + tot_vx, py + tot_vy, pz + tot_vz, "E_total", color=c_total, fontsize=12, fontweight="bold")

    ax.set_xlabel("X [m]", labelpad=10, fontweight="bold")
    ax.set_ylabel("Y [m]", labelpad=10, fontweight="bold")
    ax.set_zlabel("Z [m]", labelpad=10, fontweight="bold")
    
    ax.set_title(
        f"Campo Electrostático en 3D en el Punto $P$\n"
        f"$|\\vec{{E}}_{{\\mathrm{{total}}}}| = {format_sci_math(e_tot_mag)}$",
        fontsize=13,
        pad=15,
        fontweight="bold"
    )

    ax.view_init(elev=24, azim=40)
    plt.tight_layout()
    plt.savefig(output_path, dpi=300)
    plt.close()
    print(f"📊 Gráfica 3D guardada exitosamente en '{output_path}'")


def main():
    json_path = sys.argv[1] if len(sys.argv) > 1 else "output/calculo.json"
    output_path = sys.argv[2] if len(sys.argv) > 2 else "output/campo_electrico.png"
    
    if not os.path.exists(json_path):
        print(f"❌ Error: Archivo '{json_path}' no encontrado.")
        sys.exit(1)
        
    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    os.makedirs(os.path.dirname(os.path.abspath(output_path)), exist_ok=True)
    
    if data.get("is_2d", False):
        plot_2d(data, output_path)
    else:
        plot_3d(data, output_path)

if __name__ == "__main__":
    main()

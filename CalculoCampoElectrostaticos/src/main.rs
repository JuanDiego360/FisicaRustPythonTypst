use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const COULOMB_CONSTANT: f64 = 8.9875517923e9; // N * m^2 / C^2 (1 / (4 * pi * epsilon_0))
const EPSILON_0: f64 = 8.8541878128e-12;     // C^2 / (N * m^2)

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let clean = |v: f64| if v.abs() < 1e-14 { 0.0 } else { v };
        Self {
            x: clean(x),
            y: clean(y),
            z: clean(z),
        }
    }

    pub fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn scale(&self, s: f64) -> Vector3D {
        Vector3D::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(&self) -> Result<Vector3D, String> {
        let mag = self.magnitude();
        if mag < 1e-15 {
            Err("Vector de longitud nula, no se puede normalizar".to_string())
        } else {
            Ok(self.scale(1.0 / mag))
        }
    }

    pub fn to_typst_vec(&self, is_2d: bool) -> String {
        if is_2d {
            format!("vec({}, {})", format_scientific_typst(self.x), format_scientific_typst(self.y))
        } else {
            format!(
                "vec({}, {}, {})",
                format_scientific_typst(self.x),
                format_scientific_typst(self.y),
                format_scientific_typst(self.z)
            )
        }
    }

    pub fn to_typst_ij(&self, is_2d: bool) -> String {
        if is_2d {
            format!(
                "({}) thin hat(i) + ({}) thin hat(j)",
                format_scientific_typst(self.x),
                format_scientific_typst(self.y)
            )
        } else {
            format!(
                "({}) thin hat(i) + ({}) thin hat(j) + ({}) thin hat(k)",
                format_scientific_typst(self.x),
                format_scientific_typst(self.y),
                format_scientific_typst(self.z)
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeInput {
    pub id: usize,
    pub name: String,
    pub pos: Vector3D,
    pub pos_typst: String,
    pub q: f64,
    pub q_formatted: String,
    pub q_micro: f64,
    pub is_positive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeStep {
    pub charge: ChargeInput,
    pub r_vec: Vector3D,
    pub r_mag: f64,
    pub r_mag_cubed: f64,
    pub r_hat: Vector3D,
    pub e_vec: Vector3D,
    pub e_mag: f64,
    pub percent_contribution: f64,
    // Representaciones formateadas para Typst
    pub r_vec_typst: String,
    pub r_mag_typst: String,
    pub r_mag_cubed_typst: String,
    pub r_hat_typst: String,
    pub e_vec_typst: String,
    pub e_vec_ij_typst: String,
    pub e_mag_typst: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Angles3D {
    pub theta_deg: f64,
    pub phi_deg: f64,
    pub cos_alpha: f64,
    pub cos_beta: f64,
    pub cos_gamma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub is_2d: bool,
    pub point_p: Vector3D,
    pub point_p_typst: String,
    pub charges: Vec<ChargeInput>,
    pub steps: Vec<ChargeStep>,
    pub e_total_vec: Vector3D,
    pub e_total_mag: f64,
    pub e_total_vec_typst: String,
    pub e_total_ij_typst: String,
    pub e_total_mag_typst: String,
    pub e_total_x_typst: String,
    pub e_total_y_typst: String,
    pub e_total_z_typst: Option<String>,
    pub sum_x_typst: String,
    pub sum_y_typst: String,
    pub sum_z_typst: Option<String>,
    pub angle_2d_deg: Option<f64>,
    pub angle_2d_rad: Option<f64>,
    pub angles_3d: Option<Angles3D>,
    pub constants: HashMap<String, f64>,
    pub num_charges: usize,
}

pub fn format_scientific_typst(val: f64) -> String {
    if val.abs() < 1e-15 {
        return "0".to_string();
    }
    let abs_val = val.abs();
    if abs_val >= 0.001 && abs_val < 10000.0 {
        let s = format!("{:.4}", val);
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        trimmed.to_string()
    } else {
        let exp = val.abs().log10().floor() as i32;
        let mantissa = val / 10.0_f64.powi(exp);
        format!("{:.3} times 10^({})", mantissa, exp)
    }
}

pub fn parse_input_file(content: &str) -> Result<(Vector3D, Vec<ChargeInput>, bool), String> {
    let mut point_p: Option<Vector3D> = None;
    let mut charges: Vec<ChargeInput> = Vec::new();
    let mut reading_charges = false;
    let mut charge_index = 1;

    for (line_num, raw_line) in content.lines().enumerate() {
        let line_num = line_num + 1;
        let line_without_comment = match raw_line.split_once('#') {
            Some((before, _)) => before.trim(),
            None => raw_line.trim(),
        };

        if line_without_comment.is_empty() {
            continue;
        }

        let lower = line_without_comment.to_lowercase();

        if lower.starts_with("punto:") || lower.starts_with("point:") {
            let parts: Vec<&str> = line_without_comment
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split_whitespace()
                .collect();
            if parts.is_empty() {
                return Err(format!("Línea {}: Declaración de PUNTO vacía.", line_num));
            }

            let p = match parts.len() {
                2 => {
                    let x: f64 = parts[0].parse().map_err(|_| format!("Línea {}: Coordenada x inválida en PUNTO", line_num))?;
                    let y: f64 = parts[1].parse().map_err(|_| format!("Línea {}: Coordenada y inválida en PUNTO", line_num))?;
                    Vector3D::new(x, y, 0.0)
                }
                3.. => {
                    let x: f64 = parts[0].parse().map_err(|_| format!("Línea {}: Coordenada x inválida en PUNTO", line_num))?;
                    let y: f64 = parts[1].parse().map_err(|_| format!("Línea {}: Coordenada y inválida en PUNTO", line_num))?;
                    let z: f64 = parts[2].parse().map_err(|_| format!("Línea {}: Coordenada z inválida en PUNTO", line_num))?;
                    Vector3D::new(x, y, z)
                }
                _ => return Err(format!("Línea {}: PUNTO debe tener 2 o 3 coordenadas (x y [z]).", line_num)),
            };
            point_p = Some(p);
            continue;
        }

        if lower.starts_with("cargas:") || lower.starts_with("charges:") {
            reading_charges = true;
            continue;
        }

        if reading_charges {
            let tokens: Vec<&str> = line_without_comment.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            let (pos, q, name) = match tokens.len() {
                3 => {
                    let x: f64 = tokens[0].parse().map_err(|_| format!("Línea {}: x inválido", line_num))?;
                    let y: f64 = tokens[1].parse().map_err(|_| format!("Línea {}: y inválido", line_num))?;
                    let q: f64 = tokens[2].parse().map_err(|_| format!("Línea {}: q inválido", line_num))?;
                    let name = format!("q{}", charge_index);
                    (Vector3D::new(x, y, 0.0), q, name)
                }
                4 => {
                    if let (Ok(z), Ok(q)) = (tokens[2].parse::<f64>(), tokens[3].parse::<f64>()) {
                        let x: f64 = tokens[0].parse().map_err(|_| format!("Línea {}: x inválido", line_num))?;
                        let y: f64 = tokens[1].parse().map_err(|_| format!("Línea {}: y inválido", line_num))?;
                        let name = format!("q{}", charge_index);
                        (Vector3D::new(x, y, z), q, name)
                    } else if let Ok(q) = tokens[2].parse::<f64>() {
                        let x: f64 = tokens[0].parse().map_err(|_| format!("Línea {}: x inválido", line_num))?;
                        let y: f64 = tokens[1].parse().map_err(|_| format!("Línea {}: y inválido", line_num))?;
                        let name = tokens[3].to_string();
                        (Vector3D::new(x, y, 0.0), q, name)
                    } else {
                        return Err(format!("Línea {}: Formato de carga no reconocido: '{}'", line_num, line_without_comment));
                    }
                }
                _ => {
                    let x: f64 = tokens[0].parse().map_err(|_| format!("Línea {}: x inválido", line_num))?;
                    let y: f64 = tokens[1].parse().map_err(|_| format!("Línea {}: y inválido", line_num))?;
                    let z: f64 = tokens[2].parse().map_err(|_| format!("Línea {}: z inválido", line_num))?;
                    let q: f64 = tokens[3].parse().map_err(|_| format!("Línea {}: q inválido", line_num))?;
                    let name = tokens[4..].join(" ");
                    (Vector3D::new(x, y, z), q, name)
                }
            };

            let q_formatted = format_scientific_typst(q);
            let is_positive = q >= 0.0;
            charges.push(ChargeInput {
                id: charge_index,
                name,
                pos,
                pos_typst: String::new(), // se llena luego con is_2d conocido
                q,
                q_formatted,
                q_micro: q * 1e6,
                is_positive,
            });
            charge_index += 1;
        }
    }

    let point_p = point_p.ok_or_else(|| "No se especificó la sección 'PUNTO: x y [z]' en datos.txt".to_string())?;

    if charges.is_empty() {
        return Err("No se encontraron cargas en la sección 'CARGAS:' de datos.txt".to_string());
    }

    let is_2d = point_p.z.abs() < 1e-12 && charges.iter().all(|c| c.pos.z.abs() < 1e-12);

    for c in &mut charges {
        c.pos_typst = c.pos.to_typst_vec(is_2d);
    }

    Ok((point_p, charges, is_2d))
}

pub fn calculate_fields(point_p: Vector3D, charges: Vec<ChargeInput>, is_2d: bool) -> Result<CalculationResult, String> {
    let mut raw_steps = Vec::new();
    let mut e_total_vec = Vector3D::new(0.0, 0.0, 0.0);
    let mut sum_magnitudes = 0.0;

    for ch in &charges {
        let r_vec = point_p.sub(&ch.pos);
        let r_mag = r_vec.magnitude();

        if r_mag < 1e-14 {
            return Err(format!(
                "Singularidad electrostática: El punto P coincide exactamente con la carga '{}' en ({}, {}, {}). El campo es divergente.",
                ch.name, ch.pos.x, ch.pos.y, ch.pos.z
            ));
        }

        let r_hat = r_vec.unit()?;
        let r_mag_cubed = r_mag * r_mag * r_mag;

        let factor = COULOMB_CONSTANT * ch.q / r_mag_cubed;
        let e_vec = r_vec.scale(factor);
        let e_mag = e_vec.magnitude();

        sum_magnitudes += e_mag;
        e_total_vec = e_total_vec.add(&e_vec);

        raw_steps.push((ch.clone(), r_vec, r_mag, r_mag_cubed, r_hat, e_vec, e_mag));
    }

    let mut steps = Vec::new();
    let mut x_terms = Vec::new();
    let mut y_terms = Vec::new();
    let mut z_terms = Vec::new();

    for (ch, r_vec, r_mag, r_mag_cubed, r_hat, e_vec, e_mag) in raw_steps {
        let percent = if sum_magnitudes > 1e-15 {
            (e_mag / sum_magnitudes) * 100.0
        } else {
            0.0
        };

        x_terms.push(format_scientific_typst(e_vec.x));
        y_terms.push(format_scientific_typst(e_vec.y));
        if !is_2d {
            z_terms.push(format_scientific_typst(e_vec.z));
        }

        steps.push(ChargeStep {
            charge: ch,
            r_vec,
            r_mag,
            r_mag_cubed,
            r_hat,
            e_vec,
            e_mag,
            percent_contribution: percent,
            r_vec_typst: r_vec.to_typst_vec(is_2d),
            r_mag_typst: format_scientific_typst(r_mag),
            r_mag_cubed_typst: format_scientific_typst(r_mag_cubed),
            r_hat_typst: r_hat.to_typst_vec(is_2d),
            e_vec_typst: e_vec.to_typst_vec(is_2d),
            e_vec_ij_typst: e_vec.to_typst_ij(is_2d),
            e_mag_typst: format_scientific_typst(e_mag),
        });
    }

    let e_total_mag = e_total_vec.magnitude();

    let (angle_2d_deg, angle_2d_rad, angles_3d) = if is_2d {
        let rad = e_total_vec.y.atan2(e_total_vec.x);
        let mut deg = rad.to_degrees();
        if deg < 0.0 {
            deg += 360.0;
        }
        (Some(deg), Some(rad), None)
    } else {
        let r = if e_total_mag > 1e-15 { e_total_mag } else { 1.0 };
        let cos_alpha = e_total_vec.x / r;
        let cos_beta = e_total_vec.y / r;
        let cos_gamma = e_total_vec.z / r;
        let theta_rad = (e_total_vec.z / r).clamp(-1.0, 1.0).acos();
        let phi_rad = e_total_vec.y.atan2(e_total_vec.x);

        let a3d = Angles3D {
            theta_deg: theta_rad.to_degrees(),
            phi_deg: phi_rad.to_degrees(),
            cos_alpha,
            cos_beta,
            cos_gamma,
        };
        (None, None, Some(a3d))
    };

    let mut constants = HashMap::new();
    constants.insert("k_e".to_string(), COULOMB_CONSTANT);
    constants.insert("epsilon_0".to_string(), EPSILON_0);

    let sum_x_typst = x_terms.join(" + ");
    let sum_y_typst = y_terms.join(" + ");
    let sum_z_typst = if !is_2d { Some(z_terms.join(" + ")) } else { None };

    let res = CalculationResult {
        is_2d,
        point_p,
        point_p_typst: point_p.to_typst_vec(is_2d),
        num_charges: charges.len(),
        charges,
        steps,
        e_total_vec,
        e_total_mag,
        e_total_vec_typst: e_total_vec.to_typst_vec(is_2d),
        e_total_ij_typst: e_total_vec.to_typst_ij(is_2d),
        e_total_mag_typst: format_scientific_typst(e_total_mag),
        e_total_x_typst: format_scientific_typst(e_total_vec.x),
        e_total_y_typst: format_scientific_typst(e_total_vec.y),
        e_total_z_typst: if !is_2d { Some(format_scientific_typst(e_total_vec.z)) } else { None },
        sum_x_typst,
        sum_y_typst,
        sum_z_typst,
        angle_2d_deg,
        angle_2d_rad,
        angles_3d,
        constants,
    };

    Ok(res)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).map(|s| s.as_str()).unwrap_or("datos.txt");
    let output_path = args.get(2).map(|s| s.as_str()).unwrap_or("output/calculo.json");

    println!("============================================================");
    println!("⚡ MOTOR DE CÁLCULO ELECTROSTÁTICO (Rust High Performance)");
    println!("============================================================");
    println!("📖 Leyendo archivo de entrada: {}", input_path);

    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Error al leer '{}': {}", input_path, e);
            std::process::exit(1);
        }
    };

    let (point_p, charges, is_2d) = match parse_input_file(&content) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Error de formato en '{}': {}", input_path, e);
            std::process::exit(1);
        }
    };

    println!("📍 Punto P evaluado: ({:.4}, {:.4}, {:.4}) m", point_p.x, point_p.y, point_p.z);
    println!("🔋 Cantidad de cargas detectadas: {}", charges.len());
    println!("🌐 Modo detectado: {}", if is_2d { "2D (Plano XY, z=0)" } else { "3D Espacial" });

    let result = match calculate_fields(point_p, charges, is_2d) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("❌ Error de cálculo físico: {}", e);
            std::process::exit(1);
        }
    };

    println!("------------------------------------------------------------");
    for step in &result.steps {
        println!(
            "  • Carga {:<4}: q = {:>9.3e} C en ({:.3}, {:.3}, {:.3}) m -> r = {:.4} m | |E| = {:>10.3e} N/C",
            step.charge.name, step.charge.q, step.charge.pos.x, step.charge.pos.y, step.charge.pos.z,
            step.r_mag, step.e_mag
        );
    }
    println!("------------------------------------------------------------");
    println!("🎯 CAMPO ELÉCTRICO TOTAL E_total en P:");
    if result.is_2d {
        println!("   Ex = {:>12.4e} N/C", result.e_total_vec.x);
        println!("   Ey = {:>12.4e} N/C", result.e_total_vec.y);
        println!("   Magnitud |E| = {:>12.4e} N/C", result.e_total_mag);
        if let Some(ang) = result.angle_2d_deg {
            println!("   Dirección θ = {:.2}° respecto a +X", ang);
        }
    } else {
        println!("   Ex = {:>12.4e} N/C", result.e_total_vec.x);
        println!("   Ey = {:>12.4e} N/C", result.e_total_vec.y);
        println!("   Ez = {:>12.4e} N/C", result.e_total_vec.z);
        println!("   Magnitud |E| = {:>12.4e} N/C", result.e_total_mag);
        if let Some(ref a3d) = result.angles_3d {
            println!("   Ángulo polar θ = {:.2}°, azimutal φ = {:.2}°", a3d.theta_deg, a3d.phi_deg);
        }
    }

    if let Some(parent) = Path::new(output_path).parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("❌ No se pudo crear directorio de salida: {}", e);
                std::process::exit(1);
            }
        }
    }

    let json_string = match serde_json::to_string_pretty(&result) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("❌ Error serializando JSON: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = fs::write(output_path, json_string) {
        eprintln!("❌ Error escribiendo archivo '{}': {}", output_path, e);
        std::process::exit(1);
    }

    println!("💾 Resultados guardados exitosamente en '{}'", output_path);
    println!("============================================================");
}

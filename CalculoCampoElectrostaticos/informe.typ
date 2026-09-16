#set page(
  paper: "a4",
  margin: (top: 2.3cm, bottom: 2.3cm, left: 2.2cm, right: 2.2cm),
  header: context {
    if counter(page).get().first() > 1 {
      align(right)[
        #text(size: 8.5pt, fill: rgb("#64748b"))[
          _Cálculo de Campo Electrostático_ | Solución Paso a Paso
        ]
      ]
    }
  },
  footer: context {
    align(center)[
      #text(size: 9pt, fill: rgb("#94a3b8"))[
        Página #counter(page).get().first() de #counter(page).final().first()
      ]
    ]
  }
)

#set text(
  font: "New Computer Modern",
  size: 10.5pt,
  lang: "es"
)

#set par(
  justify: true,
  leading: 0.68em
)

// Cargar datos calculados por el motor de Rust
#let data = json("output/calculo.json")

// Encabezado principal
#block(
  width: 100%,
  fill: rgb("#0f172a"),
  radius: 8pt,
  inset: 18pt,
  stroke: 1pt + rgb("#334155")
)[
  #grid(
    columns: (1fr),
    gutter: 10pt,
    [
      #text(size: 9pt, weight: "bold", fill: rgb("#38bdf8"))[
        LABORATORIO DE FÍSICA COMPUTACIONAL • ELECTROMAGNETISMO
      ]
      #v(3pt)
      #text(size: 20pt, weight: "bold", fill: white)[
        Cálculo y Simulación de Campo Electrostático
      ]
      #v(2pt)
      #text(size: 11pt, fill: rgb("#cbd5e1"))[
        Resolución Vectorial Paso a Paso con Principio de Superposición
      ]
    ]
  )
  #v(10pt)
  #line(length: 100%, stroke: 0.8pt + rgb("#334155"))
  #v(6pt)
  #grid(
    columns: (1fr, 1fr, 1.2fr),
    gutter: 8pt,
    [
      #text(size: 9pt, fill: rgb("#94a3b8"))[Modo de Operación:] \
      #text(size: 10pt, weight: "bold", fill: if data.is_2d { rgb("#34d399") } else { rgb("#60a5fa") })[
        #if data.is_2d [Plano 2D (z = 0)] else [Espacio 3D]
      ]
    ],
    [
      #text(size: 9pt, fill: rgb("#94a3b8"))[Cargas en el Sistema:] \
      #text(size: 10pt, weight: "bold", fill: white)[
        #data.num_charges cargas puntuales
      ]
    ],
    [
      #text(size: 9pt, fill: rgb("#94a3b8"))[Punto de Evaluación $P$:] \
      #text(size: 10pt, weight: "bold", fill: rgb("#fde047"))[
        $P = #eval(data.point_p_typst, mode: "math")$ m
      ]
    ]
  )
]

#v(10pt)

== 1. Planteamiento del Problema

Se considera un sistema de *#data.num_charges cargas puntuales* estáticas en el vacío. El objetivo es determinar analítica y vectorialmente el vector de campo eléctrico resultante $arrow(E)_"total"$ ejercido sobre el punto del espacio $P$, aplicando rigurosamente la Ley de Coulomb y el Principio de Superposición Lineal.

#v(4pt)

#align(center)[
  #table(
    columns: (1fr, 1.3fr, 1.2fr, 1.4fr, 1.2fr),
    fill: (col, row) => if row == 0 { rgb("#f1f5f9") } else if calc.even(row) { rgb("#f8fafc") } else { white },
    stroke: 0.6pt + rgb("#cbd5e1"),
    align: (center + horizon),
    [*Carga*], [*Posición* $arrow(r)_q$ [m]], [*Valor* $q$ [C]], [*Magnitud práctica*], [*Naturaleza*],
    ..data.charges.map(c => (
      text(weight: "bold", fill: rgb("#0f172a"))[#c.name],
      [$#eval(c.pos_typst, mode: "math")$],
      [$#eval(c.q_formatted, mode: "math")$],
      [#calc.round(c.q_micro, digits: 3) $mu"C"$],
      if c.is_positive {
        text(fill: rgb("#dc2626"), weight: "bold")[$+$ Positiva]
      } else {
        text(fill: rgb("#1d4ed8"), weight: "bold")[$−$ Negativa]
      }
    )).flatten()
  )
]

== 2. Marco Teórico y Formulación

El campo electrostático $arrow(E)_i$ generado por una carga puntual $q_i$ ubicada en la posición $arrow(r)_(q_i)$ sobre un punto de observación $arrow(r)_P$ se define mediante la *Ley de Coulomb en forma vectorial*:

$ arrow(E)_i = 1 / (4 pi epsilon_0) frac(q_i, r_i^2) hat(r)_i = k_e frac(q_i, r_i^3) arrow(r)_i $

donde:
- $arrow(r)_i = arrow(r)_P - arrow(r)_(q_i)$ representa el vector de desplazamiento dirigido desde la carga fuente $q_i$ hacia el punto de prueba $P$.
- $r_i = |arrow(r)_i| = sqrt((x_P - x_i)^2 + (y_P - y_i)^2 + (z_P - z_i)^2)$ es la distancia euclidiana entre la carga y el punto $P$.
- $hat(r)_i = frac(arrow(r)_i, r_i)$ es el vector unitario radial correspondiente.
- $k_e = frac(1, 4 pi epsilon_0) approx 8.98755 times 10^9 space "N" dot "m"^2 / "C"^2$ es la constante de Coulomb en el vacío.

Por el *Principio de Superposición*, el campo eléctrico total resultante en el punto $P$ es la suma vectorial de las contribuciones individuales de cada una de las cargas:

#if data.is_2d [
  $ arrow(E)_"total" = sum_(i=1)^N arrow(E)_i = (sum_(i=1)^N E_(i,x)) hat(i) + (sum_(i=1)^N E_(i,y)) hat(j) $
] else [
  $ arrow(E)_"total" = sum_(i=1)^N arrow(E)_i = (sum_(i=1)^N E_(i,x)) hat(i) + (sum_(i=1)^N E_(i,y)) hat(j) + (sum_(i=1)^N E_(i,z)) hat(k) $
]

== 3. Desarrollo Detallado Paso a Paso por Carga

A continuación se realiza el desglose algebraico de las magnitudes y componentes de cada carga:

#for step in data.steps [
  #block(
    width: 100%,
    breakable: false,
    stroke: 0.8pt + rgb("#cbd5e1"),
    radius: 6pt,
    fill: rgb("#f8fafc"),
    inset: 12pt,
    spacing: 12pt
  )[
    #grid(
      columns: (1fr, auto),
      [
        #text(size: 12pt, weight: "bold", fill: rgb("#0f172a"))[
          Cálculo para la Carga #step.charge.name
        ]
        #text(size: 9.5pt, fill: rgb("#64748b"))[
          #if step.charge.is_positive [(Fuente electrostática divergente)] else [(Sumidero electrostático convergente)]
        ]
      ],
      [
        #text(size: 10pt, weight: "bold", fill: if step.charge.is_positive { rgb("#dc2626") } else { rgb("#1d4ed8") })[
          $q = #eval(step.charge.q_formatted, mode: "math") space "C"$ (#calc.round(step.charge.q_micro, digits: 2) $mu"C"$)
        ]
      ]
    )
    #v(3pt)
    #line(length: 100%, stroke: 0.5pt + rgb("#e2e8f0"))
    #v(3pt)

    *Paso 1: Vector de posición relativo $arrow(r)_#step.charge.name$* \
    Se calcula la diferencia vectorial entre el punto $P$ y la posición de la carga:
    $ arrow(r)_#step.charge.name = arrow(r)_P - arrow(r)_(#step.charge.name) = #eval(data.point_p_typst, mode: "math") - #eval(step.charge.pos_typst, mode: "math") = #eval(step.r_vec_typst, mode: "math") space "m" $

    *Paso 2: Distancia radial $r_#step.charge.name$ y factor de escala cúbico* \
    La magnitud de la distancia euclidiana es:
    $ r_#step.charge.name = |arrow(r)_#step.charge.name| = #eval(step.r_mag_typst, mode: "math") space "m" $
    Elevando al cubo para el factor vectorial:
    $ (r_#step.charge.name)^3 = #eval(step.r_mag_cubed_typst, mode: "math") space "m"^3 $

    *Paso 3: Vector unitario $hat(r)_#step.charge.name$* \
    $ hat(r)_#step.charge.name = frac(arrow(r)_#step.charge.name, r_#step.charge.name) = #eval(step.r_hat_typst, mode: "math") $

    *Paso 4: Campo electrostático individual $arrow(E)_#step.charge.name$* \
    Aplicando la constante $k_e$:
    $ arrow(E)_#step.charge.name = k_e frac(q_#step.charge.name, (r_#step.charge.name)^3) arrow(r)_#step.charge.name = #eval(step.e_vec_typst, mode: "math") space "N/C" $
    #if data.is_2d [
      En base cartesiana canónica $(hat(i), hat(j))$:
    ] else [
      En base cartesiana canónica $(hat(i), hat(j), hat(k))$:
    ]
    $ arrow(E)_#step.charge.name = #eval(step.e_vec_ij_typst, mode: "math") space "N/C" $

    *Paso 5: Magnitud del campo individual* \
    $ |arrow(E)_#step.charge.name| = #eval(step.e_mag_typst, mode: "math") space "N/C" quad (#calc.round(step.percent_contribution, digits: 1)% "de la suma total de magnitudes") $
  ]
]

#pagebreak()

== 4. Superposición Vectorial y Campo Total Resultante

Efectuando la sumatoria componente por componente:

#align(center)[
  #block(
    width: 95%,
    stroke: 1pt + rgb("#0284c7"),
    fill: rgb("#f0f9ff"),
    radius: 6pt,
    inset: 12pt
  )[
    $ E_(x,"total") = sum_(i=1)^#data.num_charges E_(i,x) = #eval(data.sum_x_typst, mode: "math") = #eval(data.e_total_x_typst, mode: "math") space "N/C" $
    #v(2pt)
    $ E_(y,"total") = sum_(i=1)^#data.num_charges E_(i,y) = #eval(data.sum_y_typst, mode: "math") = #eval(data.e_total_y_typst, mode: "math") space "N/C" $
    #if not data.is_2d [
      #v(2pt)
      $ E_(z,"total") = sum_(i=1)^#data.num_charges E_(i,z) = #eval(data.sum_z_typst, mode: "math") = #eval(data.e_total_z_typst, mode: "math") space "N/C" $
    ]
  ]
]

El vector de campo eléctrico total resultante en el punto $P$ es:

$ arrow(E)_"total" = #eval(data.e_total_vec_typst, mode: "math") space "N/C" $
$ arrow(E)_"total" = #eval(data.e_total_ij_typst, mode: "math") space "N/C" $

#v(4pt)
*Magnitud del campo eléctrico total:*
#if data.is_2d [
  $ |arrow(E)_"total"| = sqrt(E_(x,"total")^2 + E_(y,"total")^2) = #eval(data.e_total_mag_typst, mode: "math") space "N/C" $
] else [
  $ |arrow(E)_"total"| = sqrt(E_(x,"total")^2 + E_(y,"total")^2 + E_(z,"total")^2) = #eval(data.e_total_mag_typst, mode: "math") space "N/C" $
]

#if data.is_2d [
  *Dirección angular (en el plano $X Y$ respecto al semieje $+X$):*
  $ theta = "arctan2"(E_(y,"total"), E_(x,"total") ) = #calc.round(data.angle_2d_deg, digits: 2) degree quad (#calc.round(data.angle_2d_rad, digits: 4) " rad") $
] else [
  *Ángulos directores y esféricos:*
  - Ángulo polar: $theta = #calc.round(data.angles_3d.theta_deg, digits: 2) degree$
  - Ángulo azimutal: $phi = #calc.round(data.angles_3d.phi_deg, digits: 2) degree$
  - Cosenos directores: $cos alpha = #calc.round(data.angles_3d.cos_alpha, digits: 4)$, $cos beta = #calc.round(data.angles_3d.cos_beta, digits: 4)$, $cos gamma = #calc.round(data.angles_3d.cos_gamma, digits: 4)$
]

#pagebreak()

== 5. Visualización Gráfica del Campo Electrostático

A continuación se presenta la simulación gráfica de alta fidelidad generada por el pipeline de Python:

#align(center)[
  #image("output/campo_electrico.png", width: 78%)
  #v(-4pt)
  #text(size: 8.5pt, fill: rgb("#475569"))[
    *Figura 1:* Simulación espacial del sistema electrostático. Se ilustran las cargas con sus valores, el punto de interés $P$, los vectores de campo componentes $arrow(E)_i$ (flechas delgadas), y el campo eléctrico resultante $arrow(E)_"total"$ (flecha verde destacada). Las líneas de flujo continuas representan la topología de las líneas de fuerza en el espacio.
  ]
]

#v(8pt)

== 6. Tabla Comparativa de Contribuciones

#align(center)[
  #table(
    columns: (1fr, 1.2fr, 1.8fr, 1.4fr, 1.1fr),
    fill: (col, row) => if row == 0 { rgb("#0f172a") } else if calc.even(row) { rgb("#f8fafc") } else { white },
    stroke: 0.5pt + rgb("#cbd5e1"),
    align: (center + horizon),
    [#text(fill: white, weight: "bold")[Carga]],
    [#text(fill: white, weight: "bold")[Distancia $r_i$ [m]]],
    [#text(fill: white, weight: "bold")[Vector $arrow(E)_i$ [N/C]]],
    [#text(fill: white, weight: "bold")[Magnitud $|arrow(E)_i|$ [N/C]]],
    [#text(fill: white, weight: "bold")[% Relativo]],
    ..data.steps.map(s => (
      text(weight: "bold")[#s.charge.name],
      [#eval(s.r_mag_typst, mode: "math")],
      [$#eval(s.e_vec_typst, mode: "math")$],
      [$#eval(s.e_mag_typst, mode: "math")$],
      [#calc.round(s.percent_contribution, digits: 1)%]
    )).flatten(),
    table.cell(colspan: 2, fill: rgb("#e2e8f0"))[#text(weight: "bold")[TOTAL RESULTANTE]],
    table.cell(fill: rgb("#e2e8f0"))[$#eval(data.e_total_vec_typst, mode: "math")$],
    table.cell(fill: rgb("#e2e8f0"))[#text(weight: "bold", fill: rgb("#047857"))[$#eval(data.e_total_mag_typst, mode: "math") space "N/C"$]],
    table.cell(fill: rgb("#e2e8f0"))[#text(weight: "bold")[100%]]
  )
]

#v(12pt)
#align(center)[
  #text(size: 8.5pt, fill: rgb("#94a3b8"))[
    Documento generado automáticamente por el pipeline integrado Rust + Python + Typst mediante mise.
  ]
]

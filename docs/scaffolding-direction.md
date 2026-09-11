# iced-desktop-shell — Dirección para convertirlo en un generador de scaffolding

Repositorio base:

<https://github.com/JhonMA82/iced-desktop-shell>

## Objetivo

Evolucionar `iced-desktop-shell` desde un boilerplate con una única composición visual hacia un **generador de scaffolding para aplicaciones técnicas de escritorio en Rust + Iced**.

La intención no es crear varias plantillas duplicadas ni mantener forks del mismo boilerplate. Debe existir un solo core reusable y estable, capaz de generar diferentes tipos de aplicación mediante:

- presets de layout;
- capacidades opcionales;
- configuración declarativa;
- argumentos de generación;
- defaults sensatos;
- composición de módulos existentes.

PI Agent y Gentle AI deben decidir la implementación técnica más adecuada respetando esta idea y la arquitectura actual del repositorio.

No se busca imponer una estructura interna concreta si existe una solución más simple, idiomática y mantenible.

---

# Principio principal

Separar claramente dos conceptos:

## Layout

Define **cómo se organiza visualmente la aplicación**.

Ejemplos:

- ribbon superior;
- sidebar;
- activity bar;
- explorer;
- workspace central;
- inspector;
- panel inferior;
- status bar;
- toolbars;
- paneles secundarios.

## Features

Define **qué capacidades incluye la aplicación**.

Ejemplos:

- explorer;
- inspector;
- console;
- output;
- logs;
- notifications;
- command palette;
- status bar;
- tabs;
- activity bar;
- toolbar;
- ribbon;
- persistence.

Un layout puede activar features por defecto, pero layout y features no deben ser el mismo concepto.

---

# Filosofía del generador

El generador debe favorecer:

- pocos argumentos;
- defaults útiles;
- composición;
- mínima duplicación;
- código generado legible;
- estructura fácil de modificar por humanos y agentes;
- compatibilidad con la arquitectura actual;
- posibilidad de añadir nuevos presets sin duplicar el core.

Evitar convertir el generador en un sistema excesivamente configurable.

La experiencia ideal debe ser:

```bash
cargo xtask generate my-app --layout technical-ribbon
```

y producir inmediatamente una aplicación coherente y ejecutable.

Después, cuando sea necesario, el usuario puede especificar capacidades:

```bash
cargo xtask generate my-app \
  --layout ide \
  --with activity-bar,tabs,terminal,notifications
```

La sintaxis exacta, el mecanismo de generación y la herramienta utilizada quedan a criterio de PI Agent/Gentle AI si encuentran una opción más idiomática para Rust.

---

# Presets principales

La primera familia de layouts debe cubrir diferentes categorías de aplicaciones técnicas sin intentar representar cada caso posible.

Los presets base recomendados son:

```text
technical-ribbon
ide
studio
operator
minimal
```

No tratar estos presets como aplicaciones distintas.

Son diferentes composiciones del mismo shell.

---

# `technical-ribbon`

Es la evolución y generalización del layout actual inspirado conceptualmente en aplicaciones CAD/CAE de escritorio.

No usar `autocad` como nombre interno ni público.

El preset debe llamarse:

```text
technical-ribbon
```

porque describe mejor el patrón sin asociar el proyecto a un producto específico.

Composición conceptual:

```text
┌──────────────────────────────────────────────────────┐
│ Menu                                                 │
├──────────────────────────────────────────────────────┤
│ Ribbon                                               │
├─────────────┬────────────────────────┬───────────────┤
│ Explorer    │                        │ Inspector     │
│             │       Workspace        │               │
│             │                        │               │
├─────────────┴────────────────────────┴───────────────┤
│ Output / Console / Diagnostics                       │
├──────────────────────────────────────────────────────┤
│ Status Bar                                           │
└──────────────────────────────────────────────────────┘
```

Casos naturales:

- CAD/CAM;
- CNC;
- configuradores de máquinas;
- software de ingeniería;
- configuración industrial;
- herramientas con gran cantidad de comandos;
- software técnico con muchas operaciones agrupadas.

Características importantes:

- Ribbon data-driven.
- Herramientas organizadas por tabs y groups.
- Explorer opcional.
- Inspector opcional.
- Bottom panel opcional.
- Status bar.
- Workspace central dominante.

El preset actual del proyecto debe evolucionar hacia este nombre/concepto sin introducir dependencias de dominio.

---

# `ide`

Preset inspirado en el patrón de workbench de IDEs modernos.

No copiar visualmente VS Code ni ningún otro producto; reutilizar únicamente el patrón de organización.

Composición conceptual:

```text
┌──────────────────────────────────────────────────────┐
│ Menu / Command Center                                │
├───┬──────────────┬──────────────────────┬────────────┤
│ A │ Primary      │                      │ Secondary  │
│ C │ Sidebar      │                      │ Sidebar    │
│ T │              │      Workspace       │            │
│ I │              │                      │            │
│ V │              │                      │            │
├───┴──────────────┴──────────────────────┴────────────┤
│ Problems │ Output │ Terminal │ Logs                  │
├──────────────────────────────────────────────────────┤
│ Status Bar                                           │
└──────────────────────────────────────────────────────┘
```

Conceptos característicos:

- Activity Bar.
- Primary Sidebar.
- Workspace central.
- Secondary Sidebar opcional.
- Bottom Panel.
- Tabs/document tabs cuando la aplicación las necesite.
- Status Bar.
- Command-oriented navigation.

Casos naturales:

- DevTools;
- gestores de servidores;
- administradores de dispositivos;
- herramientas de networking;
- firmware tools;
- database clients;
- OMT/NDI/media control;
- herramientas de infraestructura;
- editores técnicos.

---

# `studio`

Preset orientado a aplicaciones donde el workspace es la pieza central y las áreas periféricas cambian según la actividad.

Inspiración conceptual: aplicaciones creativas y técnicas con workspaces especializados.

Composición:

```text
┌──────────────────────────────────────────────────────┐
│ Workspace Selector / Toolbar                         │
├──────────────┬──────────────────────────┬─────────────┤
│ Hierarchy    │                          │ Outliner /  │
│ Explorer     │                          │ Properties  │
│              │     Main Workspace       │             │
│              │                          │             │
├──────────────┴──────────────────────────┴─────────────┤
│ Timeline / Nodes / Data / Console                    │
└──────────────────────────────────────────────────────┘
```

Conceptos importantes:

- Workspace central prioritario.
- Paneles contextuales.
- Posibilidad futura de diferentes workspace configurations dentro de la misma aplicación.
- Bottom area adaptable.
- Menos énfasis en Ribbon.
- Toolbars y paneles contextuales según actividad.

Casos naturales:

- GIS;
- diagramas;
- editores nodales;
- modelado;
- visualización;
- simulación;
- análisis;
- multimedia;
- tooling científico.

Una evolución natural de este preset es permitir que una sola aplicación tenga diferentes configuraciones como:

```text
Layout
Configure
Analyze
Monitor
Diagnostics
Simulation
```

Estas configuraciones deben reutilizar los mismos comandos, paneles y estado de aplicación cuando corresponda.

---

# `operator`

Preset orientado a operación, monitorización y control.

Debe tener una filosofía diferente de `technical-ribbon`.

Las acciones críticas deben estar claramente visibles y el estado operativo debe ser prioritario.

Composición conceptual:

```text
┌──────────────────────────────────────────────────────┐
│ System / Machine Status                              │
├───────────┬──────────────────────────────┬────────────┤
│ Navigation│                              │ Controls   │
│           │        Main View             │            │
│           │        Dashboard             │            │
│           │        Mimic / Process       │            │
├───────────┴──────────────────────────────┴────────────┤
│ Alarms │ Events │ Diagnostics                         │
└──────────────────────────────────────────────────────┘
```

Casos naturales:

- HMI;
- control industrial;
- CNC operator interface;
- instrumentación;
- hardware control;
- test benches;
- máquinas;
- IoT industrial;
- monitoring de equipos.

Características:

- Estado del sistema prominente.
- Controles operativos persistentes.
- Alarmas/eventos claramente accesibles.
- Menos profundidad de menús.
- No requiere Ribbon por defecto.
- Workspace diseñado para dashboards, mimic panels o visualizaciones.

---

# `minimal`

Preset deliberadamente pequeño.

Debe existir para evitar que todas las aplicaciones generadas nazcan con una interfaz excesiva.

Composición:

```text
┌──────────────────────────────────────────────────────┐
│ Menu / Toolbar                                       │
├──────────────────────────────────────────────────────┤
│                                                      │
│                    Workspace                         │
│                                                      │
├──────────────────────────────────────────────────────┤
│ Status Bar                                           │
└──────────────────────────────────────────────────────┘
```

Casos naturales:

- viewers;
- converters;
- pequeñas utilidades;
- herramientas de diagnóstico;
- aplicaciones de una sola tarea;
- clientes sencillos;
- herramientas internas.

Debe ser la opción cuando Explorer, Inspector, Ribbon o Bottom Panel no aporten valor.

---

# Layouts futuros

No agregar inicialmente presets por cada variación imaginable.

Patrones como:

```text
monitor
inspector
dashboard
```

pueden surgir posteriormente.

Antes de crear un nuevo preset se debe comprobar si puede expresarse mediante:

- uno de los presets base;
- features opcionales;
- configuración;
- composición de paneles.

Crear un nuevo preset solo cuando represente una organización espacial realmente diferente.

---

# Definición declarativa

Los layouts deben poder expresarse mediante una representación declarativa o equivalente.

La forma exacta queda a criterio de implementación, pero conceptualmente debe poder describirse algo similar a:

```text
LayoutPreset
├── top
├── left
├── center
├── right
├── bottom
├── defaults
└── capabilities
```

No se requiere que exista exactamente esta estructura Rust.

El objetivo es que agregar un preset nuevo no implique copiar una aplicación completa.

Los presets deben reutilizar:

- `CommandRegistry`;
- `PanelRegistry`;
- `DockLayout`;
- Theme;
- Persistence;
- Workspace;
- Notifications;
- Task handling;
- Shell state;
- widgets compartidos.

---

# Panel Registry

El sistema actual de paneles genéricos debe seguir siendo la base.

Un preset no debe conocer implementaciones concretas de paneles de dominio.

Debe poder declarar posiciones o capacidades sobre identificadores como:

```text
explorer
inspector
output
console
problems
logs
devices
properties
telemetry
```

sin introducirlos como enums cerrados dentro del shell.

La aplicación generada decide qué paneles concretos existen.

---

# Scaffolding

El generador debe crear una aplicación funcional y compilable, no únicamente carpetas vacías.

El resultado debe tener:

- nombre de aplicación;
- layout seleccionado;
- shell configurado;
- demo mínima reemplazable;
- comandos iniciales;
- paneles requeridos por el preset;
- tema;
- persistencia cuando aplique;
- documentación mínima para agentes;
- estructura preparada para extenderse.

No generar módulos vacíos para features no seleccionadas.

No generar código muerto únicamente para demostrar posibilidades futuras.

---

# Argumentos

Mantener pocos argumentos y hacer que los presets proporcionen defaults útiles.

Conceptualmente se desea soporte para algo equivalente a:

```text
<project-name>

--layout <preset>

--with <features>

--without <features>

--theme <theme>

--app-name <display-name>
```

No es obligatorio usar exactamente estos nombres.

PI Agent/Gentle AI pueden ajustar la CLI si encuentran una convención más idiomática.

---

# `--layout`

Debe aceptar inicialmente:

```text
technical-ribbon
ide
studio
operator
minimal
```

Ejemplo conceptual:

```bash
cargo xtask generate printnc-config --layout technical-ribbon
```

---

# `--with`

Debe permitir activar capacidades adicionales.

Ejemplo conceptual:

```text
--with explorer,inspector,console,statusbar
```

Features potenciales:

```text
activity-bar
bottom-panel
command-palette
console
explorer
inspector
logs
notifications
output
statusbar
tabs
toolbar
ribbon
secondary-sidebar
```

No todas tienen que existir inmediatamente.

Solo exponer argumentos para capacidades realmente implementadas.

---

# `--without`

Debe permitir eliminar una capacidad activada por defecto por el preset.

Ejemplo:

```bash
cargo xtask generate machine-tool \
  --layout technical-ribbon \
  --without inspector
```

El resultado debe seguir siendo visualmente coherente.

---

# Defaults

Cada preset debe tener buenos defaults.

Ejemplo conceptual:

```text
technical-ribbon
    ribbon
    explorer
    inspector
    bottom-panel
    statusbar

ide
    activity-bar
    explorer
    bottom-panel
    statusbar

studio
    explorer
    inspector
    bottom-panel

operator
    navigation
    controls
    alarms

minimal
    toolbar
    statusbar
```

Los defaults exactos pueden evolucionar.

El usuario no debería tener que indicar diez flags para obtener una aplicación razonable.

---

# Validación de combinaciones

El generador debe impedir o resolver combinaciones incoherentes.

Ejemplo:

```text
--layout minimal --without workspace
```

no tendría sentido.

Otro ejemplo:

```text
--with ribbon
```

sobre un layout que no lo usa puede:

- añadirse si la arquitectura lo permite;
- ser rechazado con explicación;
- producir una variación soportada.

La decisión debe ser explícita y predecible.

No generar silenciosamente interfaces rotas.

---

# Extensión futura: múltiples workspaces

Diseñar los presets de forma que en el futuro una aplicación pueda usar más de una configuración de layout.

Ejemplo:

```text
PrintNC Configurator

Configure
    technical-ribbon

Diagnostics
    ide

Monitor
    operator
```

No es necesario implementar esta capacidad inmediatamente.

La arquitectura del generador no debe impedirla.

El objetivo futuro es que un `LayoutPreset` pueda representar tanto:

- el shell inicial de una aplicación;
- como una configuración de workspace dentro de una aplicación más compleja.

---

# Relación entre Commands y layouts

Los layouts nunca deben implementar lógica de dominio.

Un botón del Ribbon, Toolbar, Activity Bar, menú o panel debe ejecutar el mismo `Command`.

Ejemplo conceptual:

```text
Command
    ↓
Ribbon
Toolbar
Shortcut
Command Palette
Context Menu
```

No duplicar acciones según el preset.

Los presets deciden dónde mostrar capacidades.

No deciden cómo funciona la aplicación.

---

# Temas

El sistema de Theme actual debe mantenerse independiente del layout.

Todos los presets deben compartir los mismos tokens y mecanismos de theme.

Un preset puede ajustar:

- spacing;
- density;
- panel dimensions;
- chrome;
- defaults visuales;

pero no debe crear un sistema de estilos paralelo.

El resultado debe conservar identidad visual común entre todos los layouts.

---

# AI-friendly

El proyecto generado debe continuar siendo fácil de entender por agentes.

`AGENTS.md` debe explicar el resultado generado:

- layout seleccionado;
- features habilitadas;
- dónde vive el dominio;
- dónde vive el shell;
- cómo registrar Commands;
- cómo agregar Panels;
- cómo agregar contenido al Workspace;
- qué partes son generadas;
- cuáles pueden modificarse libremente.

El generador no debe producir abstracciones innecesarias que compliquen la navegación del código.

---

# Metadata de generación

Sería útil que cada proyecto generado conserve una pequeña representación declarativa de cómo fue creado.

Por ejemplo, conceptualmente:

```toml
[project]
layout = "ide"
theme = "dark"

features = [
    "activity-bar",
    "explorer",
    "bottom-panel",
    "statusbar"
]
```

La ubicación y formato quedan a criterio de implementación.

Esta metadata puede ser útil posteriormente para:

- agentes;
- regeneración;
- upgrades;
- migraciones;
- Engineering Platform.

No debe convertirse en una segunda fuente de verdad que sobrescriba cambios manuales sin control.

---

# Engineering Platform

La evolución debe considerar que `iced-desktop-shell` podrá convertirse posteriormente en un boilerplate curado de Engineering Platform.

Engineering Platform debería poder seleccionar este boilerplate cuando una solución requiera:

```text
desktop
technical-desktop
industrial-desktop
engineering-tool
operator-ui
developer-tool
hardware-control
```

y pasar argumentos adecuados al generador.

Por tanto, la interfaz del scaffolder debe ser:

- determinista;
- documentada;
- amigable para agentes;
- sin prompts interactivos obligatorios;
- utilizable completamente mediante argumentos.

Puede existir modo interactivo para humanos, pero nunca debe ser la única manera de generar un proyecto.

---

# Compatibilidad con agentes

PI Agent, Gentle AI y otros agentes deben poder:

1. identificar los presets disponibles;
2. conocer sus defaults;
3. consultar features compatibles;
4. ejecutar el generador sin interacción humana;
5. recibir errores claros;
6. modificar posteriormente el proyecto generado sin depender del generador.

El proyecto final debe ser código Rust normal.

No debe requerir que el scaffolder esté presente para continuar desarrollándolo.

---

# Evitar

No convertir cada preset en:

```text
templates/technical-ribbon/
templates/ide/
templates/studio/
templates/operator/
templates/minimal/
```

si eso significa mantener cinco copias casi idénticas del proyecto.

Evitar:

- duplicación de shell;
- duplicación de command system;
- duplicación de theme;
- duplicación de persistence;
- lógica de dominio dentro de layouts;
- enums cerrados para tipos de aplicación;
- decenas de flags;
- configuración difícil de comprender;
- generación de código innecesario;
- dependencia del scaffolder durante runtime;
- cambios que rompan la arquitectura actual sin una razón técnica fuerte.

Si internamente se utilizan templates para generar fragmentos, deben ser componentes pequeños y reutilizables, no aplicaciones completas duplicadas.

---

# Resultado conceptual esperado

`iced-desktop-shell` debe poder evolucionar hacia una experiencia equivalente a:

```bash
iced-desktop-shell new printnc \
    --layout technical-ribbon

iced-desktop-shell new omt-control \
    --layout ide \
    --with notifications

iced-desktop-shell new machine-operator \
    --layout operator

iced-desktop-shell new diagram-editor \
    --layout studio

iced-desktop-shell new converter \
    --layout minimal
```

La sintaxis final puede ser diferente.

Lo importante es el comportamiento:

```text
una base
+
un preset
+
capacidades opcionales
=
aplicación Rust/Iced limpia y lista para desarrollar
```

---

# Criterio arquitectónico

La evolución debe preservar la idea fundamental:

> El shell proporciona infraestructura reusable.  
> El preset define composición visual.  
> Las features agregan capacidades.  
> La aplicación contiene el dominio.  
> Los Commands conectan las superficies de UI con las acciones.

PI Agent y Gentle AI tienen libertad para elegir los mecanismos concretos que produzcan la solución más simple, mantenible e idiomática en Rust/Iced.

No introducir complejidad únicamente para cumplir literalmente ejemplos de este documento.

El resultado debe privilegiar la calidad del boilerplate y la facilidad con la que humanos y agentes puedan construir aplicaciones técnicas reales sobre él.

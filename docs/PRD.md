# Documento de Requisitos de Producto (PRD)
## Proyecto: Capturador de Pantalla Corporativo Open Source

| Metadato | Detalle |
| :--- | :--- |
| **Producto** | Capturador de Pantalla (Screen Capture Desktop Application) |
| **Documento** | Product Requirements Document (PRD) |
| **Versión** | 1.0.0 |
| **Estado** | Aprobado / Base Inicial |
| **Fecha** | 2026-11-09 |
| **Autor** | Julio Cesar Cisneros Palomino |
| **Referencia BRD** | [BRD.md](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/docs/BRD.md) |

---

## 1. Visión del Producto

Desarrollar una aplicación de escritorio open source, moderna, ultraligera y segura, que permita a cualquier usuario en entornos corporativos y remotos capturar instantáneamente cualquier sección de su pantalla, anotarla, ofuscar información confidencial y transferirla al portapapeles o disco local con máxima agilidad, respetando en todo momento la privacidad de datos (Local-First).

---

## 2. Arquetipos de Usuario (User Personas)

### Persona 1: Ingeniero de Software / QA (Carlos, 31 años)
- **Necesidad**: Reportar bugs en Jira/GitHub con evidencias visuales precisas, pasos exactos, flechas y recuadros marcando el defecto.
- **Dolor**: Perder tiempo abriendo herramientas pesadas o tener prohibido usar extensiones o aplicaciones cloud no homologadas por la empresa. Requiere difuminar tokens de prueba, passwords o datos personales en la captura.
- **Objetivo**: Atajo global rápido → selección de área → resaltar error con flecha → difuminar datos sensibles → `Ctrl+C` / `Cmd+C` → pegar directamente en el ticket.

### Persona 2: Analista / Gestor de Proyectos Remoto (Elena, 38 años)
- **Necesidad**: Explicar requerimientos o aclarar dudas de diseño y flujo en canales de Slack, Teams o correos corporativos.
- **Dolor**: La herramienta nativa de su sistema operativo no tiene herramientas de anotación inmediata o es torpe para recortar y copiar al instante.
- **Objetivo**: Experiencia fluida similar a Lightshot, pero sin anuncios, sin pagos y sin riesgos de seguridad de subida a servidores desconocidos.

### Persona 3: Administrador de Seguridad y TI (Fernando, 45 años)
- **Necesidad**: Asegurar que todas las herramientas instaladas en la flota de laptops de la empresa cumplan con normativas de ciberseguridad, no posean puertas traseras ni telemetría que exponga secretos corporativos.
- **Objetivo**: Disponer de un binario transparente, open source, auditable y con arquitectura 100% local.

---

## 3. Mapeo de Épicas e Historias de Usuario

### Épica 1: Captura de Pantalla y Selección de Área (Capture Engine)
- **US-1.1**: Como usuario, quiero presionar una combinación de teclas global para activar el modo de captura instantáneamente, sin importar qué aplicación tenga activa en primer plano.
- **US-1.2**: Como usuario, quiero que la pantalla se congele suavemente con un oscurecimiento sutil y poder arrastrar el cursor para demarcar un área rectangular precisa de captura.
- **US-1.3**: Como usuario, quiero poder ajustar y redimensionar los bordes o mover la región seleccionada después de haber soltado el cursor inicial.
- **US-1.4**: Como usuario con múltiples monitores, quiero poder seleccionar una región en cualquiera de mis pantallas sin distorsiones ni desalineaciones de DPI.
- **US-1.5**: Como usuario, quiero poder cancelar la captura en cualquier momento presionando la tecla `Esc`, restaurando mi pantalla inmediatamente.

### Épica 2: Anotación y Ofuscación de Datos (Annotation Studio)
- **US-2.1**: Como usuario, quiero disponer de una barra de herramientas flotante junto a la selección que incluya: rectángulo, círculo/elipse, flecha, línea recta, lápiz libre y texto.
- **US-2.2**: Como usuario, quiero poder cambiar el color y el grosor del trazo de las herramientas de dibujo.
- **US-2.3**: Como usuario corporativo, quiero disponer de una herramienta de difuminado (blur/pixelate) para pasar sobre textos sensibles, contraseñas o datos personales y hacerlos ilegibles.
- **US-2.4**: Como usuario, quiero disponer de atajos para Deshacer (`Ctrl+Z` / `Cmd+Z`) y Rehacer (`Ctrl+Y` / `Cmd+Shift+Z`) trazos aplicados.

### Épica 3: Acciones de Salida y Portapapeles (Output Actions)
- **US-3.1**: Como usuario, quiero presionar `Enter` o hacer clic en el botón de "Copiar" para enviar la imagen final con anotaciones directamente al portapapeles del sistema operativo y cerrar el overlay.
- **US-3.2**: Como usuario, quiero poder hacer clic en "Guardar como" para almacenar la captura en disco local (PNG, JPG, WebP), o configurar guardado rápido automático en una carpeta designada.
- **US-3.3**: Como usuario, quiero recibir una notificación sutil y no invasiva cuando la imagen se haya copiado con éxito al portapapeles.

### Épica 4: Preferencias y Configuración del Sistema (Settings & System Tray)
- **US-4.1**: Como usuario, quiero que la aplicación cuente con un ícono en la bandeja del sistema (System Tray / Menú de estado) para acceder a las opciones, cambiar atajos o cerrar la aplicación.
- **US-4.2**: Como usuario, quiero poder personalizar la combinación de teclas global de captura (ej. `PrintScreen`, `Cmd+Shift+X`, `Ctrl+Alt+S`).
- **US-4.3**: Como usuario, quiero configurar si la aplicación debe iniciar automáticamente con el sistema operativo.

---

## 4. Requisitos Funcionales Detallados (FR)

| ID | Requisito Funcional | Prioridad | Regla BR Relacionada |
| :--- | :--- | :--- | :--- |
| **FR-001** | **Atajo Global de Activación**: Registro y captura de atajos globales a nivel de sistema operativo para disparar el modo de captura en cualquier momento. | Must Have (MVP) | BR-005, BR-007 |
| **FR-002** | **Captura y Congelamiento de Pantalla**: Obtención de buffer de pantalla de alta fidelidad respetando escala de pantalla (HiDPI / Retina) y múltiples monitores. | Must Have (MVP) | BR-007 |
| **FR-003** | **Overlay Interactivo**: Presentación de capa visual a pantalla completa con atenuación de fondo y cursor de selección interactivo. | Must Have (MVP) | BR-004 |
| **FR-004** | **Selector de Región Rectangular**: Creación de marco con dimensiones en píxeles visibles en tiempo real (ej. `800 x 600 px`). | Must Have (MVP) | BR-004 |
| **FR-005** | **Manipulación de Bounding Box**: Manijas de ajuste en las 4 esquinas y 4 bordes para redimensionar el área, y arrastre interno para mover la selección. | Must Have (MVP) | BR-004 |
| **FR-006** | **Barra de Herramientas Flotante**: Barra adherida a la selección (posicionada automáticamente para no salirse de los límites de pantalla) con botones de herramientas y acciones. | Must Have (MVP) | BR-004 |
| **FR-007** | **Herramienta Rectángulo y Elipse**: Dibujo de figuras geométricas con borde coloreado y fondo transparente o semitransparente. | Must Have (MVP) | BR-004 |
| **FR-008** | **Herramienta Flecha y Línea**: Dibujo de flechas indicadoras precisas con punta vectorial y líneas rectas con control de orientación. | Must Have (MVP) | BR-004 |
| **FR-009** | **Herramienta Lápiz / Trazo Libre**: Dibujo a mano alzada suavizado para notas rápidas o firmas. | Must Have (MVP) | BR-004 |
| **FR-010** | **Herramienta Texto**: Inserción de texto en cualquier posición de la captura con selección de color y tamaño legible. | Must Have (MVP) | BR-004 |
| **FR-011** | **Herramienta Resaltador (Highlighter)**: Trazos semitransparentes (amarillo/verde/etc.) para enfatizar texto o áreas sin ocultar el contenido inferior. | Must Have (MVP) | BR-004 |
| **FR-012** | **Herramienta Ofuscación / Difuminado (Blur/Pixelate)**: Aplicación de filtro de desenfoque gaussiano o pixelado destructivo irreversible sobre la región seleccionada para censura de secretos y PII. | Must Have (MVP) | BR-003, BR-001 |
| **FR-013** | **Historial de Deshacer / Rehacer**: Pila de acciones de anotación para revertir o restaurar trazos (`Undo`/`Redo`). | Must Have (MVP) | BR-004 |
| **FR-014** | **Copia Inmediata a Portapapeles**: Renderizado compuesto (imagen base recortada + capas de anotación aplicadas) y escritura directa al portapapeles en formato MIME `image/png`. | Must Have (MVP) | BR-004 |
| **FR-015** | **Exportación a Archivo Local**: Cuadro de diálogo nativo o guardado directo en ruta configurada (ej. `~/Pictures/Screenshots/Capture_YYYY-MM-DD_HH-mm-ss.png`). | Must Have (MVP) | BR-001, BR-004 |
| **FR-016** | **Cancelación Rápida**: Presionar tecla `Esc` descarta cualquier selección o edición sin realizar escrituras en disco ni portapapeles. | Must Have (MVP) | BR-004 |
| **FR-017** | **Bandeja del Sistema (Tray Icon)**: Ícono residente que provee menú contextual: "Capturar pantalla", "Preferencias", "Acerca de", "Salir". | Must Have (MVP) | BR-005 |
| **FR-018** | **Panel de Configuración**: Interfaz gráfica para parametrizar atajos de teclado, ruta por defecto, formato de guardado e inicio automático con el SO. | Must Have (MVP) | BR-005 |
| **FR-019** | **Fase 2 - Modo Grabación de Pantalla**: Selección de región para captura de video (H.264/WebM) con botón de inicio, pausa y parada. | Nice to Have (v2.0) | BR-001, BR-007 |
| **FR-020** | **Fase 2 - Exportación a GIF**: Conversión optimizada de fragmentos breves de video a formato GIF animado. | Nice to Have (v2.0) | BR-001, BR-007 |

---

## 5. Requisitos No Funcionales (NFR)

- **NFR-001 (Rendimiento de Activación)**: El tiempo transcurrido entre la presión del atajo global de teclado y el despliegue del overlay interactivo de captura NO DEBE superar los 200 milisegundos.
- **NFR-002 (Consumo de Memoria en Reposo)**: El proceso en segundo plano en espera de atajos NO DEBE superar los 50 MB de memoria RAM en reposo.
- **NFR-003 (Huella de Almacenamiento)**: El instalador y los binarios de la aplicación NO DEBEN exceder un tamaño total razonable (objetivo: < 25 MB en binario compilado).
- **NFR-004 (Soberanía y Seguridad de Datos)**: Cero tráfico de red no solicitado. No debe existir ningún paquete saliente hacia internet para telemetría, analíticas o almacenamiento remoto.
- **NFR-005 (Fidelidad Visual y DPI)**: La captura DEBE preservar la escala y resolución nativa de los monitores con factor de escala superior a 1x (HiDPI / Retina displays).
- **NFR-006 (Accesibilidad y Ergonomía)**: Soporte completo para navegación por teclado en atajos principales (`Enter` = Copiar, `Esc` = Salir, `Ctrl+S` = Guardar, `Ctrl+Z` = Deshacer).
- **NFR-007 (Compatibilidad de Plataforma)**: Soporte garantizado para:
  - macOS: 12.0 (Monterey) en adelante (arquitecturas Apple Silicon M1/M2/M3 y x86_64).
  - Windows: Windows 10 y Windows 11 (64-bit).
  - Linux: Ubuntu 22.04 LTS / Fedora 38+ en entornos X11 y Wayland.

---

## 6. Experiencia de Usuario y Guía de Interacción (UX/UI)

### 6.1 Flujo de Usuario Principal (Happy Path)
```
[Usuario presiona Atajo Global]
        │
        ▼
[Se congela la pantalla con overlay sutil (40% opacidad)]
        │
        ▼
[Usuario arrastra cursor del mouse para definir rectángulo]
        │
        ▼
[Se dibuja la caja delimitadora + Aparece la Barra de Herramientas flotante]
        │
        ├─────────────────────────────────────────────────┐
        ▼                                                 ▼
[Opcional: Aplica flecha/rectángulo/texto/blur]    [Sin anotaciones]
        │                                                 │
        └────────────────────────┬────────────────────────┘
                                 │
                                 ▼
                     [Presiona Enter o botón Copiar]
                                 │
                                 ▼
         [Overlay desaparece + Imagen en Portapapeles]
                                 │
                                 ▼
                    [Listo para pegar en Teams/Slack/Jira]
```

### 6.2 Comportamiento de la Barra de Herramientas Flotante
- Posicionamiento dinámico: por defecto en el borde inferior exterior derecho de la selección. Si la selección está cerca del borde inferior de la pantalla, la barra se reposiciona automáticamente arriba o adentro de la selección.
- Botones de diseño minimalista con iconos vectoriales claros e indicadores de herramienta activa.
- Paleta de colores rápida: Rojo (alerta/defecto), Verde (éxito), Azul (neutral), Amarillo (resaltador), Negro y Blanco.

---

## 7. Criterios de Aceptación del Producto (Definition of Done para v1.0)

1. Todas las historias de usuario de las épicas 1, 2, 3 y 4 (Must Have) están implementadas y pasan pruebas automatizadas.
2. La herramienta funciona de forma idéntica en macOS, Linux y Windows.
3. El filtro de difuminado/ofuscación es irreversible en los píxeles exportados (garantía de seguridad de datos).
4. El pipeline de CI ejecuta análisis de seguridad (SCA y SAST) sin advertencias críticas.

# Documento de Requisitos de Negocio (BRD)
## Proyecto: Capturador de Pantalla Corporativo Open Source

| Metadato | Detalle |
| :--- | :--- |
| **Proyecto** | Capturador de Pantalla (Screen Capture Tool) |
| **Documento** | Business Requirements Document (BRD) |
| **Versión** | 1.0.0 |
| **Estado** | Aprobado / Base Inicial |
| **Fecha** | 2026-11-09 |
| **Autor** | Julio Cesar Cisneros Palomino |
| **Clasificación** | Abierto / Open Source (Gobernanza Corporativa) |

---

## 1. Resumen Ejecutivo

En entornos corporativos y dinámicas de trabajo remoto o híbrido, la comunicación visual asíncrona mediante capturas de pantalla es una necesidad operativa crítica para desarrolladores, analistas de calidad (QA), soporte técnico y usuarios de negocio. 

Sin embargo, las herramientas comerciales más populares del mercado presentan dos barreras insalvables para las organizaciones:
1. **Costos recurrentes y restricciones de licenciamiento**: Imposición de modelos de suscripción de alto costo por usuario que dificultan su despliegue masivo.
2. **Políticas de Seguridad y Privacidad Estrictas**: Prohibición explícita de herramientas gratuitas de terceros (e.g., Lightshot y similares) debido a riesgos graves de fuga de información (exfiltración automática a la nube pública, servidores no auditados, falta de control sobre datos sensibles de clientes o código fuente).

El objetivo de este proyecto es construir una **solución de escritorio de código abierto (Open Source), multiplataforma (macOS, Windows, Linux) y segura por diseño**, que brinde una experiencia de captura rápida y configurable, con arquitectura **Local-First** (100% local, cero fugas a la red) y con total viabilidad legal y técnica para su adopción en empresas e instituciones corporativas.

---

## 2. Declaración del Problema de Negocio

| Dimensión | Situación Actual | Impacto en el Negocio |
| :--- | :--- | :--- |
| **Seguridad de la Información** | Herramientas gratuitas suben capturas a servidores públicos no controlados o incluyen telemetría invasiva. | Riesgo crítico de incumplimiento normativo (GDPR, ISO 27001, SOC 2, Secreto Bancario/Comercial). Bloqueo total por parte de los equipos de Seguridad/CISO. |
| **Costos Operativos** | Las soluciones corporativas aprobadas (e.g. Snagit) exigen presupuestos elevados de licenciamiento. | Sub-licenciamiento, fricción administrativa y usuarios recurriendo a métodos deficientes (tecla ImprPant + Paint). |
| **Productividad Remota** | Falta de estandarización en atajos rápidos y herramientas de recorte y anotación ágiles en las estaciones de trabajo corporativas. | Pérdida de tiempo en la documentación de incidentes, comunicación de requerimientos y retroalimentación entre equipos. |
| **Heterogeneidad de SO** | Los empleados y desarrolladores corporativos utilizan diferentes sistemas operativos (macOS, Linux y Windows). | Fragmentación de flujos de trabajo al no existir una herramienta unificada con el mismo comportamiento en todas las plataformas. |

---

## 3. Visión y Objetivos Estratégicos del Negocio

### 3.1 Objetivos de Negocio (BO)

- **BO-1 (Costo Cero y Cumplimiento de Licencias)**: Proporcionar una alternativa 100% gratuita y Open Source bajo licencia permisiva corporativa (MIT o Apache 2.0), libre de regalías o restricciones de uso empresarial.
- **BO-2 (Privacidad Garantizada y Local-First)**: Garantizar que ninguna imagen, metadato o información procesada salga de la máquina local del usuario sin autorización explícita.
- **BO-3 (Soporte Multiplataforma Total)**: Ofrecer paridad de funcionalidades y experiencia de usuario consistente en macOS, Windows y distribuciones Linux estándar.
- **BO-4 (Agilidad Operativa)**: Reducir el tiempo promedio necesario para capturar una sección de pantalla, redactar anotaciones/ocultamiento y disponerla en el portapapeles a menos de 5 segundos.
- **BO-5 (Alineación con la Constitución)**: Garantizar trazabilidad completa (BRD → PRD → TRD → Código → Pruebas) con pruebas automatizadas y arquitectura desacoplada según la Constitución ratificada del proyecto.

---

## 4. Alcance del Proyecto

### 4.1 En Alcance (Fase 1 - MVP v1.0: Imágenes Estáticas)
- Aplicación de escritorio residente en segundo plano (System Tray / Menú de estado).
- Disparo mediante atajos de teclado globales y configurables por el usuario.
- Selección interactiva y fluida de áreas/regiones rectangulares de la pantalla (soporte multimonitor y pantallas de alta densidad DPI / Retina).
- Herramientas de anotación in-situ inmediatas: flechas, rectángulos, elipses, texto, líneas, marcador/resaltador.
- **Herramienta de Difuminado/Pixelado (Obfuscation)**: Capacidad obligatoria para censurar datos confidenciales (passwords, datos personales PII, números de tarjetas, API keys) antes de compartir la imagen.
- Salida inmediata:
  - Copia directa de la imagen resultante al portapapeles del sistema operativo.
  - Guardado rápido en almacenamiento local (formatos PNG, JPG, WebP) con nomenclatura parametrizable.
- Ajustes de configuración locales (atajos, rutas predeterminadas, formatos).

### 4.2 En Alcance (Fase 2 - Evolución v2.0: Captura de Video y Movimiento)
- Selección de sección de pantalla para grabación de video (formato MP4 / WebM) y GIF animado corto.
- Indicador visual de grabación activa, pausa, detención y temporizador.
- Exportación local de grabaciones y optimización de peso para mensajería corporativa.

### 4.3 Fuera de Alcance Inicial
- Subida automática o almacenamiento en servidores en la nube no corporativos.
- Herramientas de reconocimiento óptico de caracteres (OCR) en la versión 1.0 (evaluable en fases posteriores).
- Edición avanzada de imágenes multilínea de tiempo o filtros complejos propios de suites de diseño.

---

## 5. Partes Interesadas (Stakeholders)

| Rol / Stakeholder | Interés Principal | Criterio de Éxito |
| :--- | :--- | :--- |
| **Oficial de Seguridad / CISO** | Cero fuga de datos, código auditable, sin telemetría ni comunicación saliente obligatoria. | Aprobación de la herramienta en catálogo corporativo de software seguro. |
| **Desarrollador / Tester QA** | Rapidez de captura de bugs, atajos rápidos, anotación de áreas con errores y difuminado de credenciales. | Herramienta ágil, ligera en memoria y sin fricción de interfaz. |
| **Administrador de Sistemas TI** | Instalación sencilla, binarios autocontenidos, soporte para Mac/Win/Linux sin dependencias complejas. | Despliegue empaquetado (.dmg, .exe/msi, .deb/AppImage). |
| **Comunidad Open Source** | Código limpio, modular, bien documentado, con pruebas automáticas y arquitectura extensible. | Facilidad para reportar incidencias, proponer PRs y compilar localmente. |

---

## 6. Reglas de Negocio (Business Rules - BR)

Las siguientes reglas son de cumplimiento OBLIGATORIO para todo desarrollo en el marco de la Constitución del proyecto:

- **BR-001 (Política Local-First Estricta)**: La aplicación NO DEBE realizar ninguna conexión a la red externa para transferir capturas, imágenes o datos del usuario. Cualquier procesamiento gráfico y almacenamiento DEBE efectuarse 100% de manera local en el dispositivo del usuario.
- **BR-002 (Licenciamiento Corporativo Permisivo)**: El software y todas sus dependencias de terceros DEBEN estar bajo licencias compatibles con el uso empresarial sin restricciones virales (ej. MIT, Apache 2.0, BSD).
- **BR-003 (Protección de Información Confidencial)**: La herramienta DEBE incluir funcionalidad nativa de ofuscación (blur/pixelate) para permitir que los usuarios enmascaren información confidencial antes de compartir o copiar la captura.
- **BR-004 (Persistencia y Portapapeles)**: Tras confirmar una captura, la imagen procesada DEBE poder copiarse al portapapeles del sistema operativo en formato estándar compatible con clientes de mensajería (Slack, Teams, etc.) y correo corporativo sin requerir pasos intermedios.
- **BR-005 (Configurabilidad Sin Privilegios de Administrador)**: La configuración de atajos globales y preferencias de guardado DEBE residir en el perfil local del usuario y NO DEBE requerir privilegios de superusuario/administrador una vez instalada la aplicación.
- **BR-006 (Consumo Mínimo de Recursos)**: El proceso residente en segundo plano NO DEBE superar umbrales razonables de memoria RAM ni CPU mientras se encuentra en estado de escucha pasiva, garantizando que no degrade el rendimiento de la estación de trabajo.
- **BR-007 (Paridad de Plataforma)**: Todas las funciones de la Fase 1 DEBEN estar disponibles de forma homogénea en macOS, Windows y Linux, adaptándose a las particularidades del gestor de ventanas de cada sistema.

---

## 7. Criterios de Aceptación del Negocio

1. **Aprobación de Seguridad**: El código fuente debe ser completamente auditable, sin dependencias con vulnerabilidades críticas y con análisis estático en verde.
2. **Prueba de Flujo Completo**: Un usuario presiona la combinación de teclas configurada, selecciona un cuadrante de pantalla en menos de 1 segundo, dibuja una anotación o difumina un texto, presiona Enter (o botón copiar) y la imagen resultante se pega correctamente en una aplicación externa.
3. **Multi-SO Verificado**: La aplicación se ejecuta y compila exitosamente en macOS (Apple Silicon e Intel), Windows 10/11 y Linux (Ubuntu/Debian con X11/Wayland).

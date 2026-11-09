<!--
Sync Impact Report:
- Version change: Unratified template → 1.0.0
- List of modified principles:
  - [PRINCIPLE_1_NAME] → I. Desarrollo Guiado por Pruebas (TDD)
  - [PRINCIPLE_2_NAME] → II. Principios de Diseño SOLID
  - [PRINCIPLE_3_NAME] → III. Clean Architecture y Desacoplamiento
  - [PRINCIPLE_4_NAME] → IV. Simplicidad, Reusabilidad y No Redundancia (DRY, KISS y YAGNI)
  - [PRINCIPLE_5_NAME] → V. Adherencia a Requisitos de Negocio y Arquitectura (BRD y ARD)
- Added sections:
  - ## Seguridad y Gestión de Dependencias (materializa [SECTION_2_NAME])
  - ## Estándares de Calidad y Prácticas de Desarrollo (materializa [SECTION_3_NAME])
  - ## Governance (materializa [GOVERNANCE_RULES] con directrices de gobernanza, trazabilidad y control de cambios)
- Removed sections: Ninguna (todos los marcadores de posición fueron materializados).
- Follow-up TODOs: Ninguno (todos los principios y políticas requeridas fueron formalizados y ratificados).
-->

# Capturador de Pantalla Constitution

## Core Principles

### I. Desarrollo Guiado por Pruebas (TDD)
- **Reglas OBLIGATORIAS**:
  - El ciclo Red-Green-Refactor es OBLIGATORIO: ninguna línea de código de producción DEBE escribirse sin que exista previamente una prueba automatizada que falle demostrando su necesidad.
  - Las pruebas unitarias DEBEN escribirse antes de la lógica de negocio, cubriendo casos nominales, casos borde y manejo de errores.
  - La refactorización DEBE efectuarse únicamente cuando todas las pruebas estén en estado verde, garantizando la preservación del comportamiento sin alterar contratos funcionales.
- **Racional**: Asegura diseño desacoplado, previene regresiones tempranas y consolida una especificación ejecutable y viva de los componentes de software.

### II. Principios de Diseño SOLID
- **Reglas OBLIGATORIAS**:
  - Toda estructura de software DEBE adherirse estrictamente a los cinco principios SOLID:
    1. **Single Responsibility (SRP)**: Cada módulo, clase o componente DEBE tener una única responsabilidad y una única razón para cambiar.
    2. **Open/Closed (OCP)**: Las entidades de software DEBEN estar abiertas para su extensión, pero cerradas para su modificación directa.
    3. **Liskov Substitution (LSP)**: Los subtipos o implementaciones derivadas DEBEN ser completamente sustituibles por sus tipos base sin alterar la corrección del programa.
    4. **Interface Segregation (ISP)**: Los clientes NO DEBEN ser forzados a depender de contratos o interfaces que no utilizan; se DEBEN diseñar interfaces cohesivas y de grano fino.
    5. **Dependency Inversion (DIP)**: Los módulos de alto nivel NO DEBEN depender de módulos de bajo nivel; ambos DEBEN depender de abstracciones. Las abstracciones NO DEBEN depender de los detalles de implementación.
- **Racional**: Garantiza mantenibilidad, extensibilidad y modularidad a largo plazo, minimizando el impacto de cambios y reduciendo el acoplamiento no deseado.

### III. Clean Architecture y Desacoplamiento
- **Reglas OBLIGATORIAS**:
  - La lógica de negocio y las entidades de dominio DEBEN residir en el núcleo, completamente aisladas e independientes de frameworks, bases de datos, librerías externas o interfaces de usuario.
  - La regla de dependencias DEBE cumplirse estrictamente: las dependencias del código fuente DEBEN apuntar siempre hacia el interior (hacia las abstracciones y casos de uso del dominio).
  - El diseño DEBE ser agnóstico respecto a la tecnología, permitiendo sustituir motores de persistencia, protocolos de transporte o frameworks de presentación sin modificar las reglas de negocio.
  - Toda interacción con mecanismos de persistencia, sistemas de terceros o APIs externas DEBE orquestarse a través de puertos y adaptadores (abstracciones e interfaces).
- **Racional**: Protege el activo fundamental de la organización (la lógica de negocio) de la obsolescencia técnica y los ciclos de vida de herramientas de terceros, facilitando su verificación independiente.

### IV. Simplicidad, Reusabilidad y No Redundancia (DRY, KISS y YAGNI)
- **Reglas OBLIGATORIAS**:
  - **DRY (Don't Repeat Yourself)**: Toda pieza de conocimiento o regla lógica DEBE poseer una única representación fidedigna dentro del sistema. NO DEBE duplicarse lógica; se DEBEN extraer y reutilizar componentes comunes.
  - **KISS (Keep It Simple, Stupid)**: La arquitectura y el código DEBEN optar siempre por la solución más simple y directa que resuelva el requerimiento de forma segura y eficaz. NO DEBE introducirse sobreingeniería ni patrones que no aporten valor directo justificado.
  - **YAGNI (You Aren't Gonna Need It)**: NO DEBE desarrollarse funcionalidad, abstracciones, interfaces ni puntos de extensión especulativos para supuestos requerimientos futuros no solicitados formalmente.
- **Racional**: Disminuye la deuda técnica, minimiza el esfuerzo de mantenimiento y reduce la superficie de posibles fallas y vulnerabilidades.

### V. Adherencia a Requisitos de Negocio y Arquitectura (BRD y ARD)
- **Reglas OBLIGATORIAS**:
  - Toda funcionalidad desarrollada DEBE respetar y validar de forma estricta las reglas y flujos de negocio especificados en el Business Requirements Document (BRD), cuando dicho documento exista.
  - La solución técnica DEBE cumplir de manera rigurosa con los lineamientos, patrones, restricciones no funcionales y decisiones consignadas en el Architecture Requirements Document (ARD), cuando dicho documento exista.
  - NO DEBEN implementarse excepciones ni desviaciones técnicas o de negocio respecto al BRD o ARD sin una aprobación formal y documentada del comité técnico o de producto responsable.
- **Racional**: Garantiza la trazabilidad y congruencia entre la visión estratégica de negocio, los estándares de arquitectura corporativa y la implementación de software entregada.

## Seguridad y Gestión de Dependencias

### I. Gestión y Gobernanza de Dependencias Externas
- **Reglas OBLIGATORIAS**:
  - Se PERMITE el uso de frameworks y librerías externas únicamente si cumplen simultáneamente las siguientes condiciones corporativas:
    1. **Licenciamiento**: Poseer licencias compatibles con las políticas corporativas (e.g., licencias permisivas tipo MIT, Apache 2.0, BSD; se prohíben licencias restrictivas o copyleft viral no autorizadas).
    2. **Mantenimiento Activo**: Demostrar soporte activo por parte de su comunidad o fabricante, con versiones estables y parches de seguridad periódicos.
    3. **Análisis de Vulnerabilidades**: NO DEBEN presentar vulnerabilidades de severidad crítica o alta (según métricas CVSS o equivalentes) sin un plan de mitigación formalmente aprobado por seguridad.
  - Toda dependencia DEBE estar inventariada y verificada mediante herramientas automatizadas de análisis de composición de software (SCA) en el pipeline de CI/CD.

### II. Directrices Obligatorias de Seguridad y Protección de Datos
- **Reglas OBLIGATORIAS**:
  - **Prohibición de Secretos en Código**: Queda TERMINANTEMENTE PROHIBIDO almacenar contraseñas, tokens de API, certificados, llaves criptográficas o cualquier tipo de secreto directamente en el código fuente o repositorios. Los secretos DEBEN inyectarse exclusivamente en tiempo de ejecución a través de variables de entorno seguras o servicios corporativos de gestión de secretos (Vaults).
  - **Validación de Entradas**: Toda entrada proveniente de usuarios, servicios externos, red o archivos DEBE ser rigurosamente validada, tipada y desinfectada contra esquemas de lista blanca antes de cualquier procesamiento interno.
  - **Autenticación y Autorización**: El acceso a servicios, APIs y operaciones privilegiadas DEBE implementar autenticación sólida y control de acceso basado en el principio de mínimo privilegio y defensa en profundidad.
  - **Protección de Datos Sensibles**: La información confidencial o datos de carácter sensible/personal (PII) DEBEN cifrarse en tránsito (mediante TLS 1.3 o equivalente) y en reposo. Dichos datos NO DEBEN ser expuestos en archivos de registro (logs), trazas de depuración, respuestas de error ni interfaces públicas.
  - **Manejo Seguro de Errores**: Todo error o excepción DEBE ser capturado de manera controlada. Las respuestas expuestas al usuario final o a clientes externos NO DEBEN exponer detalles internos de la arquitectura, trazas de pila (stack traces) ni información sensible. Los registros de error internos DEBEN ser estructurados y auditables.

## Estándares de Calidad y Prácticas de Desarrollo

### I. Pruebas Automatizadas y Cobertura
- **Reglas OBLIGATORIAS**:
  - La suite de pruebas automatizadas (unitarias, integración y de contrato/sistema según aplique) es OBLIGATORIA para todo código entregado.
  - El 100% de las pruebas automatizadas DEBE ejecutarse y pasar satisfactoriamente en los pipelines de integración continua (CI) antes de cualquier fusión o despliegue.
  - El proyecto DEBE cumplir con los umbrales mínimos de cobertura de código establecidos institucionalmente, sin permitir regresiones en la cobertura global.

### II. Convenciones de Código y Documentación
- **Reglas OBLIGATORIAS**:
  - El código fuente DEBE cumplir con las convenciones oficiales de formateo, nomenclatura y análisis estático (linting) definidas para el ecosistema tecnológico en uso.
  - Toda API, contrato público, módulo central y decisión arquitectónica DEBE contar con documentación técnica clara, precisa y actualizada en conjunto con el código.

### III. Accesibilidad, Componentes Reutilizables y Sistema de Diseño
- **Reglas OBLIGATORIAS**:
  - Toda interfaz de usuario DEBE construirse cumpliendo los estándares corporativos de accesibilidad web/móvil (alineados a las pautas WCAG aplicables), garantizando operatividad y legibilidad universal.
  - Los desarrollos DEBEN reutilizar componentes transversales existentes y cumplir rigurosamente las guías y componentes del Sistema de Diseño Corporativo cuando aplique, evitando la proliferación de componentes visuales ad-hoc.

## Governance

### I. Trazabilidad Integral y Control de Cumplimiento
- **Reglas OBLIGATORIAS**:
  - DEBE existir trazabilidad bidireccional completa y explícita: Requisito de Negocio (BRD) → Decisión Arquitectónica (ARD) → Especificación Técnica → Código Fuente → Pruebas Automatizadas.
  - Todo cambio en el código fuente DEBE estar vinculado a un identificador de requerimiento, incidencia o tarea formalmente autorizada.

### II. Revisión de Código y Puertas de Calidad (Quality Gates)
- **Reglas OBLIGATORIAS**:
  - Ningún cambio DEBE fusionarse a ramas protegidas o promocionarse a ambientes productivos sin una revisión de código (Peer Review) aprobada por al menos un ingeniero calificado.
  - Los revisores de código DEBEN verificar explícitamente el cumplimiento de los principios y políticas establecidos en esta Constitución antes de otorgar su aprobación.
  - Los controles automatizados en el pipeline de CI/CD (análisis estático, linters, SAST, SCA, suite de pruebas) actúan como bloqueadores obligatorios: ningún fallo o vulnerabilidad no aprobada DEBE ser ignorada para permitir una integración.

### III. Procedimiento de Enmiendas y Versionado Semántico
- **Reglas OBLIGATORIAS**:
  - La presente Constitución prevalece sobre cualquier otra guía de estilo local, convenio no documentado o práctica divergente dentro del proyecto.
  - Toda modificación, derogación o incorporación a este documento DEBE gestionarse mediante una propuesta formal de cambio (Pull Request / RFC), detallando justificación, impacto técnico y plan de migración.
  - El versionado de la Constitución se rige estrictamente por Versionado Semántico (MAJOR.MINOR.PATCH):
    - **MAJOR**: Modificaciones que rompan compatibilidad con principios previos, derogaciones o redefiniciones estructurales de gobernanza.
    - **MINOR**: Incorporación de nuevos principios, secciones o adición material de directrices normativas.
    - **PATCH**: Ajustes de redacción, corrección de erratas ortográficas o clarificaciones no semánticas.

**Version**: 1.0.0 | **Ratified**: 2026-11-09 | **Last Amended**: 2026-11-09

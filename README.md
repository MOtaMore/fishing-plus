# Fishing Plus — Dredge Edition (`fishing_plus`)

Mod híbrido (**Contenido + Código Nativo en Rust**) para **Minecraft Horizon (Rust Edition / 26.3)** inspirado en las mecánicas marítimas y de expedición pesquera de **Dredge**.

---

## ⚓ Características de Dredge

### 1. Barco y Bodega de Casillas (Cargo Hold)
- **Bodega de Carga del Barco (`fishing_plus:cargo_crate`)**: Bloque contenedor de estiba naval con remaches y candado náutico. Posee un inventario de **27 casillas (9x3)** ideal para organizar y clasificar tus capturas, motores y cañas tanto en el muelle como a bordo.
- **Motor Fuera de Borda (`fishing_plus:outboard_engine`)**: Motor náutico con hélice y tirador que representa la motorización básica de tu embarcación.
- **Motor Marino Mejorado (`fishing_plus:improved_engine`)**: Motor bicilíndrico de alta compresión con acabados dorados para expediciones lejanas.
- **Lámpara de Proa (`fishing_plus:boat_lamp`)**: Linterna de latón y glowstone para navegar e identificar bancos de peces en la niebla o la noche.

### 2. Sistema de Bancos de Peces
- **Boya de Banco de Peces (`fishing_plus:fishing_buoy`)**: Bloque flotante de superficie con **modelo 3D real** (diseño clásico de boya salvavidas roja y blanca con mástil y farol). Se coloca directamente sobre el agua para marcar visualmente dónde se encuentran los bancos activos de pesca en tus rutas marítimas o crear caladeros propios.
- **Peces por Profundidad y Zona**:
  - **Costeros**: Caballa Rayada (`striped_mackerel`).
  - **Estuarios y Aguas Calmas**: Carpa Dorada (`golden_carp`).
  - **Mar Abierto / Oceánicos**: Atún Azul (`bluefin_tuna`).
  - **Arrecifes / Bioluminiscentes**: Calamar Luminoso (`glowing_squid`).
  - **Aguas Profundas y Fosas**: Rape Abisal (`abyssal_angler`) y Pez Víbora (`viperfish`).

### 3. Dragado de Naufragios y Restos Marinos (Dredging)
- **Grúa de Dragado (`fishing_plus:dredge_crane`)**: Herramienta con gancho triple de salvamento marítimo para dragar el lecho marino y restos de pecios.
- **Chatarra de Naufragio (`fishing_plus:scrap_metal`)**: Metales corroídos y engranajes dragados del mar; se pueden fundir o reutilizar.
- **Rollo de Lona Marina (`fishing_plus:cloth_roll`)**: Tela impermeable recuperada para velas y cubiertas.
- **Madera Tratada (`fishing_plus:refined_wood`)**: Tablones curados con salitre y resina.
- **Reliquia Hundida (`fishing_plus:sunken_trinket`)**: Joyas y cálices arcaicos recuperados de naufragios (con brillo de encantamiento permanente).

---

## 🎨 Guía de Modelado 3D para Horizon (Blockbench)

Para crear modelos 3D personalizados en Horizon (como la boya o bloques con volumen):

1. **Software:** Abre [Blockbench](https://www.blockbench.net/).
2. **Tipo de proyecto:** Selecciona **"Minecraft Block/Item"**.
3. **Modelado:** Añade cubos (`elements`), ajusta coordenadas de 0 a 16 (la escala de 1 bloque) y pinta las caras UV en la textura (16x16 o 32x32).
4. **Exportar:**
   - Ve a **File > Export > Export Block/Item Model**.
   - Se guardará un archivo `.json` con la sección `"elements": [...]`.
5. **Ubicación en el Mod:**
   - Guarda el modelo en: `assets/fishing_plus/models/block/<nombre>.json` (o `models/item/`).
   - Guarda la textura PNG en: `assets/fishing_plus/textures/block/<nombre>.png`.
   - Vincula el modelo en `assets/fishing_plus/blockstates/<nombre>.json`.

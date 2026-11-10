//! # Fishing Plus (Dredge Edition) — Módulo Nativo en Rust (`Trait Mod`)
//!
//! El Launcher de Horizon enlaza este crate con el ejecutable del juego y llama a [`hooks()`].
//! Cada método del trait [`Hooks`] corresponde a un punto de enganche (*hook point*) oficial
//! del motor de Horizon (ver `docs/native-mods.md`).

use horizon_client::mod_api::{
    Flow, Hooks, Host, LootTableLoadEvent, PlayerFishEvent, Text, UseItemEvent, UseItemOnEvent,
    serde_json::json,
};

/// Punto de entrada requerido por Horizon para registrar el Trait Mod.
pub fn hooks() -> Box<dyn Hooks> {
    Box::new(DredgeMod)
}

/// Estructura principal del mod estilo Dredge.
struct DredgeMod;

/// Estados del evento `player_fish` (equivalente a `PlayerFishEvent` de Bukkit):
mod fish_state {
    /// El pescador lanza el sedal o el gancho al agua.
    pub const CAST: i32 = 0;
    /// Una criatura muerde el anzuelo o el gancho golpea un objeto sumergido.
    pub const BITE: i32 = 1;
    /// El pescador recoge el sedal a tiempo y sube la captura a bordo.
    pub const CATCH: i32 = 2;
    /// El sedal se recoge vacío o la criatura escapa hacia las profundidades.
    pub const REEL_EMPTY: i32 = 3;
    /// El anzuelo se engancha a un objeto, pecio o entidad flotante.
    pub const REEL_ENTITY: i32 = 4;
}

impl Hooks for DredgeMod {
    /// Se ejecuta al hacer clic derecho con un objeto sobre un bloque o superficie.
    /// Permite colocar el Barco Arrastrero directamente en el agua o la orilla.
    fn use_item_on(&self, host: &mut dyn Host, event: &mut UseItemOnEvent<'_>) -> Flow {
        let item_id = event.stack.item.as_ref();

        // 1. Colocación del Barco Arrastrero (Dredge Boat)
        if item_id == "fishing_plus:fishing_boat" {
            if let Some(player) = host.acting_player() {
                host.action_bar(
                    player.uuid,
                    Text::literal("⚓ [DREDGE] Barco Arrastrero botado. Usa Shift+Clic para abrir la bodega de carga o pulsa E al navegar."),
                );
            }
            return Flow::Continue;
        }

        // 2. Notificación al colocar una Boya de Pesca (Banco de Peces)
        if item_id == "fishing_plus:fishing_buoy" {
            if let Some(player) = host.acting_player() {
                host.action_bar(
                    player.uuid,
                    Text::literal("⚓ [DREDGE] ¡Boya colocada! Banco de Peces activo: mayores probabilidades de pesca abisal y dragado."),
                );
            }
        }

        Flow::Continue
    }

    /// Se ejecuta al hacer clic derecho en el aire o sobre el agua libre.
    /// Si el jugador mira al agua con el Barco en la mano, lo coloca frente a él.
    fn use_item(&self, host: &mut dyn Host, event: &mut UseItemEvent<'_>) -> Flow {
        if event.stack.item.as_ref() == "fishing_plus:fishing_boat" {
            if let Some(player) = host.acting_player() {
                host.action_bar(
                    player.uuid,
                    Text::literal("⚓ [DREDGE] Barco Arrastrero botado al agua. ¡Sube a bordo y usa su bodega de carga de 27 casillas!"),
                );
            }
            return Flow::Continue;
        }

        Flow::Continue
    }

    /// Se ejecuta al cargar las tablas de botín.
    /// Interceptamos las tablas de botín de pesca y naufragios para añadir:
    /// - Peces comunes y Aberraciones de Dredge.
    /// - Materiales de dragado: chatarra, lona y reliquias.
    fn loot_table_load(&self, event: &mut LootTableLoadEvent<'_>) {
        // 1. Peces en la tabla de pesca
        if event.id == "minecraft:gameplay/fishing/fish" {
            if let Some(pools) = event.table.get_mut("pools").and_then(|p| p.as_array_mut())
                && let Some(first_pool) = pools.first_mut()
                && let Some(entries) = first_pool
                    .get_mut("entries")
                    .and_then(|e| e.as_array_mut())
            {
                // Caballa Rayada (Costero común, peso 25)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:striped_mackerel",
                    "weight": 25
                }));

                // Carpa Dorada (Aguas dulces y estuarios, peso 15)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:golden_carp",
                    "weight": 15
                }));

                // Atún Azul (Gran pez oceánico de altamar, peso 10)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:bluefin_tuna",
                    "weight": 10
                }));

                // Calamar Luminoso (Bioluminiscente de arrecife, peso 8)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:glowing_squid",
                    "weight": 8
                }));

                // Rape Abisal (Fosas abisales, peso 5)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:abyssal_angler",
                    "weight": 5
                }));

                // Pez Víbora (Depredador abisal con fotóforos, peso 3)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:viperfish",
                    "weight": 3
                }));

                // --- ABERRACIONES DE DREDGE (Peces mutados raros) ---
                // Caballa Grotesca (Mutación de tres ojos, peso 3)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:grotesque_mackerel",
                    "weight": 3
                }));

                // Carpa Purulenta (Mutación purulenta de estuario, peso 2)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:festering_carp",
                    "weight": 2
                }));

                // Atún Putrefacto (Carne pálida expuesta, peso 2)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:decaying_tuna",
                    "weight": 2
                }));

                // Víbora del Vacío (Pez abisal alienígena, peso 1)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:void_viper",
                    "weight": 1
                }));

                // Calamar Omnividente (Tentáculos con decenas de ojos, peso 1)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:all_seeing_squid",
                    "weight": 1
                }));
            }
            return;
        }

        // 2. Materiales de dragado en la pesca de chatarra / fondo marino
        if event.id == "minecraft:gameplay/fishing/junk" {
            if let Some(pools) = event.table.get_mut("pools").and_then(|p| p.as_array_mut())
                && let Some(first_pool) = pools.first_mut()
                && let Some(entries) = first_pool
                    .get_mut("entries")
                    .and_then(|e| e.as_array_mut())
            {
                // Chatarra de Naufragio (dragado habitual)
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:scrap_metal",
                    "weight": 14
                }));

                // Rollo de Lona Marina
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:cloth_roll",
                    "weight": 9
                }));

                // Madera Tratada
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:refined_wood",
                    "weight": 9
                }));
            }
            return;
        }

        // 3. Reliquias hundidas y aberraciones en la pesca de tesoros
        if event.id == "minecraft:gameplay/fishing/treasure" {
            if let Some(pools) = event.table.get_mut("pools").and_then(|p| p.as_array_mut())
                && let Some(first_pool) = pools.first_mut()
                && let Some(entries) = first_pool
                    .get_mut("entries")
                    .and_then(|e| e.as_array_mut())
            {
                // Reliquia marina sumergida
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:sunken_trinket",
                    "weight": 4
                }));

                // Aberración de aguas profundas
                entries.push(json!({
                    "type": "minecraft:item",
                    "name": "fishing_plus:all_seeing_squid",
                    "weight": 1
                }));
            }
        }
    }

    /// Se ejecuta en el servidor cuando un jugador utiliza la caña o grúa de pesca.
    /// Envía alertas temáticas de navegación, dragado y pesca de Dredge en la barra de acción.
    fn player_fish(&self, host: &mut dyn Host, event: &mut PlayerFishEvent<'_>) -> Flow {
        match event.state {
            fish_state::CAST => {
                host.action_bar(
                    event.player,
                    Text::literal("⚓ [DREDGE] Anzuelo lanzado a las profundidades marinas..."),
                );
            }
            fish_state::BITE => {
                host.action_bar(
                    event.player,
                    Text::literal("⚓ [DREDGE] ¡Burbujas en el agua! ¡Una criatura del fondo muerde el anzuelo!"),
                );
            }
            fish_state::CATCH => {
                host.action_bar(
                    event.player,
                    Text::literal("⚓ [DREDGE] ¡Captura izada a bordo! Almacénala en la bodega del barco."),
                );
            }
            fish_state::REEL_EMPTY => {
                host.action_bar(
                    event.player,
                    Text::literal("⚓ [DREDGE] El banco de peces se dispersó... El sedal volvió vacío."),
                );
            }
            fish_state::REEL_ENTITY => {
                host.action_bar(
                    event.player,
                    Text::literal("⚓ [DREDGE] ¡Algo pesado en el lecho marino! ¿Un pecio o reliquia hundida?"),
                );
            }
            _ => {}
        }

        Flow::Continue
    }
}

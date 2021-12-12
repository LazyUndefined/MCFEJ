/*
   Copyright 2021 UndefinedSoviet

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/
extern crate rstk;
extern crate serde;
extern crate serde_json;
use rstk::*;
use serde::{Deserialize, Serialize};
// vars
static ENCH_IDS: [&str; 38] = [
    "sharpness",
    "smite",
    "bane_of_arthropods",
    "knockback",
    "fire_aspect",
    "sweeping_edge",
    "looting",
    "impaling",
    "loyalty",
    "riptide",
    "channeling",
    "power",
    "punch",
    "flame",
    "infinity",
    "multishot",
    "piercing",
    "quick_charge",
    "protection",
    "fire_protection",
    "blast_protection",
    "projectile_protection",
    "thorsns",
    "respiration",
    "aqua_affinity",
    "feather_falling",
    "depth_strider",
    "frost_walker",
    "soul_speed",
    "efficiency",
    "silk_touch",
    "fortune",
    "luck_of_the_sea",
    "lure",
    "unbreaking",
    "mending",
    "vanishing_curse",
    "binding_curse",
];
static POT_IDS: [&str; 31] = [
    "speed",
    "slowness",
    "haste",
    "mining_fatigue",
    "strength",
    "instant_health",
    "instant_damage",
    "jump_boost",
    "nausea",
    "regeneration",
    "resistance",
    "fire_resistance",
    "water_breathing",
    "invisibility",
    "blindness",
    "night_vision",
    "hunger",
    "weakness",
    "poison",
    "wither",
    "health_boost",
    "absorption",
    "saturation",
    "glowing",
    "levitation",
    "luck",
    "unluck",
    "slow_falling",
    "conduit_power",
    "bad_omen",
    "hero_of_the_village",
];
#[derive(Serialize, Deserialize, Debug)]
struct Display {
    Name: String,
    Lore: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Enchantment {
    id: String,
    lvl: u32,
}
#[derive(Serialize, Deserialize)]
struct PotionEffect {
    Id: u8,
    Amplifier: u8,
    Duration: u32,
}
#[derive(Serialize, Deserialize)]
struct NBT {
    display: Display,
    Enchantments: Vec<Enchantment>,
    CustomPotionEffects: Vec<PotionEffect>,
}

fn mctext_from(text: &String) -> String {
    format!("{{\"text\": \"{}\"}}", text)
}

fn main() {
    // initialize
    let root = rstk::start_wish().unwrap();
    root.title("MCFEJ");
    // setup widgets
    // header
    let label_give = rstk::make_label(&root);
    label_give.text("[ /give command ]");
    label_give.grid().row(1).column(0).layout();
    // item id
    let label_id = rstk::make_label(&root);
    label_id.text("ID: ");
    label_id.grid().row(2).column(0).layout();
    let entry_id = rstk::make_entry(&root);
    entry_id.width(50);
    entry_id.grid().row(2).column(1).layout();
    // item name
    let label_name = rstk::make_label(&root);
    label_name.text("Name: ");
    label_name.grid().row(3).column(0).layout();
    let entry_name = rstk::make_entry(&root);
    entry_name.width(50);
    entry_name.grid().row(3).column(1).layout();
    // item lore
    let label_lore = rstk::make_label(&root);
    label_lore.text("Lore: ");
    label_lore.grid().row(4).column(0).layout();
    let entry_lore = rstk::make_entry(&root);
    entry_lore.width(50);
    entry_lore.grid().row(4).column(1).layout();
    // enchantments
    let mut widgets_ench: Vec<(&str, rstk::TkCheckButton, rstk::TkEntry)> = Vec::new();
    // enchantments frame
    let frame_ench = rstk::make_label_frame(&root);
    frame_ench.text("Enchantments: ");
    // init widgets_ench
    let mut i: u32 = 0;
    for ench_id in ENCH_IDS {
        let frame = rstk::make_frame(&frame_ench);
        let cbutton_id = rstk::make_check_button(&frame);
        cbutton_id.text(ench_id);
        cbutton_id.grid().row(0).column(0).layout();
        let entry_lvl = rstk::make_entry(&frame);
        entry_lvl.grid().row(0).column(1).layout();
        frame
            .grid()
            .row((i / 2).into())
            .column((i % 2).into())
            .layout();
        widgets_ench.push((ench_id, cbutton_id, entry_lvl));
        i += 1;
    }
    // visibility of frame_ench
    let cbutton_visible_ench = rstk::make_check_button(&root);
    cbutton_visible_ench.text("show Enchantments");
    cbutton_visible_ench.grid().row(5).column(0).layout();
    cbutton_visible_ench.command(move |visible| {
        if visible {
            frame_ench.grid().row(6).column(0).column_span(2).layout();
        } else {
            frame_ench.grid_forget();
        }
    });
    // potion effects
    // effects
    let mut widgets_potion: Vec<(u8, rstk::TkCheckButton, rstk::TkEntry, rstk::TkEntry)> =
        Vec::new();
    // effects frame
    let frame_potion = rstk::make_label_frame(&root);
    frame_potion.text("Potion effects: ");
    // init widgets_potion
    let mut i: u8 = 0;
    for potion_id in POT_IDS {
        let frame = rstk::make_frame(&frame_potion);
        let cbutton_id = rstk::make_check_button(&frame);
        cbutton_id.text(&(format!("{}: ", potion_id)));
        cbutton_id.grid().row(0).column(0).layout();
        let label_amplifier = rstk::make_label(&frame);
        label_amplifier.text("Amplifier: ");
        label_amplifier.grid().row(0).column(1).layout();
        let entry_amplifier = rstk::make_entry(&frame);
        entry_amplifier.width(5);
        entry_amplifier.grid().row(0).column(2).layout();
        let label_duration = rstk::make_label(&frame);
        label_duration.text("Duration: ");
        label_duration.grid().row(0).column(3).layout();
        let entry_duration = rstk::make_entry(&frame);
        entry_duration.width(5);
        entry_duration.grid().row(0).column(4).layout();
        frame
            .grid()
            .row((i / 2).into())
            .column((i % 2).into())
            .layout();
        widgets_potion.push((i + 1, cbutton_id, entry_amplifier, entry_duration));
        i += 1;
    }
    // visibility of frame_ench
    let cbutton_visible_potion = rstk::make_check_button(&root);
    cbutton_visible_potion.text("show potion effects");
    cbutton_visible_potion.grid().row(7).column(0).layout();
    cbutton_visible_potion.command(move |visible| {
        if visible {
            frame_potion.grid().row(8).column(0).column_span(2).layout();
        } else {
            frame_potion.grid_forget();
        }
    });
    // generate button
    let button_gen = rstk::make_button(&root);
    button_gen.text("<Generate command>");
    button_gen.grid().row(0).column(0).layout();
    button_gen.command(move || {
        let disp = Display {
            Name: mctext_from(&entry_name.value_get()),
            Lore: [mctext_from(&entry_lore.value_get())].to_vec(),
        };
        let enabled_enchs: Vec<_> = widgets_ench
            .iter()
            .filter(|&each| each.1.is_selected())
            .collect();
        let enchs = enabled_enchs
            .iter()
            .map(|&each| Enchantment {
                id: each.0.to_string(),
                lvl: each.2.value_get().parse().unwrap(),
            })
            .collect();
        let enabled_effs: Vec<_> = widgets_potion
            .iter()
            .filter(|&each| each.1.is_selected())
            .collect();
        let effs = enabled_effs
            .iter()
            .map(|&each| PotionEffect {
                Id: each.0,
                Amplifier: each.2.value_get().parse().unwrap(),
                Duration: each.3.value_get().parse().unwrap(),
            })
            .collect();
        let nbt = NBT {
            display: disp,
            Enchantments: enchs,
            CustomPotionEffects: effs,
        };
        println!(
            "/give @p {}{}",
            entry_id.value_get(),
            serde_json::to_string(&nbt).unwrap()
        );
    });
    rstk::mainloop();
}

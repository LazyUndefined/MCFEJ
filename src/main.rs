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

#[derive(Serialize, Deserialize, Debug)]
struct Display {
    Name: String,
    Lore: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct NBT {
    display: Display,
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
    label_give.grid().row(0).column(0).layout();
    // item id
    let label_id = rstk::make_label(&root);
    label_id.text("ID: ");
    label_id.grid().row(1).column(0).layout();
    let entry_id = rstk::make_entry(&root);
    entry_id.width(50);
    entry_id.grid().row(1).column(1).layout();
    // item name
    let label_name = rstk::make_label(&root);
    label_name.text("Name: ");
    label_name.grid().row(2).column(0).layout();
    let entry_name = rstk::make_entry(&root);
    entry_name.width(50);
    entry_name.grid().row(2).column(1).layout();
    // item lore
    let label_lore = rstk::make_label(&root);
    label_lore.text("Lore: ");
    label_lore.grid().row(3).column(0).layout();
    let entry_lore = rstk::make_entry(&root);
    entry_lore.width(50);
    entry_lore.grid().row(3).column(1).layout();
    // generated command
    let label_generated_command = rstk::make_label(&root);
    label_generated_command.background("aqua");
    label_generated_command.text("(generated command is displayed here)");
    label_generated_command.grid().row(4).column(1).layout();
    // generate button
    let button_gen = rstk::make_button(&root);
    button_gen.text("<Generate command>");
    button_gen.grid().row(4).column(0).layout();
    button_gen.command(move || {
        let disp = Display {
            Name: mctext_from(&entry_name.value_get()),
            Lore: [mctext_from(&entry_lore.value_get())].to_vec(),
        };
        let nbt = NBT { display: disp };
        label_generated_command.text(
            &(format!(
                "/give @p {}{}",
                entry_id.value_get(),
                serde_json::to_string(&nbt).unwrap()
            )),
        );
        println!(
            "/give @p {}{}",
            entry_id.value_get(),
            serde_json::to_string(&nbt).unwrap()
        );
    });
    rstk::mainloop();
}

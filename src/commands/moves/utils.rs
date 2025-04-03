use crate::structs::{MoveAliases, MoveInfo};

pub async fn get_normal_moves(moves_info: &[MoveInfo], aliases_data: &[MoveAliases]) -> String {

    let mut normal_moves = String::new();
    // Grabs all normal moves
    for moves in moves_info.iter().take_while(|x| x.move_type.to_lowercase() == "normal") {

        normal_moves =
            normal_moves.to_owned() + "\n- **" + &moves.input + " / " + &moves.name + "**";

        if moves.input == moves.name {
            normal_moves = normal_moves.to_owned() + "\n- **" + &moves.input + "**";
        }

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input  {
                normal_moves += "\n\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        normal_moves = normal_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    }
                    else {
                        normal_moves = normal_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                normal_moves = normal_moves.to_owned() + "`\n";
            }
            else {
                continue;
            }
        }
    }
    normal_moves
}

pub async fn get_special_moves(moves_info: &[MoveInfo], aliases_data: &[MoveAliases]) -> String {

    let mut special_moves = String::new();
    // Grabs all special moves
    for moves in moves_info.iter()
        .skip_while(|x| x.move_type.to_lowercase() == "normal")
        .take_while(|x| x.move_type.to_lowercase() == "special" || x.move_type.to_lowercase() == "other") {
        
        special_moves = special_moves.to_owned() + "\n- **"
            + &moves.input + " / " + &moves.name + "**";

        if moves.input == moves.name {
            special_moves = special_moves.to_owned() + "\n- **" + &moves.input + "**";
        }

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                special_moves += "\n\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        special_moves =
                            special_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    }
                    else {
                        special_moves = special_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                special_moves = special_moves.to_owned() + "`\n";
            }
            else {
                continue;
            }
        }
    }
    special_moves
}

pub async fn get_super_moves(moves_info: &[MoveInfo], aliases_data: &[MoveAliases]) -> String {

    let mut super_moves = String::new();
    for moves in moves_info.iter().skip_while(|x| x.move_type.to_lowercase() != "super") {

        super_moves = super_moves.to_owned() + "\n- **"
            + &moves.input + " / " + &moves.name + "**";
        
        if moves.input == moves.name {
            super_moves = super_moves.to_owned() + "\n- **" + &moves.input + "**";
        }

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input  {
                super_moves += "\n\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        super_moves = super_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    }
                    else {
                        super_moves = super_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                super_moves = super_moves.to_owned() + "`\n";
            }
            else {
                continue;
            }
        }
    }
    super_moves
}

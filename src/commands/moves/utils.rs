use crate::structs::MoveList;

pub async fn get_normal_moves(move_list: &[MoveList]) -> String {

    let mut normal_moves = String::new();

    for x in 0..move_list.len() {
        if move_list[x].move_type == "normal" {

            if x == 0 || (x > 0 && move_list[x-1].id != move_list[x].id) {
                if move_list[x].input == move_list[x].name {
                    normal_moves = normal_moves.to_owned() + "\n- **" + &move_list[x].input + "**";
                }
                else {
                    normal_moves = normal_moves.to_owned() + "\n- **" + &move_list[x].input + " / " + &move_list[x].name + "**";
                }
            }

            if move_list[x].alias.is_empty() { continue; }

            if x == 0 || (x > 0 && move_list[x-1].id != move_list[x].id) {
                normal_moves += "\n\tAliases → `";
            }

            if x + 1 < move_list.len() {
                if move_list[x+1].id == move_list[x].id {
                    normal_moves = normal_moves.to_owned() + &move_list[x].alias + "`, `";
                }
                else {
                    normal_moves = normal_moves.to_owned() + &move_list[x].alias + "`\n";

                }
            }
            else {
                normal_moves = normal_moves.to_owned() + &move_list[x].alias;
            }
        }
    }
    normal_moves
}

pub async fn get_special_moves(move_list: &[MoveList]) -> String {

    let mut special_moves = String::new();

    for x in 0..move_list.len() {
        if move_list[x].move_type == "special" || move_list[x].move_type == "other" {

            if x > 0 && move_list[x-1].id != move_list[x].id {
                if move_list[x].input == move_list[x].name {
                    special_moves = special_moves.to_owned() + "\n- **" + &move_list[x].input + "**";
                }
                else {
                    special_moves = special_moves.to_owned() + "\n- **" + &move_list[x].input + " / " + &move_list[x].name + "**";
                }
            }

            if move_list[x].alias.is_empty() { continue; }

            if x == 0 || (x > 0 && move_list[x-1].id != move_list[x].id) {
                special_moves += "\n\tAliases → `";
            }

            if x + 1 < move_list.len() {
                if move_list[x+1].id == move_list[x].id {
                    special_moves = special_moves.to_owned() + &move_list[x].alias + "`, `";
                }
                else {
                    special_moves = special_moves.to_owned() + &move_list[x].alias + "`\n";

                }
            }
            else {
                special_moves = special_moves.to_owned() + &move_list[x].alias;
            }
        }
    }
    special_moves
}

pub async fn get_super_moves(move_list: &[MoveList]) -> String {

    let mut super_moves = String::new();

    for x in 0..move_list.len() {
        if move_list[x].move_type == "super" {

            if x > 0 && move_list[x-1].id != move_list[x].id {
                if move_list[x].input == move_list[x].name {
                    super_moves = super_moves.to_owned() + "\n- **" + &move_list[x].input + "**";
                }
                else {
                    super_moves = super_moves.to_owned() + "\n- **" + &move_list[x].input + " / " + &move_list[x].name + "**";
                }
            }

            if move_list[x].alias.is_empty() { continue; }

            if x == 0 || (x > 0 && move_list[x-1].id != move_list[x].id) {
                super_moves += "\n\tAliases → `";
            }

            if x + 1 < move_list.len() {
                if move_list[x+1].id == move_list[x].id {
                    super_moves = super_moves.to_owned() + &move_list[x].alias + "`, `";
                }
                else {
                    super_moves = super_moves.to_owned() + &move_list[x].alias + "`\n";

                }
            }
            else {
                super_moves = super_moves.to_owned() + &move_list[x].alias + "`";
            }
        }
    }
    super_moves
}

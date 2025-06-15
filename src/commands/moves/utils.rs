use crate::structs::MoveList;

pub async fn get_normal_moves(move_list: &[MoveList]) -> String {

    let mut normal_moves = String::new();

    for x in 0..move_list.len() {
        if move_list[x].move_type.to_lowercase().trim() == "normal" {

            if x == 0 || (x > 0 && move_list[x].id != move_list[x-1].id) {
                if move_list[x].input == move_list[x].name {
                    normal_moves += &("\n- **".to_owned() + &move_list[x].input + "**");
                }
                else {
                    normal_moves += &("\n- **".to_owned() + &move_list[x].input + " / " + &move_list[x].name + "**");
                }
            }

            if move_list[x].alias.is_empty() { continue; }

            if x == 0 || (x > 0 && move_list[x].id != move_list[x-1].id) {
                normal_moves += "\n\tAliases → `";
            }

            if x + 1 < move_list.len() {
                if move_list[x].id == move_list[x+1].id {
                    normal_moves += &(move_list[x].alias.to_owned() + "`, `");
                }
                else {
                    normal_moves += &(move_list[x].alias.to_owned() + "`\n");

                }
            }
            else {
                normal_moves += &move_list[x].alias;
            }
        }
    }
    normal_moves
}

pub async fn get_special_moves(move_list: &[MoveList]) -> String {

    let mut special_moves = String::new();

    for x in 0..move_list.len() {
        if move_list[x].move_type.to_lowercase().trim() == "special" || move_list[x].move_type.to_lowercase().trim() == "other" {

            if x > 0 && move_list[x].id != move_list[x-1].id {
                if move_list[x].input == move_list[x].name {
                    special_moves += &("\n- **".to_owned() + &move_list[x].input + "**");
                }
                else {
                    special_moves += &("\n- **".to_owned() + &move_list[x].input + " / " + &move_list[x].name + "**");
                }
            }

            if move_list[x].alias.is_empty() { continue; }

            if x == 0 || (x > 0 && move_list[x].id != move_list[x-1].id) {
                special_moves += "\n\tAliases → `";
            }

            if x + 1 < move_list.len() {
                if move_list[x].id == move_list[x+1].id {
                    special_moves +=  &(move_list[x].alias.to_owned() + "`, `");
                }
                else {
                    special_moves += &(move_list[x].alias.to_owned() + "`\n");

                }
            }
            else {
                special_moves += &move_list[x].alias;
            }
        }
    }
    special_moves
}

pub async fn get_super_moves(move_list: &[MoveList]) -> String {

    let mut super_moves = String::new();

    for x in 0..move_list.len() {
        if move_list[x].move_type.to_lowercase().trim() == "super" {

            if x > 0 && move_list[x].id != move_list[x-1].id {
                if move_list[x].input == move_list[x].name {
                    super_moves += &("\n- **".to_owned() + &move_list[x].input + "**");
                }
                else {
                    super_moves += &("\n- **".to_owned() + &move_list[x].input + " / " + &move_list[x].name + "**");
                }
            }

            if move_list[x].alias.is_empty() { continue; }

            if x == 0 || (x > 0 && move_list[x].id != move_list[x-1].id) {
                super_moves += "\n\tAliases → `";
            }

            if x + 1 < move_list.len() {
                if move_list[x].id == move_list[x+1].id {
                    super_moves += &(move_list[x].alias.to_owned() + "`, `");
                }
                else {
                    super_moves += &(move_list[x].alias.to_owned() + "`\n");

                }
            }
            else {
                super_moves += &(move_list[x].alias.to_owned() + "`");
            }
        }
    }
    super_moves
}

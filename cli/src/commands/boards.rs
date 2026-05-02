use crate::boards;

pub fn run() {
    println!("Boards supportées :\n");
    for board in boards::list() {
        println!("  {} — flash via {}", board.name, board.flash_tool);
    }
}

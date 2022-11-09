#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::game::Player;
    use super::super::Board;
    use super::super::BoardSlot;
    use indoc::indoc;

    #[test]
    fn smoke() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn board1() {
        let expected = indoc! {"
        | | | | | | | |
        | | | | | | | |
        | | |1| | | | |
        | | |2| | | | |
        | | |1| | | | |
        |2| |1| | | | |
        "};

        let mut board = Board::new();
        board = board.insert(BoardSlot::P1, 2).unwrap_or(board);
        board = board.insert(BoardSlot::P2, 0).unwrap_or(board);
        board = board.insert(BoardSlot::P1, 2).unwrap_or(board);
        board = board.insert(BoardSlot::P2, 2).unwrap_or(board);
        board = board.insert(BoardSlot::P1, 2).unwrap_or(board);
        let actual = board.to_string();
        assert_eq!(expected, actual, "\nExpected:\n{}Actual:\n{}", expected, actual)
    }

    #[test]
    fn board_topping_out_1() {
        let expected = indoc! {"
        | | |1| | | | |
        | | |1| | | | |
        | | |1| | | | |
        | | |1| | | | |
        | | |1| | | | |
        | | |1| | | | |
        "};

        let mut board = Board::new();
        for _ in 0..10 {
            board = board.insert(BoardSlot::P1, 2).unwrap_or(board);

        }

        let actual = board.to_string();
        assert_eq!(expected, actual, "\nExpected:\n{}Actual:\n{}", expected, actual)
    }

    #[test]
    #[should_panic(expected="Slot 99 does not exist!")]
    fn board_invalid_slot() {
        Board::new().insert(BoardSlot::P1, 99).expect("unreachable");
    }

    // check that taking turns works
    #[test]
    fn game_1() {
        let expected = indoc! {"
        | | | | | | | |
        | | | | | | | |
        | | | | | | | |
        | | | | | | | |
        | | | | | | | |
        |1|2|1|2|1|2| |
        "};

        let mut game: Game = Game::new(Player::One);

        for i in 0..6 {
            game.take_turn(i).expect("Bad code, moves should be valid.");
        }

        let actual = game.get_board().to_string();
        assert_eq!(expected, actual, "\nExpected:\n{}Actual:\n{}", expected, actual)
    }
}
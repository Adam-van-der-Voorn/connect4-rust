#[cfg(test)]
mod tests {
    use crate::board::board_slot::BoardSlot;
    use crate::board::Board;
    use crate::game::Dir;
    use crate::game::Game;
    use crate::game::Player;
    use rstest::rstest;

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
        assert_eq!(
            expected, actual,
            "\nExpected:\n{}Actual:\n{}",
            expected, actual
        )
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
        assert_eq!(
            expected, actual,
            "\nExpected:\n{}Actual:\n{}",
            expected, actual
        )
    }

    #[test]
    #[should_panic(expected = "Slot 99 does not exist!")]
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
        |1|2|1| |2|1|2|
        "};

        let mut game: Game = Game::new(Player::One);

        for _ in 0..6 {
            game.move_cursor(Dir::Left);
            game.take_turn();
        }

        let actual = game.get_board().to_string();
        assert_eq!(
            expected, actual,
            "\nExpected:\n{}Actual:\n{}",
            expected, actual
        )
    }

    #[rstest]
    #[case::horizontal_p2_win(vec!(0, 1, 0, 2, 0, 3, 5, 4), Player::Two)]
    #[case::horizontal_p1_win(vec!(0, 6, 1, 5, 2, 6, 3), Player::One)]
    #[case::vertical_p2_win(vec!(0, 0, 2, 0, 3, 0, 2, 0), Player::Two)]
    #[case::vertical_p1_win(vec!(5, 6, 5, 1, 5, 1, 5), Player::One)]
    #[case::diagonal_uphill_p2_win(vec!(
        1, 0, 2, 1, // first two
        2, 2, // third
        3, 3, 3, 3, // final
    ), Player::Two)]
    #[case::diagonal_uphill_p1_win(vec!(
        1, 2, 3, 4, // lay base
        1, 2, 2, // first two
        6, 3, 3, 3, // third
        6, 4, 4, 4, 5, 4, // final
    ), Player::One)]
    #[case::diagonal_uphill_p1_win(vec!(
        0, 1, 2, 3, 0, 1, 2, 3, // lay base
        0, 1, 1, // first two
        5, 2, 2, 2, // third
        6, 3, 3, 3, 5, 3, // final
    ), Player::One)]
    #[case::diagonal_downhill_p1_win(vec!(
        2, 2, 2, 2, // peak
        3, 3, 0, 3, // second
        4, 4, 0, 5, // last two
    ), Player::Two)]
    #[case::diagonal_downhill_p1_win(vec!(
        3, 4, 5, 6, 3, 4, 5, 6, // lay base
        3, 3, 3, 1, 3, // peak
        0, 4, 4, 4, // second
        0, 6, 5, 5, // last two
    ), Player::One)]
    #[case::diagonal_downhill_p1_win(vec!(
        0, 0, 0, 6, 0, 5, // peak
        1, 1, 1, 5, // second
        3, 2, 2, // last two
    ), Player::One)]
    fn should_only_win_at_end(#[case] moves: Vec<usize>, #[case] expected_winner: Player) {
        let mut game = Game::new(Player::One);
        for m in moves {
            // no winner yet...
            assert!(game.check_winner().is_none());

            game.set_cursor(m);
            game.take_turn();
        }
        // should be a winner now!
        let actual_winner = game.check_winner().expect("There shuold be a winner by the end of the move sequence");
        assert_eq!(expected_winner, actual_winner);
    }
}

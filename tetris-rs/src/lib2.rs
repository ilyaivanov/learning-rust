mod game;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::game;
    #[test]
    fn drops_falling_figure_on_step() {
        let mut game_state = game::GameState {
            board: vec![
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ],
            colors: HashMap::new(),
            figure_falling: Vec::new(),
            figure_falling_color: 1,
        };

        assert_eq!(game_state.board[0], [0, 0, 0]);

        game_state.board[0][1] = 1;
        game_state.figure_falling.push(game::Point { x: 1, y: 0 });

        assert_eq!(game_state.board[0], [0, 1, 0]);

        game::drop_one_step(&mut game_state);

        assert_eq!(game_state.board[0], [0, 0, 0]);
        assert_eq!(game_state.board[1], [0, 1, 0]);
    }
}

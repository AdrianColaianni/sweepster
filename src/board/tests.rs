#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn find_nearby_cells() {
        // Remember: r,c or h,w
        let board = Board::new(1, 1, 0);
        assert!(board.nearby_cells((0, 0)).is_empty());

        let board = Board::new(2, 2, 0);
        assert_eq!(board.nearby_cells((0, 0)), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(board.nearby_cells((1, 0)).len(), 3);
        assert_eq!(board.nearby_cells((0, 1)).len(), 3);
        assert_eq!(board.nearby_cells((1, 1)).len(), 3);

        let board = Board::new(3, 3, 0);
        assert_eq!(board.nearby_cells((0, 0)), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(board.nearby_cells((1, 0)).len(), 5);
        assert_eq!(board.nearby_cells((2, 0)).len(), 3);
        assert_eq!(board.nearby_cells((0, 1)).len(), 5);
        assert_eq!(
            board.nearby_cells((1, 1)),
            vec![
                (0, 1),
                (0, 0),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 1),
                (2, 0),
                (2, 2)
            ]
        );
        assert_eq!(board.nearby_cells((2, 1)).len(), 5);
        assert_eq!(board.nearby_cells((0, 2)).len(), 3);
        assert_eq!(board.nearby_cells((1, 2)).len(), 5);
        assert_eq!(board.nearby_cells((2, 2)).len(), 3);

        let board = Board::new(2, 3, 0);
        assert_eq!(board.nearby_cells((0, 0)), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(board.nearby_cells((1, 0)), vec![(0, 0), (1, 1), (1, 0)]);
        assert_eq!(board.nearby_cells((0, 1)).len(), 5);
        assert_eq!(
            board.nearby_cells((1, 1)),
            vec![(0, 1), (0, 0), (1, 0), (2, 1), (2, 0)]
        );
        assert_eq!(board.nearby_cells((0, 2)).len(), 3);
        assert_eq!(board.nearby_cells((1, 2)).len(), 3);
    }
}

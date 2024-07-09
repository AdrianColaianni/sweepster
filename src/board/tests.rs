#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn find_nearby_cells() {
        // Remember: r,c

        let board = Board::new(1, 1, 0);
        assert!(board.nearby_cells((0, 0)).is_empty());

        /*
         * (0, 0) (0, 1)
         * (1, 0) (1, 1)
         */
        let board = Board::new(2, 2, 0);
        let answers = vec![
            ((0, 0), vec![(1, 0), (0, 1), (1, 1)]),
            ((0, 1), vec![(0, 0), (1, 0), (1, 1)]),
            ((1, 0), vec![(0, 0), (0, 1), (1, 1)]),
            ((1, 1), vec![(1, 0), (0, 0), (0, 1)]),
        ];
        for (c, mut a) in answers {
            let mut n = board.nearby_cells(c);
            n.sort();
            a.sort();
            assert_eq!(n, a);
        }

        /*
         * (0, 0) (0, 1) (0, 2)
         * (1, 0) (1, 1) (1, 2)
         * (2, 0) (2, 1) (2, 2)
         */
        let board = Board::new(3, 3, 0);
        let answers = vec![
            ((0, 0), vec![(1, 0), (0, 1), (1, 1)]),
            ((0, 1), vec![(0, 0), (1, 0), (1, 1), (1, 2), (0, 2)]),
            ((0, 2), vec![(0, 1), (1, 1), (1, 2)]),
            ((1, 0), vec![(0, 0), (0, 1), (1, 1), (2, 1), (2, 0)]),
            ((1, 1), vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2), (2, 1), (2, 0), (1, 0)]),
            ((1, 2), vec![(0, 2), (0, 1), (1, 1), (2, 1), (2, 2)]),
            ((2, 0), vec![(1, 0), (1, 1), (2, 1)]),
            ((2, 1), vec![(2, 0), (1, 0), (1, 1), (1, 2), (2, 2)]),
            ((2, 2), vec![(2, 1), (1, 1), (1, 2)]),
        ];
        for (c, mut a) in answers {
            let mut n = board.nearby_cells(c);
            n.sort();
            a.sort();
            assert_eq!(n, a);
        }

        /*
         * (0, 0) (0, 1) (0, 2)
         * (1, 0) (1, 1) (1, 2)
         */
        let board = Board::new(2, 3, 0);
        let answers = vec![
            ((0, 0), vec![(1, 0), (0, 1), (1, 1)]),
            ((0, 1), vec![(0, 0), (1, 0), (1, 1), (1, 2), (0, 2)]),
            ((0, 2), vec![(0, 1), (1, 1), (1, 2)]),
            ((1, 0), vec![(0, 0), (0, 1), (1, 1)]),
            ((1, 1), vec![(1, 0), (0, 0), (0, 1), (0, 2), (1, 2)]),
            ((1, 2), vec![(1, 1), (0, 1), (0, 2)]),
        ];
        for (c, mut a) in answers {
            let mut n = board.nearby_cells(c);
            n.sort();
            a.sort();
            assert_eq!(n, a);
        }
    }
}

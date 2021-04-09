
// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use main;

//     #[test]
//     fn test_row_check() {

//       let n = 10;
//       let board = make_board(n);  
//       let board = set_queen_at(n, board, (5,0));
//       let board = set_queen_at(n, board, (5,1));
//       let board = set_queen_at(n, board, (6,2));
//       let board = set_queen_at(n, board, (n-1,n-1));
//       let board = set_queen_at(n, board, (0,0));


//       // let qat = has_queen_at(n, &board, (0, 0));
//       let qat = row_has_queen(n, &board, 0);
//       assert_eq!(qat, true, "Row 0 has a queen");

//       let qat = row_has_queen(n, &board, 3);
//       assert_eq!(qat, false, "Row 3 does not have a queen");
      
//     }

// }
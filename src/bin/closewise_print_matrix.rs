use std::fmt::Display;

fn clockwise_print_matrix<T>(mat: &Vec<Vec<T>>)
  where T: Display
{
  // 取出矩阵的长宽，并排除矩阵为空的情况
  let rows = mat.len() as i32;

  if rows == 0 {
    return;
  }

  let cols = mat[0].len() as i32;

  if cols == 0 {
    return;
  }

  // 设置四个方向的迭代，以及它们的迭代的目标行或列、迭代起终止索引（总是左小、右大，遍历时逆过来即可）
  enum Dir {
    Right(i32, i32, i32),
    Down(i32, i32, i32),
    Left(i32, i32, i32),
    Up(i32, i32, i32)
  }

  let mut dirs = [
    Dir::Right(0, 0, cols),
    Dir::Down(cols - 1, 1, rows),
    Dir::Left(rows - 1, 0, cols - 1),
    Dir::Up(0, 1, rows - 1)
  ];

  loop {
    for dir in &mut dirs {
      // 标识当前方向是否为空，为空后退出
      let mut is_dir_empty = true;

      // 打印一个元素
      let mut print_one = |row: i32, col: i32| {
        is_dir_empty = false;
        println!("{}", mat[row as usize][col as usize]);
      };

      // 根据当前遍历的方向的不同，进行遍历，并缩小他们的范围
      match *dir {
        Dir::Right(row, l_idx, r_idx) => {
          if row < rows {
            for col in l_idx..r_idx {
              print_one(row, col);
            }

            *dir = Dir::Right(row + 1, l_idx + 1, r_idx - 1);
          }
        }
        Dir::Down(col, u_idx, d_idx) => {
          if col >= 0 {
            for row in u_idx..d_idx {
              print_one(row, col);
            }

            *dir = Dir::Down(col - 1, u_idx + 1, d_idx - 1);
          }
        }
        Dir::Left(row, l_idx, r_idx) => {
          if row >= 0 {
            for col in (l_idx..r_idx).rev() {
              print_one(row, col);
            }

            *dir = Dir::Left(row - 1, l_idx + 1, r_idx - 1);
          }
        }
        Dir::Up(col, u_idx, d_idx) => {
          if col < cols {
            for row in (u_idx..d_idx).rev() {
              print_one(row, col);
            }

            *dir = Dir::Up(col + 1, u_idx + 1, d_idx - 1);
          }
        }
      }

      // 若该方向没有可以打印的内容，说明遍历结束
      if is_dir_empty {
        return;
      }
    }
  }
}


fn main() {
  println!("case 1:");
  clockwise_print_matrix(&vec![
    vec![1, 2, 3],
    vec![4, 5, 6]
  ]);

  println!("case 2:");
  clockwise_print_matrix(&vec![
    vec![1, 2],
    vec![3, 4],
    vec![5, 6],
  ]);

  println!("case 3:");
  clockwise_print_matrix(&vec![
    vec![1],
  ]);

  println!("case 4:");
  clockwise_print_matrix(&vec![
    vec![1,  2,  3,  4,  5],
    vec![6,  7,  8,  9,  10],
    vec![11, 12, 13, 14, 15],
    vec![16, 17, 18, 19, 20],
    vec![21, 22, 23, 24, 25],
  ]);
}
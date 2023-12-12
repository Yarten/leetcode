struct Solution {}

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        // 使用两层结构，表示 5 x n 的数组里的上一行、与当前行
        let mut two_levels_counts = [1; 10];

        // 将数组分成前后两组，在迭代中，交替地表示上一行与当前行
        let mut last_level_begin_idx = 0;
        let mut curr_level_begin_idx = 5;

        // 从第 2 行开始遍历至第 n 行，求每一行、也即取不同数量字符时，以各个元音开头时的排列组合数
        for ni in (1..n) {
            // 5 个元音以 0, 1, 2, 3, 4 数字表示，本行的某个元音开头时可取的排列组合，
            // 为上一行（也即字符数量少一时），以它或它之后的元音开头时、可取的排列组合之和
            for i in (0..5) {
                two_levels_counts[curr_level_begin_idx + i] =
                    two_levels_counts[last_level_begin_idx + i..last_level_begin_idx + 5].iter().sum();
            }

            // 进入下一行的处理
            curr_level_begin_idx = last_level_begin_idx;
            last_level_begin_idx = 5 - curr_level_begin_idx;
        }

        // 将上一行，也即第 n 行的各个元音排头时的排列组合数求和，即为本题答案
        return two_levels_counts[last_level_begin_idx..last_level_begin_idx + 5].iter().sum();
    }
}

fn main() {
    println!("{}", Solution::count_vowel_strings(33));
}
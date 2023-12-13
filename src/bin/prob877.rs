struct Solution {}

struct State {
    l_idx: usize,
    r_idx: usize,
    a_score: i32,
    b_score: i32,
    valid: bool
}

struct Node {
    states: [State; 4],
    idx: usize
}

enum Direction {
    Left, Right
}

impl State {
    fn new_initial(piles: &Vec<i32>) -> Self {
        State {
            l_idx: 0,
            r_idx: piles.len() - 1,
            a_score: 0,
            b_score: 0,
            valid: false
        }
    }

    fn new_alice(&self, piles: &Vec<i32>, dir: Direction) -> Self {
        match dir {
            Direction::Left => State{
                l_idx: self.l_idx + 1,
                r_idx: self.r_idx,
                a_score: self.a_score + piles[self.l_idx],
                b_score: self.b_score,
                valid: false
            },
            Direction::Right => State{
                l_idx: self.l_idx,
                r_idx: self.r_idx - 1,
                a_score: self.a_score + piles[self.r_idx],
                b_score: self.b_score,
                valid: false
            }
        }
    }

    fn new_bob(&self, piles: &Vec<i32>, dir: Direction) -> Self {
        match dir {
            Direction::Left => State{
                l_idx: self.l_idx + 1,
                r_idx: self.r_idx,
                a_score: self.a_score,
                b_score: self.b_score + piles[self.l_idx],
                valid: false
            },
            Direction::Right => State{
                l_idx: self.l_idx,
                r_idx: self.r_idx - 1,
                a_score: self.a_score,
                b_score: self.b_score + piles[self.r_idx],
                valid: false
            }
        }
    }
}

impl Node {
    fn new(piles: &Vec<i32>, curr_alice: &State) -> Self {
        let l_bob = curr_alice.new_bob(piles, Direction::Left);
        let r_bob = curr_alice.new_bob(piles, Direction::Right);

        Node {
            states: [
                l_bob.new_alice(piles, Direction::Left),
                l_bob.new_alice(piles, Direction::Right),
                r_bob.new_alice(piles, Direction::Left),
                r_bob.new_alice(piles, Direction::Right)
            ],
            idx: 0
        }
    }

    fn set_and_next(&mut self, valid: bool) {
        self.states[self.idx].valid = valid;
        self.idx = (self.idx / 2 + 1) * 2;
    }
}

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        // 处理只有两块石头的情况
        if piles.len() == 2 {
            return true;
        }

        // 初始化没有选石头的时候的状态
        let root_state = State::new_initial(&piles);

        // 尝试先取左、或者取右边的石头，只要有一边存在必赢的情况即可
        Solution::test_first_choice(&piles, root_state.new_alice(&piles, Direction::Left))
            ||
        Solution::test_first_choice(&piles, root_state.new_alice(&piles, Direction::Right))
    }

    fn test_first_choice(piles: &Vec<i32>, start_state: State) -> bool {
        // 求和石头总数的一半（若哪一方的石头数量大于该树，后续可以停止搜索，因为已经决出胜负）
        let half_sum: i32 = piles.iter().sum::<i32>() / 2;

        // 使用深度优先搜索
        let mut stack = Vec::with_capacity(piles.len());

        // 压入第一个节点
        stack.push(Node::new(piles, &start_state));

        'dfs: loop {
            let curr_node = stack.last_mut().unwrap();

            // 本节点还未扩展完毕，继续从当前遍历的状态、往后推演
            if curr_node.idx < 4 {
                let curr_state = & curr_node.states[curr_node.idx];

                // 根据当前所有索引是否指向同一个石头，来判断当前是否遍历到了最后一层
                if curr_state.l_idx != curr_state.r_idx {
                    // 若后续还有节点，则基于它扩展，继续往后搜索。
                    // 如果当前状态可以断言某一方必赢（石头数量超过半数），则可不必向后搜索
                    if curr_state.a_score < half_sum && curr_state.b_score < half_sum  {
                        stack.push(Node::new(piles, curr_state));
                    }
                    else {
                        curr_node.set_and_next(curr_state.a_score > half_sum);
                    }

                    continue 'dfs;
                }
                else {
                    // 若后续已经没有节点了，那当前节点必然是刚被创建出来的
                    assert_eq!(curr_node.idx, 0);

                    //
                }
            }

            // 根据本节点的分析结果，更新上一个节点、对应的、衍生本节点的那个状态的有效情况，
            // 并弹出本节点。


            // 如果本节点是第一个节点，则
        }


        return start_state.valid;
    }
}

fn main() {

}
#[test]
pub fn test() {
    let mut dag = Graphadj::new(5);
    dag.add_edge(0, 1);
    dag.add_edge(0, 3);
    dag.add_edge(1, 3);
    dag.add_edge(1, 2);
    dag.add_edge(2, 4);
    dag.add_edge(3, 2);
    dag.add_edge(3, 4);

    println!("是否成功:{}", dag.topological_sort());
}

/// 有序图-邻接表
pub struct Graphadj {
    v: usize,
    adj: Vec<Vec<usize>>,
    q: Vec<usize>,
    indegree: Vec<usize>,
}

impl Graphadj {
    pub fn new(v: usize) -> Self {
        let mut dag = Self {
            v,
            adj: Vec::new(),
            q: Vec::new(),
            indegree: Vec::new(),
        };
        for _ in 0..v {
            dag.adj.push(Vec::new());
            dag.indegree.push(0);
        }
        dag
    }

    /// 添加路径-双向添加则为无序图
    fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.indegree[w] = self.indegree[w] + 1;
        // 双向添加则为无序图
        // self.adj[w].push(v);
        // self.indegree[v] = self.indegree[v] + 1;
    }

    /// 拓扑排序
    fn topological_sort(mut self) -> bool {
        for (i, v) in self.indegree.iter().enumerate() {
            if *v == 0 {
                self.q.push(i);
            }
        }
        let mut count = 0;
        while self.q.len() != 0 {
            let v: usize = self.q[0];
            self.q.remove(0);
            println!("{}", &v);
            count = count + 1;
            for a in &self.adj[v] {
                if self.indegree[*a] > 0 {
                    self.indegree[*a] = self.indegree[*a] - 1;
                }
                if self.indegree[*a] == 0 {
                    self.q.push(*a);
                }
            }
        }

        if count < self.v {
            return false;
        }
        true
    }
}

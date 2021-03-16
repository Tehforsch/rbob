use crate::array_utils::FArray1;

static LEAF_MAX_NUM_POINTS: usize = 1;
static NUM_DIM: usize = 3;

pub enum KdTree {
    Tree(KdTreeNode),
    Leaf(Vec<FArray1>),
}

pub struct KdTreeNode {
    split_1: Box<KdTree>,
    split_2: Box<KdTree>,
    split_axis: usize,
    split_pos: f64,
}

impl KdTree {
    pub fn new(points: Vec<FArray1>) -> KdTree {
        assert!(points.len() > 0);
        let first_split_axis = 0;
        KdTree::construct(points, first_split_axis)
    }

    pub fn construct(points: Vec<FArray1>, split_axis: usize) -> KdTree {
        match points.len() < LEAF_MAX_NUM_POINTS {
            true => KdTree::Leaf(points),
            false => {
                let (split_1, split_2, split_pos) = split_along(points, split_axis);
                let node = KdTreeNode {
                    split_1: Box::new(split_1),
                    split_2: Box::new(split_2),
                    split_pos,
                    split_axis,
                };
                KdTree::Tree(node)
            }
        }
    }

    pub fn nearest_neighbour(&self, point: FArray1) -> FArray1 {
        todo!()
    }
}

fn split_along(points: Vec<FArray1>, split_axis: usize) -> (KdTree, KdTree, f64) {
    // points.sort_by_key(|x| OrderedFloat::new(x[split_axis]));
    let mid = points.len() / 2;
    let split = points.split_at(mid);
    let pos = points[mid][split_axis];
    let next_split_axis = (split_axis + 1).rem_euclid(NUM_DIM);
    (
        KdTree::construct(split.0.to_vec(), next_split_axis),
        KdTree::construct(split.1.to_vec(), next_split_axis),
        pos,
    )
}

#[cfg(test)]
mod tests {
    use ndarray::array;
    use ndarray::s;

    #[test]
    pub fn kdtree() {
        let coords = vec![
            (0, 0, array![2., 1., 1.]),
            (0, 1, array![2., 1., 2.]),
            (0, 2, array![2., 1., 3.]),
            (1, 0, array![2., 2., 1.]),
            (1, 1, array![2., 2., 2.]),
            (1, 2, array![2., 2., 3.]),
            (2, 0, array![2., 3., 1.]),
            (2, 1, array![2., 3., 2.]),
            (2, 2, array![2., 3., 3.]),
        ];
        dbg!(KdTree::new(coords));
        assert!(false);
    }
}

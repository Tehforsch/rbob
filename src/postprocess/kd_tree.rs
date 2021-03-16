use crate::array_utils::FArray1;
use ordered_float::OrderedFloat;

static LEAF_MAX_NUM_POINTS: usize = 10;
static NUM_DIM: usize = 3;
static MAX_DEPTH: usize = 15;

#[derive(Debug)]
pub enum KdTree {
    Tree(KdTreeNode),
    Leaf(Vec<(usize, FArray1)>),
}

#[derive(Debug)]
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
        let points = points.iter().enumerate().map(|(i, x)| (i, x.clone())).collect();
        KdTree::construct(points, first_split_axis, 0)
    }

    pub fn construct(points: Vec<(usize, FArray1)>, split_axis: usize, depth: usize) -> KdTree {
        if points.len() <= LEAF_MAX_NUM_POINTS || depth > MAX_DEPTH {
            KdTree::Leaf(points)
        } else {
            let (split_1, split_2, split_pos) = split_along(points, split_axis, depth);
            let node = KdTreeNode {
                split_1: Box::new(split_1),
                split_2: Box::new(split_2),
                split_pos,
                split_axis,
            };
            KdTree::Tree(node)
        }
    }

    pub fn nearest_neighbour(&self, point: &FArray1) -> &(usize, FArray1) {
        self.nearest_neighbour_traverse(point, 0)
    }

    pub fn nearest_neighbour_traverse(&self, point: &FArray1, split_axis: usize) -> &(usize, FArray1) {
        match self {
            KdTree::Tree(node) => match point[split_axis] < node.split_pos {
                true => node
                    .split_1
                    .nearest_neighbour_traverse(point, next_axis(split_axis)),
                false => node
                    .split_2
                    .nearest_neighbour_traverse(point, next_axis(split_axis)),
            },
            KdTree::Leaf(list) => nearest_neighbour_from_list(&point, list),
        }
    }
}

fn nearest_neighbour_from_list<'a>(point: &FArray1, list: &'a Vec<(usize, FArray1)>) -> &'a (usize, FArray1) {
    assert!(list.len() > 0);
    list.iter()
        .min_by_key(|(i, p1)| {
            OrderedFloat(squared_distance(p1, point))
        })
        .unwrap()
}

fn squared_distance(p1: &FArray1, p2: &FArray1) -> f64 {
    (p1 - p2).dot(&(p1 - p2))
}

fn split_along(mut points: Vec<(usize, FArray1)>, split_axis: usize, depth: usize) -> (KdTree, KdTree, f64) {
    points.sort_by_key(|x| OrderedFloat(x.1[split_axis]));
    let (index, pos) = find_split_index_and_pos(&points, split_axis);
    let split = points.split_at(index);
    (
        KdTree::construct(split.0.to_vec(), next_axis(split_axis), depth + 1),
        KdTree::construct(split.1.to_vec(), next_axis(split_axis), depth + 1),
        pos,
    )
}

fn find_split_index_and_pos(points: &Vec<(usize, FArray1)>, split_axis: usize) -> (usize, f64) {
    let mid = points.len() / 2;
    let pos = points[mid].1[split_axis];
    let split_index = points.iter().enumerate().filter(|(_, point)| point.1[split_axis] == pos).next().unwrap().0;
    (split_index, pos)
}

fn next_axis(axis: usize) -> usize {
    (axis + 1).rem_euclid(NUM_DIM)
}

#[cfg(test)]
mod tests {
    use crate::postprocess::kd_tree::nearest_neighbour_from_list;
    use crate::postprocess::kd_tree::KdTree;
    use ndarray::array;

    #[test]
    pub fn kdtree() {
        let coords = vec![
            (array![1., 1., 1.]),
            (array![1., 1., 2.]),
            (array![1., 2., 1.]),
            (array![1., 2., 2.]),
            (array![2., 1., 1.]),
            (array![2., 1., 2.]),
            (array![2., 2., 1.]),
            (array![2., 9., 2.]),
            (array![2., 2., 9.]),
            (array![2., 2., 5.]),
            (array![2., 2., 5.]),
            (array![2., 2., 5.]),
            (array![2., 2., 5.]),
            (array![2., 2., 5.]),
            (array![2., 2., 5.]),
            (array![2., 2., 5.]),
            (array![2., 5., 2.]),
            (array![5., 2., 2.]),
        ];
        let tree = KdTree::new(coords.clone());
        for coord in coords.iter() {
            assert_eq!(
                &tree.nearest_neighbour(coord + &array![0.1, 0.1, 0.1]).1,
                coord
            );
        }
    }
}

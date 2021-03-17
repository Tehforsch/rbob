use crate::array_utils::FArray1;
use ordered_float::OrderedFloat;

static LEAF_MAX_NUM_POINTS: usize = 10;
static NUM_DIM: usize = 3;
static MAX_DEPTH: usize = 15;

#[derive(Debug)]
pub enum KdTree<'a> {
    Tree(KdTreeNode<'a>),
    Leaf(&'a Vec<FArray1>, Vec<usize>),
}

#[derive(Debug)]
pub struct KdTreeNode<'a> {
    split_1: Box<KdTree<'a>>,
    split_2: Box<KdTree<'a>>,
    points: &'a Vec<FArray1>,
    split_axis: usize,
    split_pos: f64,
}

impl<'a> KdTree<'a> {
    pub fn new(points: &'a Vec<FArray1>) -> KdTree<'a> {
        assert!(points.len() > 0);
        let first_split_axis = 0;
        let indices = points.iter().enumerate().map(|(i, _)| i).collect();
        KdTree::construct(points, indices, first_split_axis, 0)
    }

    pub fn construct(points: &'a Vec<FArray1>, indices: Vec<usize>, split_axis: usize, depth: usize) -> KdTree<'a> {
        if indices.len() <= LEAF_MAX_NUM_POINTS || depth > MAX_DEPTH {
            KdTree::Leaf(points, indices)
        } else {
            let (split_1, split_2, split_pos) = split_along(points, indices, split_axis, depth);
            let node = KdTreeNode {
                split_1: Box::new(split_1),
                split_2: Box::new(split_2),
                points,
                split_pos,
                split_axis,
            };
            KdTree::Tree(node)
        }
    }

    pub fn get_point(&self, index: usize) -> &FArray1 {
        match self {
            KdTree::Tree(node) => {&node.points[index]}
            KdTree::Leaf(points, _) => {&points[index]}
        }
    }

    pub fn nearest_neighbour(&self, point: &FArray1) -> &FArray1 {
        self.get_point(self.nearest_neighbour_traverse(point, 0))
    }

    pub fn nearest_neighbour_index(&self, point: &FArray1) -> usize {
        self.nearest_neighbour_traverse(point, 0)
    }

    pub fn nearest_neighbour_traverse(&self, point: &FArray1, split_axis: usize) -> usize {
        match self {
            KdTree::Tree(node) => match point[split_axis] < node.split_pos {
                true => node
                    .split_1
                    .nearest_neighbour_traverse(point, next_axis(split_axis)),
                false => node
                    .split_2
                    .nearest_neighbour_traverse(point, next_axis(split_axis)),
            },
            KdTree::Leaf(points, list) => nearest_neighbour_from_list(points, list, point),
        }
    }
}

fn nearest_neighbour_from_list<'a>(points: &'a Vec<FArray1>, indices: &Vec<usize>, point: &FArray1 ) -> usize {
    assert!(indices.len() > 0);
    *indices.iter()
        .min_by_key(|index| {
            OrderedFloat(squared_distance(&points[**index], point))
        })
        .unwrap()
}

fn squared_distance(p1: &FArray1, p2: &FArray1) -> f64 {
    (p1 - p2).dot(&(p1 - p2))
}

fn split_along(points: &Vec<FArray1>, mut indices: Vec<usize>, split_axis: usize, depth: usize) -> (KdTree, KdTree, f64) {
    indices.sort_by_key(|i| OrderedFloat(points[*i][split_axis]));
    dbg!(&indices);
    let (index, pos) = find_split_index_and_pos(points, &indices, split_axis);
    let split = indices.split_at(index);
    (
        KdTree::construct(points, split.0.to_vec(), next_axis(split_axis), depth + 1),
        KdTree::construct(points, split.1.to_vec(), next_axis(split_axis), depth + 1),
        pos,
    )
}

fn find_split_index_and_pos(points: &Vec<FArray1>, indices: &Vec<usize>, split_axis: usize) -> (usize, f64) {
    assert!(indices.len() > 1);
    let mid = indices.len() / 2;
    let pos = points[indices[mid]][split_axis];
    let split_index = indices.iter().enumerate().filter(|(i, index)| points[**index][split_axis] == pos).next().unwrap().0;
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
        let tree = KdTree::new(&coords);
        for (i, coord) in coords.iter().enumerate() {
            assert_eq!(
                tree.nearest_neighbour(&(coord + &array![0.1, 0.1, 0.1])),
                coord
            );
        }
    }
}

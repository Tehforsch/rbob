pub struct Selection<T> {
    items: Vec<T>,
    selection: Vec<usize>,
}

fn get_index_of<T>(items: &[T], item: &T) -> Option<usize>
where
    T: PartialEq,
{
    items
        .iter()
        .enumerate()
        .find(|(_, x)| *x == item)
        .map(|x| x.0)
}

impl<T> Selection<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            selection: vec![],
        }
    }
    pub fn get_selected(&self) -> impl Iterator<Item = &T> {
        self.selection.iter().map(|index| &self.items[*index])
    }

    pub fn add_or_remove_from_selection(&mut self, item_index: usize) {
        let index = get_index_of(&self.selection, &item_index);
        if let Some(index) = index {
            self.selection.remove(index);
        } else {
            self.selection.push(item_index);
        }
    }

    pub fn contains(&self, index: usize) -> bool {
        self.selection.contains(&index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn num_selected(&self) -> usize {
        self.selection.len()
    }
}

impl<T> FromIterator<T> for Selection<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
            selection: vec![],
        }
    }
}

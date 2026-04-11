use kdtree::ErrorKind;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

pub struct VectorSpace {
    dimensions: usize,
    tree: KdTree<f64, String, &'static [f64]>,
}

impl VectorSpace {
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensions,
            tree: KdTree::new(dimensions),
        }
    }

    pub fn add(&mut self, id: String, vector: Vec<f64>) -> Result<(), ErrorKind> {
        if vector.len() != self.dimensions {
            return Ok(());
        }

        let vec_slice = vector.into_boxed_slice();
        let vec_ref: &'static [f64] = Box::leak(vec_slice);

        self.tree.add(vec_ref, id)
    }

    pub fn search(&self, query: &[f64], k: usize) -> Result<Vec<(f64, String)>, ErrorKind> {
        let results = self.tree.nearest(query, k, &squared_euclidean)?;
        Ok(results
            .into_iter()
            .map(|(dist, id)| (dist, id.clone()))
            .collect())
    }
}

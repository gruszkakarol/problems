use super::graph;

pub fn select_vertex<'v, T: PartialEq + Clone + std::fmt::Debug>(
    (left, right): &'v (T, T),
    vertex: &T,
    direction: &graph::LinkDirection,
) -> Option<T> {
    let selected = match (direction, left == vertex, right == vertex) {
        (graph::LinkDirection::Direct, true, _) => Some(right),
        (graph::LinkDirection::Indirect, _, true) => Some(left),
        (graph::LinkDirection::All, is_left, is_right) => {
            if is_left {
                Some(right)
            } else if is_right {
                Some(left)
            } else {
                None
            }
        }
        _ => None,
    };
    selected.map(|s| s.clone())
}

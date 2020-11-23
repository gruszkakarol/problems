use super::utils::select_vertex;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum LinkDirection {
    Direct,
    Indirect,
    All,
}

pub trait Graph<T> {
    /// Gets all of the direct and indirect vertices that are connected to the supplied vertice.
    fn get_edges(&self, vertice: T, direction: LinkDirection) -> Vec<T>;
    /// Calculates checksum by counting all of the direct and indirect connections in the graph.
    fn calculate_checksum(&self, vertex: T, offset: usize) -> usize;
    fn checksum(&self) -> usize;
    fn distance(&self, from: T, to: T) -> Option<usize>;
}

pub struct Objects<'a> {
    vertices: Vec<(&'a str, &'a str)>,
}

impl<'a> From<Vec<(&'a str, &'a str)>> for Objects<'a> {
    fn from(vertices: Vec<(&'a str, &'a str)>) -> Self {
        Self { vertices }
    }
}

impl<'a> Graph<&'a str> for Objects<'a> {
    fn get_edges(&self, vertex: &'a str, direction: LinkDirection) -> Vec<&'a str> {
        self.vertices
            .iter()
            .map(|adjacent| select_vertex(adjacent, &vertex, &direction))
            .flatten()
            .collect()
    }

    fn calculate_checksum(&self, vertex: &'a str, offset: usize) -> usize {
        self.get_edges(vertex, LinkDirection::Direct)
            .iter()
            .map(|v| self.calculate_checksum(v, offset + 1))
            .sum::<usize>()
            + offset
    }

    fn checksum(&self) -> usize {
        self.calculate_checksum("COM", 0)
    }

    fn distance(&self, from: &str, to: &str) -> Option<usize> {
        let mut distance: usize = 0;
        let mut queue: VecDeque<&str> = vec![from].into();
        let mut visited_edges: Vec<&str> = vec![from];
        let mut new_edges: Vec<&str> = vec![];

        while let Some(vertex) = queue.pop_back() {
            let edges = self.get_edges(vertex, LinkDirection::All);
            if edges.contains(&to) {
                return Some(distance);
            } else {
                new_edges.extend(edges.iter().filter(|e| !&visited_edges.contains(&e)));
                visited_edges.extend(edges);
            }

            if queue.is_empty() {
                queue.extend(&new_edges.drain(0..).collect::<Vec<&str>>());
                distance += 1;
            }
        }
        None
    }
}

mod test {
    use super::*;

    /// Method returns correct number of edges for each of the vertices in the graph
    #[test]
    fn get_edges() {
        let vertices: Vec<(&str, &str)> = vec![
            ("COM", "BDF"),
            ("BDF", "YOU"),
            ("BDF", "TST"),
            ("TST", "SAN"),
        ];
        let objects = Objects::from(vertices);

        let empty: Vec<&str> = vec![];
        // Direct edges
        assert_eq!(objects.get_edges("COM", LinkDirection::Direct), vec!["BDF"]);
        assert_eq!(
            objects.get_edges("BDF", LinkDirection::Direct),
            vec!["YOU", "TST"]
        );
        assert_eq!(objects.get_edges("TST", LinkDirection::Direct), vec!["SAN"]);

        // Indirect edges
        assert_eq!(objects.get_edges("COM", LinkDirection::Indirect), empty);
        assert_eq!(
            objects.get_edges("BDF", LinkDirection::Indirect),
            vec!["COM"]
        );
        assert_eq!(
            objects.get_edges("TST", LinkDirection::Indirect),
            vec!["BDF"]
        );

        // All edges
        assert_eq!(objects.get_edges("COM", LinkDirection::All), vec!["BDF"]);
        assert_eq!(
            objects.get_edges("BDF", LinkDirection::All),
            vec!["COM", "YOU", "TST"]
        );
        assert_eq!(objects.get_edges("TST", LinkDirection::Direct), vec!["SAN"]);
    }

    #[test]
    fn calculate_checksum() {
        let vertices: Vec<(&str, &str)> = vec![
            ("COM", "BDF"),
            ("BDF", "YOU"),
            ("BDF", "TST"),
            ("TST", "SAN"),
        ];
        let objects = Objects::from(vertices);

        assert_eq!(objects.calculate_checksum("COM", 0), 8);
        assert_eq!(objects.calculate_checksum("BDF", 1), 8);
        assert_eq!(objects.checksum(), 8);
    }

    #[test]
    fn distance() {
        let vertices: Vec<(&str, &str)> = vec![
            ("COM", "BDF"),
            ("BDF", "YOU"),
            ("BDF", "TST"),
            ("TST", "SAN"),
        ];
        let objects = Objects::from(vertices);

        assert_eq!(objects.distance("COM", "BDF"), Some(0));
        assert_eq!(objects.distance("BDF", "COM"), Some(0));
        assert_eq!(objects.distance("COM", "TST"), Some(1));
        assert_eq!(objects.distance("TST", "COM"), Some(1));
        assert_eq!(objects.distance("COM", "SAN"), Some(2));
        assert_eq!(objects.distance("YOU", "SAN"), Some(2));
        assert_eq!(objects.distance("COM", "NONE"), None);
    }
}

pub mod distance;
pub mod edit;
pub mod util;

pub use distance::*;
pub use edit::*;
use util::DistanceMatrix;

/// Computes and returns the Levenshtein distance between the source and target sequences.
///
/// # Arguments
///
/// * `source` - The source sequence
/// * `target` - The target sequence
///
/// # Examples
///
/// ```
/// use levenshtein_diff as levenshtein;
///
/// let s1 = "FLAW";
/// let s2 = "LAWN";
///
/// let (distance, _) = levenshtein::distance(s1, s2);
/// assert_eq!(distance, 2);
///
/// let v1 = vec![0, 1, 2];
/// let v2 = vec![1, 2, 3, 4];
///
/// let (distance, _) = levenshtein::distance(v1, v2); // Also works on vectors
/// assert_eq!(distance, 3);
/// ```
pub fn distance<T, U>(source: U, target: U) -> (usize, DistanceMatrix)
where
    T: Eq,
    // U is any type that can be trivially converted into a reference to a slice of type T
    // i.e. &[T]
    U: AsRef<[T]>,
{
    levenshtein_memoization(source, target)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn default_distance_test() {
        let s1 = "FLOWER";
        let s2 = "FOLLOWER";

        let expected_dist = 2;

        let (dist, _) = distance(s1, s2);

        assert_eq!(expected_dist, dist);
    }
}
use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Square {
    Wall,
    Either,
    Letter,
    Filled(char),
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Line<const N: usize>([Square; N]);
struct LineIter<'a, const N: usize>(&'a [Square; N], usize);

impl<'a, const N: usize> Iterator for LineIter<'a, N> {
    type Item = &'a Square;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.0.get(self.1);
        self.1 = self.1 + 1;
        out
    }
}

impl<const N: usize> Line<N> {
    fn iter(&self) -> LineIter<'_, N> {
        LineIter::<N>(&self.0, 0)
    }

    // TODO this makes me sad, rewrite as ierators
    fn contains_short_word(&self) -> bool {
        fn contains_short_word(&self) -> bool {
            let chunks = self.iter().chunk_by(|&&x| x == Square::Wall);
            for (wall, mut chunk) in chunks.into_iter() {
                if !wall && chunk.all(|x| matches!(x, Square::Filled(_))) {
                    return true;
                }
            }
            false
        }
    }
}

impl<const N: usize> std::default::Default for Line<N> {
    fn default() -> Self { Line([Square::Wall; N]) }
}
    
impl<const N: usize> std::ops::Index<usize> for Line<N>  {
    type Output = Square;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Line<N>  {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


struct Grid<const N: usize> {
    lines: [Line<N>; N]
}

struct GridIter<'a, const N: usize>(&'a [Line<N>; N], usize, usize);

impl<'a, const N: usize> Iterator for GridIter<'a, N> {
    type Item = &'a Square;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.0.get(self.1)
            .map(|x| x.0.get(self.2));
        if self.2 == N {
            self.2 = 0;
            self.1 = self.1 + 1;            
        } else { self.2 = self.2 + 1; }
        out.flatten()
    }
}

impl<const N: usize> std::default::Default for Grid<N> {
    fn default() -> Self { Grid{ lines: [Default::default(); N] } }
}

impl<const N: usize> std::ops::Index<usize> for Grid<N>  {
    type Output = Line<N>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.lines[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Grid<N>  {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.lines[index]
    }
}

impl<const N: usize> Grid<N> {
    fn iter(&self) -> GridIter<'_, N> {
        GridIter::<N>(&self.lines, 0, 0)
    }
    
    fn filled(&self) -> bool {
        self.iter()
            .filter(|&&x| x == Square::Either || x == Square::Letter)
            .next().is_some()
    }

    fn count_filled(&self) -> usize {
        self.iter()
            .filter(|x| match x { Square::Filled(_) => true, _ => false })
            .count()
    }

    fn transpose(&self) -> Grid<N> {
        let mut out: Grid<N> = Default::default();
        for (i, j) in (0..N).cartesian_product(0..N) {
            out[j][i] = self[i][j];
        }
        out
    }

    fn contains_short_word(&self) -> bool {
        true
    }
}

fn main() {
    println!("Hello, world!");
}

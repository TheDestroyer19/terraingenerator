pub(crate) struct Map<T> {
    size: usize,
    values: Vec<T>,
}

impl<T> Map<T>
where
    T: Default,
{
    pub(crate) fn new(size: usize) -> Self {
        Self::with_generator(size, |_| Default::default())
    }
}

impl<T> Map<T> {
    pub(crate) fn with_generator(size: usize, mut gen: impl FnMut(Pos) -> T) -> Self {
        let mut values = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                values.push(gen((x, y).into()));
            }
        }

        Self { size, values }
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }

    pub(crate) fn get(&self, pos: impl Into<Pos>) -> &T {
        let pos = pos.into();
        let idx = self.idx(pos);
        &self.values[idx]
    }

    pub(crate) fn set(&mut self, pos: impl Into<Pos>, v: T) {
        let pos = pos.into();
        let idx = self.idx(pos);
        self.values[idx] = v;
    }

    pub(crate) fn map(&mut self, mut mapper: impl FnMut(Pos, &T) -> T) {
        self.values.iter_mut().enumerate().for_each(|(idx, v)| {
            let pos = idx_pos(self.size, idx);
            *v = mapper(pos, v);
        });
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    fn idx(&self, pos: Pos) -> usize {
        pos.y * self.size + pos.x
    }
}

fn idx_pos(size: usize, idx: usize) -> Pos {
    let x = idx % size;
    let y = idx / size;
    Pos::new(x, y)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub(crate) struct Pos {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Pos {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self {
            x, y
        }
    }
    pub(crate) fn neighbors(self, max_x: usize, max_y: usize) -> Vec<Pos> {
        let Pos {x, y} = self;
        let mut results = Vec::with_capacity(4);
        if x > 0 { results.push((x-1,y).into()); }
        if x + 1 < max_x { results.push((x+1,y).into()); }
        if y > 0 { results.push((x, y-1).into());}
        if y + 1 < max_y {results.push((x,y+1).into()); }

        results
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}
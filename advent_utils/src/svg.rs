use std::ops::RangeInclusive;

use glam::I64Vec2;
use itertools::Itertools;

struct Viewbox<V> {
    xs: RangeInclusive<V>,
    ys: RangeInclusive<V>,
}

impl<V> Viewbox<V>
where
    V: Copy + std::ops::Sub<V, Output = V>,
{
    fn width(&self) -> V {
        *self.xs.end() - *self.xs.start()
    }
    fn heigth(&self) -> V {
        *self.ys.end() - *self.ys.start()
    }
}

impl<V> Default for Viewbox<V>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            xs: V::default()..=V::default(),
            ys: V::default()..=V::default(),
        }
    }
}

impl<V> std::fmt::Debug for Viewbox<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{:?} {:?} {:?} {:?}\"",
            self.xs.start(),
            self.ys.start(),
            self.xs.end(),
            self.ys.end()
        )
    }
}

impl<T: Iterator<Item = I64Vec2> + Clone> From<T> for Viewbox<i64> {
    fn from(value: T) -> Self {
        let xs = value
            .clone()
            .map(|p| p.x)
            .minmax()
            .into_option()
            .map(|(min, max)| min..=max)
            .unwrap_or(0..=0);
        let ys = value
            .clone()
            .map(|p| p.y)
            .minmax()
            .into_option()
            .map(|(min, max)| min..=max)
            .unwrap_or(0..=0);
        Self { xs, ys }
    }
}

pub fn print_svg_from_path(path: &[I64Vec2]) {
    let mut s = String::new();

    write_svg_from_path(&mut s, path).unwrap();
    println!("{s}");
}

pub fn write_svg_from_path(w: &mut impl std::fmt::Write, path: &[I64Vec2]) -> std::fmt::Result {
    let viewbox = Viewbox::from(path.iter().copied());

    write!(
        w,
        "<svg viewBox={:?} width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
        viewbox,
        viewbox.width(),
        viewbox.heigth()
    )?;
    write_svg_path_tag(w, path)?;

    writeln!(w, "\n</svg>")?;
    Ok(())
}
pub fn write_svg_path_tag(w: &mut impl std::fmt::Write, path: &[I64Vec2]) -> std::fmt::Result {
    write!(w, "<path d=\"")?;
    write_svg_path_string(w, path)?;
    write!(w, "\"></path>")?;
    Ok(())
}

pub fn write_svg_path_string(w: &mut impl std::fmt::Write, path: &[I64Vec2]) -> std::fmt::Result {
    let mut it = path.iter();
    if let Some(p) = it.next() {
        write!(w, "M{},{}", p.x, p.y)?;
    }
    for p in it {
        write!(w, "L{},{}", p.x, p.y)?;
    }
    Ok(())
}

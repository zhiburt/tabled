use papergrid;
pub use tabled_derive::Tabled;

pub trait Tabled {
    fn fields(&self) -> Vec<String>;
    fn headers() -> Vec<String>;
}

pub fn table<T, Iter>(iter: Iter) -> String
where
    T: Tabled+Sized,
    Iter: IntoIterator<Item = T>
{
    let headers = T::headers();
    let obj: Vec<Vec<String>> = iter.into_iter().map(|t| t.fields()).collect();

    let mut grid = papergrid::Grid::new(obj.len()+1, headers.len());
    for (i, h) in headers.iter().enumerate() {
        grid.cell(0, i).set_content(h).set_horizontal_ident(1);
    }

    for (i, fields) in obj.iter().enumerate() {
        for (j, field) in fields.iter().enumerate() {
            grid.cell(i+1, j).set_content(field).set_horizontal_ident(1);
        }
    }

    grid.to_string()
}

use crate::*;

pub struct GridViewFormat<'a, T, Idx, const N : usize> where Idx : IntegerIndex
{
    pub view  : GridView<'a,T,Idx,N>,
    pub separator : String,
}

impl<'a, T, Idx, const N : usize> GridViewFormat<'a, T, Idx, N> where Idx : IntegerIndex
{
    pub fn new(view  : GridView<'a,T,Idx,N>) -> Self 
    {
        Self { view, separator: "".to_owned() }
    }
    pub fn with_separator(mut self, separator : String) -> Self 
    {
        self.separator = separator;
        self
    }
}

impl<'a, T, Idx> Display for GridViewFormat<'a, T, Idx, 1> where Idx : IntegerIndex, T : Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult 
    {
        let mut display = Vec::with_capacity(self.view.area().to_usize());
        let mut max_len = 0;
        for (_, value) in self.view.iter()
        {
            let formatted = format!("{}", value);
            max_len = formatted.len().max(max_len);
            display.push(formatted)
        }

        let mut it = self.view.iter().enumerate().peekable();
        while let Some((i, _)) = it.next()
        {
            write!(f, "{:width$}", display[i], width = max_len)?;
            if it.peek().is_some()
            {
                write!(f, "{}", self.separator)?;
            }
        }
        Ok(())
    }
}


impl<'a, T, Idx> Display for GridViewFormat<'a, T, Idx, 2> where Idx : IntegerIndex, T : Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult 
    {
        let mut max_len = 0;
        let formated_grid = self.view.map(|value| 
        {
            let formatted = format!("{}", value);
            max_len = max_len.max(formatted.len());
            formatted
        });

        let rect = self.view.rect().to_usize();
        for y in (0..rect.size_y()).rev()
        {
            for x in 0..rect.size_x()
            {
                write!(f, "{:width$}", formated_grid[vector2(Idx::cast_from(x), Idx::cast_from(y))], width = max_len)?;
                if x != rect.size_x() - 1 
                {
                    write!(f, "{}", self.separator)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
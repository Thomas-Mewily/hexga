use crate::*;

pub struct GridViewFormat<'a, T, Idx, const N : usize, Sep=&'static str> where Idx : Integer, Sep : Display
{
    pub view  : GridView<'a,T,Idx,N>,
    pub separator : Sep,
}

impl<'a, T, Idx, const N : usize, Sep> GridViewFormat<'a, T, Idx, N, Sep> where Idx : Integer, Sep : Display
{
    pub fn new_with_separator(view : GridView<'a,T,Idx,N>, separator : Sep) -> Self 
    {
        Self { view, separator }
    }

    pub fn new(view : GridView<'a,T,Idx,N>) -> Self where Sep : Default
    { Self::new_with_separator(view, ___()) }

    pub fn with_separator<Sep2 : Display>(self, separator : Sep2) -> GridViewFormat<'a, T, Idx, N, Sep2> 
    {
        GridViewFormat { view : self.view, separator }
    }
}

impl<'a, T, Idx, Sep> Display for GridViewFormat<'a, T, Idx, 1, Sep> where Idx : Integer, T : Display, Sep : Display
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


impl<'a, T, Idx, Sep> Display for GridViewFormat<'a, T, Idx, 2, Sep> where Idx : Integer, T : Display, Sep : Display
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
use crate::*;

pub struct IoMediatorDirect<O,E> where O : IoOk, E : IoError
{
    read   : fn(&str, &mut Vec<u8>) -> Result<O,E>,
    write  : fn(&str, &[u8]) -> Result<O,E>,
    remove : fn(&str) -> Result<O,E>,
    append : Option<fn(&str, &[u8]) -> Result<O,E>>,
}

impl Default for IoMediatorDirect<IoDiskOk,IoDiskError> where IoDiskOk : IoOk, IoDiskError : IoError
{
    fn default() -> Self 
    {
        Self 
        { 
            read: |path, buf| Io::disk_read_buf(path, buf), 
            write: |path,data| Io::disk_write(path, data), 
            remove: |path| Io::disk_remove(path),
            append: Some(|path, data| Io::disk_append(path, data))
        }
    }
}

impl<O,E> IoMediator for IoMediatorDirect<O,E> where O : IoOk, E : IoError
{
    type Ok=O;
    type Err=E;

    fn write(&mut self, path : &str, data : &[u8]) -> Result<Self::Ok,Self::Err> 
    { (self.write)(path,data) }

    fn append(&mut self, path : &str, data : &[u8]) -> Result<Self::Ok,Self::Err> 
    { 
        match &self.append
        {
            Some(append) => append(path, data),
            None => 
            {
                let mut file_data = self.read(path)?;
                file_data.extend_from_slice(data);
                self.write(path, &file_data)
            },
        }
    }

    fn remove(&mut self, path : &str) -> Result<Self::Ok,Self::Err> {
        (self.remove)(path)
    }

    fn apply(&mut self) -> Result<Self::Ok,Self::Err> {
        Ok(O::default())
    }
    
    fn read_cache(&mut self, _ : &str) -> Result<Self::Ok,Self::Err> {
        Ok(O::default())
    }
    
    fn read(&mut self, path : &str) -> Result<Vec<u8>,Self::Err> {
        let mut data = Vec::with_capacity(2048);
        (self.read)(path,&mut data)?;
        Ok(data)
    }
}
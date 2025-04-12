use crate::*;

pub struct IoMediatorDirect<Ctx,O,E> where O : IoOk, E : IoError
{
    context : Ctx,
    read   : fn(&path, &mut Vec<u8>, &mut Ctx) -> Result<O,E>,
    write  : fn(&path, &[u8], &mut Ctx) -> Result<O,E>,
    remove : fn(&path, &mut Ctx) -> Result<O,E>,
    append : Option<fn(&path, &[u8], &mut Ctx) -> Result<O,E>>,
}

impl Default for IoMediatorDirect<(),IoDiskOk,IoDiskError> where IoDiskOk : IoOk, IoDiskError : IoError
{
    fn default() -> Self 
    {
        Self 
        { 
            context: (),
            read: |path, buf,_| Io::disk_read_buf(path, buf), 
            write: |path,data,_| Io::disk_write(path, data), 
            remove: |path,_| Io::disk_remove(path),
            append: Some(|path, data,_| Io::disk_append(path, data)),
        }
    }
}

impl IoMediatorDirect<(),IoDiskOk,IoDiskError> where IoDiskOk : IoOk, IoDiskError : IoError
{
    pub fn new_io() -> Self { Self::default() }
}

impl<Ctx,O,E> IoMediator for IoMediatorDirect<Ctx,O,E> where O : IoOk, E : IoError
{
    type Ok=O;
    type Err=E;

    fn write(&mut self, path : &path, data : &[u8]) -> Result<Self::Ok,Self::Err> 
    { (self.write)(path,data,&mut self.context) }

    fn append(&mut self, path : &path, data : &[u8]) -> Result<Self::Ok,Self::Err> 
    { 
        match &self.append
        {
            Some(append) => append(path, data, &mut self.context),
            None => 
            {
                let mut file_data = self.read(path)?;
                file_data.extend_from_slice(data);
                self.write(path, &file_data)
            },
        }
    }

    fn remove(&mut self, path : &path) -> Result<Self::Ok,Self::Err> {
        (self.remove)(path, &mut self.context)
    }

    fn apply(&mut self) -> Result<Self::Ok,Self::Err> {
        Ok(O::default())
    }
    
    fn read_cache(&mut self, _ : &path) -> Result<Self::Ok,Self::Err> {
        Ok(O::default())
    }
    
    fn read(&mut self, path : &path) -> Result<Vec<u8>,Self::Err> {
        let mut data = Vec::with_capacity(2048);
        (self.read)(path,&mut data, &mut self.context)?;
        Ok(data)
    }
}
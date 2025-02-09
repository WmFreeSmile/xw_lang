use std::{fs::File, io::{Read, Seek,SeekFrom}};



//文本流
trait TextStream {
    fn is_symbol(&mut self,symbol:u8)->bool;//是否符号
    fn get_next_char(&mut self)->Option<u8>;//获取下一个字符（不流动）
    fn get_char(&mut self)->Option<u8>;//获取下一个字符（并流动）
}

//文本流（文件）
struct FileTextStream{
    file:std::fs::File,
}
impl FileTextStream {
    fn open(path:String)->Self{
        Self { file: std::fs::File::open(path).unwrap() }
    }
}

impl TextStream for FileTextStream {
    fn is_symbol(&mut self,symbol:u8)->bool {
        match self.get_next_char() {
            Some(s)=>{
                s==symbol
            }
            _=>{false}
        }
    }
    fn get_next_char(&mut self)->Option<u8> {
        let current_pos = self.file.seek(SeekFrom::Current(0)).unwrap();
        let result=self.get_char();
        self.file.seek(SeekFrom::Start(current_pos)).unwrap();
        result
    }
    fn get_char(&mut self)->Option<u8> {
        let mut buf=[0];
        let len=self.file.read(&mut buf).unwrap();
        if len==0{
            None
        }else{
            Some(buf[0])
        }
    }
}


//文本流（内存）
struct MemTextStream{
    text:String,
    pos:usize,//当前位置
    end_pos:usize,
}
impl MemTextStream {
    fn from_string(text:String)->Self{
        Self { text:text.clone(), pos:0, end_pos:text.len() }
    }
}
impl TextStream for MemTextStream {
    fn is_symbol(&mut self,symbol:u8)->bool {
        match self.get_next_char() {
            Some(s)=>{
                s==symbol
            }
            _=>{false}
        }
    }
    fn get_next_char(&mut self)->Option<u8> {
        if self.pos>self.end_pos{
            None
        }else{
            Some(*(&self.text[self.pos..self.pos+1].chars().nth(0).unwrap()) as u8)
        }
    }
    fn get_char(&mut self)->Option<u8> {
        if self.pos>=self.end_pos{
            None
        }else{
            self.pos=self.pos+1;
            Some(*(&self.text[self.pos-1..self.pos].chars().nth(0).unwrap()) as u8)
        }
    }
}


//词
enum Token {
    Symbol,//符号
    Identifier,//标识符
}


//词法流
struct TermStream{
    
}
impl TermStream {
    
}
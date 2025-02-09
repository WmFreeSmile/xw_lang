
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{thread, vec};



pub struct Object{
    marked: bool,
    age: u8, // 对象的存活次数
    references: Vec<*mut Object>,
}
impl Object {
    pub fn new() -> *mut Self {
        Box::into_raw(Box::new(Object {
            marked: false,
            references: Vec::new(),
            age: 0,
        }))
    }

    pub fn add_reference(&mut self, obj: *mut Object) {
        self.references.push(obj);
    }
}


pub struct Vm{
    young_generation: HashSet<*mut Object>,
    old_generation: HashSet<*mut Object>,
    root_set: Vec<*mut Object>,
    thread_contexts: Vec<Arc<Mutex<ThreadContext>>>,
    max_young_age: u8, // 新生代对象晋升到老年代的最大存活次数
    young_threshold: usize, // 新生代对象数量阈值，达到该值触发回收
    old_generation_capacity: usize, // 老年代的容量
}

impl Vm {
    pub fn new(max_young_age: u8, young_threshold: usize,old_generation_capacity: usize) -> Self {
        Self {
            young_generation: HashSet::new(),
            old_generation: HashSet::new(),
            root_set: Vec::new(),
            thread_contexts: Vec::new(),
            max_young_age,
            young_threshold,
            old_generation_capacity,
        }
    }
    
    // 创建新对象并加入到新生代，可能触发新生代垃圾回收和分配担保
    pub fn new_object(&mut self) -> *mut Object {
        if self.young_generation.len() >= self.young_threshold {
            self.collect_young();
            // 回收后仍空间不足，尝试分配担保
            if self.young_generation.len() >= self.young_threshold {
                if self.try_allocation_guarantee() {
                    println!("Allocation guarantee successful.");
                } else {
                    println!("Allocation guarantee failed. Out of memory!");
                    panic!("Out of memory");
                }
            }
        }
        
        let obj = Object::new();
        self.young_generation.insert(obj);

        obj
    }
    
    // 将对象添加到根集合
    pub fn add_to_root_set(&mut self, obj: *mut Object) {
        self.root_set.push(obj);
    }

    // 创建新线程上下文
    fn create_thread_context(&mut self) -> Arc<Mutex<ThreadContext>> {
        let ctx = Arc::new(Mutex::new(ThreadContext::new()));
        self.thread_contexts.push(ctx.clone());
        ctx
    }

    // 标记阶段
    fn mark(&mut self) {
        fn mark_helper(obj: *mut Object) {
            if !unsafe { (*obj).marked } {
                unsafe { (*obj).marked = true };
                for &ref_obj in unsafe { &(*obj).references } {
                    mark_helper(ref_obj);
                }
            }
        }

        // 标记根集合中的对象
        for &obj in &self.root_set {
            mark_helper(obj);
        }

        // 标记所有线程操作数栈中的对象
        for ctx in &self.thread_contexts {
            let guard = ctx.lock().unwrap();
            for &obj in &guard.operand_stack.stack {
                mark_helper(obj);
            }
        }
    }

    // 清除新生代
    fn sweep_young(&mut self) {
        let mut to_remove = Vec::new();
        let mut obj_list =Vec::new();
        
        for &obj in &self.young_generation{
            obj_list.push(obj);
        }
        
        for obj in obj_list{
            if !unsafe { (*obj).marked } {
                to_remove.push(obj);
            } else {
                unsafe {
                    (*obj).marked = false;
                    (*obj).age += 1;
                    if (*obj).age >= self.max_young_age {
                        // 晋升到老年代
                        self.young_generation.remove(&obj);
                        self.old_generation.insert(obj);
                    }
                }
            }
        }
        
        /* 
        for &obj in &self.young_generation {
            if !unsafe { (*obj).marked } {
                to_remove.push(obj);
            } else {
                unsafe {
                    (*obj).marked = false;
                    (*obj).age += 1;
                    if (*obj).age >= self.max_young_age {
                        // 晋升到老年代
                        self.young_generation.remove(&obj);
                        self.old_generation.insert(obj);
                    }
                }
            }
        }*/
        
        for obj in to_remove {
            println!("drop:{:?}",obj);
            self.young_generation.remove(&obj);
            drop(unsafe { Box::from_raw(obj) });
        }
    }

    // 清除老年代
    fn sweep_old(&mut self) {
        let mut to_remove = Vec::new();
        for &obj in &self.old_generation {
            if !unsafe { (*obj).marked } {
                to_remove.push(obj);
            } else {
                unsafe { (*obj).marked = false };
            }
        }

        for obj in to_remove {
            self.old_generation.remove(&obj);
            drop(unsafe { Box::from_raw(obj) });
        }
    }

    // 执行新生代垃圾回收
    fn collect_young(&mut self) {
        self.mark();
        self.sweep_young();
    }

    // 执行老年代垃圾回收
    fn collect_old(&mut self) {
        self.mark();
        self.sweep_old();
    }
    
    // 尝试分配担保
    fn try_allocation_guarantee(&mut self) -> bool {
        let live_objects: Vec<*mut Object> = self.young_generation
           .iter()
           .filter(|&obj| !unsafe { (**obj).marked })
           .cloned()
           .collect();

        let required_space = live_objects.len();
        if self.old_generation.len() + required_space <= self.old_generation_capacity {
            // 老年代空间足够，进行分配担保
            for obj in live_objects {
                self.young_generation.remove(&obj);
                self.old_generation.insert(obj);
            }
            return true;
        }
        return false;
    }
    
    pub fn dump_young(&self){
        for &obj in &self.young_generation{
            println!("{:?}",obj);
        }
    }
    pub fn dump_old(&self){
        for &obj in &self.old_generation{
            println!("{:?}",obj);
        }
    }
}


// 定义操作数栈结构体
struct OperandStack {
    stack: Vec<*mut Object>,
}
impl OperandStack {
    fn new() -> Self {
        OperandStack { stack: Vec::new() }
    }

    fn push(&mut self, obj: *mut Object) {
        self.stack.push(obj);
    }

    fn pop(&mut self) -> Option<*mut Object> {
        self.stack.pop()
    }
}

// 定义线程上下文结构体
struct ThreadContext {
    operand_stack: OperandStack,
}

impl ThreadContext {
    fn new() -> Self {
        ThreadContext {
            operand_stack: OperandStack::new(),
        }
    }
}
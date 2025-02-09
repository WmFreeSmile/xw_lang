

mod vm;

fn main() {
    let mut vm_instance=vm::Vm::new(5,3,10);
    
    let obj1=vm_instance.new_object();vm_instance.add_to_root_set(obj1);println!("obj1:{:?}",obj1);
    let obj2=vm_instance.new_object();unsafe {(*obj1).add_reference(obj2)};println!("obj2:{:?}",obj2);
    let obj3=vm_instance.new_object();unsafe {(*obj1).add_reference(obj3)};println!("obj3:{:?}",obj3);
    
    let obj4=vm_instance.new_object();unsafe {(*obj1).add_reference(obj4)};println!("obj4:{:?}",obj4);//分配担保后，正常分配进新生代
    //新生代空间不足，但所有对象未达到老龄，触发分配担保
    
    let obj5=vm_instance.new_object();println!("obj5:{:?}",obj5);//
    //let obj6=vm_instance.new_object();println!("obj6:{:?}",obj6);
    //let obj7=vm_instance.new_object();println!("obj7:{:?}",obj7);
    //let obj8=vm_instance.new_object();println!("obj8:{:?}",obj8);
    
    println!("young:");
    vm_instance.dump_young();
    
    println!("old:");
    vm_instance.dump_old();
    
}

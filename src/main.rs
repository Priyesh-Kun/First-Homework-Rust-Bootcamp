struct Task {
    id: u8,
    description: String,
    completed: bool,
}

fn add_task(description: String, id: &mut u8) -> Task{
    let new_task = Task{
        id:*id,
        description:description,
        completed: false
    };
    *id = *id + 1;
    new_task
}

fn complete_task(id:u8,tasks_ref:&mut Vec<Task>) -> Option<&Task> {
    for element in tasks_ref {
        if element.id == id {
            element.completed = true;
            return Some(element);
        }
    }
    None
}

fn list_tasks(task_ref: &Vec<Task>){
    for element in task_ref{
        println!(" ");
        println!("Id: {}",element.id);
        println!("Description: {}",element.description);
        println!("Status: {}",element.completed);
        println!(" ");
    }
}

fn main(){
    let mut tasks:Vec<Task> = Vec::new();
    let mut id:u8 = 0;
    let description = String::from("Description");
    let description2 = String::from("Description");
    tasks.push(add_task(description, &mut id));
    tasks.push(add_task(description2, &mut id));
    list_tasks(&tasks);
    complete_task(0, &mut tasks);
    list_tasks(&tasks);

}
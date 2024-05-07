use std::io;

fn main (){
    println!("To-Do-List");

    let mut list : Vec<String> = Vec::new();


    loop {
        println!("Enter a command: ");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. List all tasks");
        println!("4. Exit");


        let mut command = String::new();

        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
        

        let command = command.trim();

        match command {

            "1" => {
                println!("Enter a task: ");
                let mut task = String::new();
                io::stdin().read_line(&mut task)
                    .expect("Failed to read line");
                list.push(task);
            },
            "2" => {
                println!("Enter the task you want to remove");
                let mut task = String::new();
                io::stdin().read_line(&mut task)
                    .expect("Failed to read line");
                    
                let task = task.trim();
                let mut index = 0;
                let mut found = false;
                for (i, t) in list.iter().enumerate() {
                    if t.trim() == task {
                        index = i;
                        found = true;
                        break;
                    }
                }
                if found {
                    list.remove(index);
                } else {
                    println!("Task not found");
                }
            },

            "3" => {
                for task in list.iter() {
                    println!("{}", task);
                }
            },
            "4" => {
                break;
            },
            _ => {
                println!("Invalid command");
            }

        }
    }
}
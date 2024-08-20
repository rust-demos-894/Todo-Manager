use std::fs;
use std::io;
use std::process::exit;
use serde::{Serialize, Deserialize};
use std::env;
use text_io;

const PATH: &'static str = "/data/todo.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    pub content: String,
}

impl Task {
    fn new(str: &str) -> Self {
        Task {
            content: str.into(),
        }
    }

    fn modify(&mut self, content: String) -> String {
        let s = self.content.clone();
        self.content = content;
        s
    }

    fn print(&self) {
        println!("Task: {:?}", self.content);
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            content: "To Be Determined\nwrite something to do here.".to_string(),
        }
    }
}

#[derive(Debug)]
struct List {
    count: u8,
    todo_list: Vec<Task>,
}

impl List {
    fn new(tasks: Vec<Task>) -> Self {
        List {
            count: tasks.len() as u8,
            todo_list: tasks,
        }
    }

    fn get(&self, n: u8) -> Result<&Task, ()> {
        self.todo_list.iter().nth(n as usize).ok_or(())
    }

    fn display(&self) {
        if self.count == 0 {
            println!("We have an empty todo list!");
            return ();
        }
        println!("Total Tasks: [{}]\n", self.count);
        let mut j: u8 = 0;
        for i in &self.todo_list {
            print!("{j} ");
            i.print();
            println!("");
            j+=1;
        }
    }

    fn add_task(&mut self, t: Task) {
        self.todo_list.push(t);
        self.count += 1;
    }

    fn add_default(&mut self) {
        self.todo_list.push(Task::default());
        self.count += 1;
    }

    fn del_task(&mut self, n: u8) -> Result<Task, &'static str> {
        if n >= self.count {
            return Err("Index out of Range: {n}.");
        }
        self.count -= 1;

        Ok(self.todo_list.remove(n as usize))
    }

    fn update(&mut self, n: u8, new_content: String) -> Result<String, &'static str> {
        let target = self.todo_list.iter_mut().nth(n as usize);
        let old_content;
        if target.is_none() {
            return Err("Index out of Range: {n}.");
        }
        else {
            old_content = target.unwrap().modify(new_content);
        }

        Ok(old_content)
    }

    fn clear(&mut self) {
        self.todo_list.clear();
        self.count = 0;
    }
}

impl Default for List {
    fn default() -> Self {
        List {
            count: 0,
            todo_list: Vec::new(),
        }
    }
}

fn main() -> io::Result<()>{//Result<(), io::Error>
    let mut list = init();

    loop {
        let cmd: String = text_io::read!("{}\r\n");
        match cmd {
            s if s.starts_with("ADD") => {
                if let Some(nc) = s.splitn(2, " ").nth(1) {
                    let t = Task::new(nc);
                    list.add_task(t);
                }
                else {
                    list.add_default();
                }
                println!("Adding Task Successfully!")
            },
            s if s.starts_with("DEL") => {
                if let Some(id) = s.splitn(2, " ").nth(1) {
                    if let Ok(index) = id.parse() {
                        if list.del_task(index).is_err() {
                            println!("Index out of Range: {index}.")
                        }
                        else {
                            println!("Deleting Task Successfully!")
                        }
                    }
                    else {
                        println!("Not a Number: {id}.");
                    }                    
                }
                else {
                    println!("No Index Specified.")
                }
            },
            s if s.starts_with("UPDATE") => {
                if let Some(id) = s.splitn(2, " ").nth(1) {
                    if let Ok(index) = id.parse() {
                        if list.count <= index {
                            println!("Index out of Range: {index}.")
                        }
                        else {
                            let t = list.get(index).unwrap();
                            let buf: String;
                            println!("Your Chosen Task:\n");
                            t.print();
                            println!("\nPlease enter the new content:");
                            buf = text_io::read!("{}\r\n");
                            //    .expect("Error Reading Input.");
                            list.update(index, buf).unwrap();
                            println!("Updating Task Successfully!")
                        }
                    }
                    else {
                        println!("Not a Number: {id}.");
                    }                    
                }
                else {
                    println!("No Index Specified.")
                }
            },
            s if s.starts_with("TODO") => {
                list.display();
            },
            s if s == "EXIT".to_string() => {
                println!("exiting the program...");
                save(list)?;
                exit(0);
            },
            s if s == "CLEAR".to_string() => {
                list.clear();
                println!("Clear up the Todo List Successfully!");
            },
            ns => {
                println!("unknown command: {ns}")
            }
        }
    }

    //Ok(())
}

fn read_from_file(file_name: &str) -> Result<Vec<Task>, io::Error> {
    let file = fs::File::open(file_name)?;
    let buffer = io::BufReader::new(file);
    let tasks = serde_json::from_reader(buffer)?;
    Ok(tasks)
}

fn init() -> List {
    let default_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), PATH);
    let read_res = read_from_file(&default_path);
    let tasks;
    let list;
    match read_res {
        Ok(t) => {
            tasks = t;
            list = List::new(tasks);
            list.display();
        }
        Err(_) => {
            println!("We have an empty todo list!");
            list = List::default();
        }
    }

    list
}

fn save_to_files(file_name: &str, list: List) -> io::Result<()>{
    let file = fs::File::create(file_name)?;
    serde_json::to_writer(file, &list.todo_list)?;
    Ok(())
}

fn save(list: List) -> io::Result<()> {
    let default_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), PATH);
    save_to_files(&default_path, list)
}

/*
fn clear_screen() {
    // ANSI 转义码，清空屏幕并将光标移动到左上角
    print!("\x1B[2J\x1B[1;1H");
}
*/
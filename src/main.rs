use std::io;

mod task;

fn main() -> io::Result<()> {
    let mut buffer;
    let mut v: Vec<task::Task> = Vec::new();

    loop {
        // Current tasks
        println!("========================================");
        println!("Your current tasks are:");
        for t in &v {
            println!("{:?}", t);
        }
        println!("========================================");

        // Input prompt
        println!("(a)dd task\n(s)tart task\n(f)inish task\n(q)uit");

        // Read input
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        buffer.pop();

        match &*buffer {
            "a" => {
                // Add a new task
                println!("Please input the task's text:");
                buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                buffer.pop();
                v.push(task::Task::new(v.len(), buffer));
            },
            "s" => {
                // Start a task
                println!("Please input the task to start's id:");
                buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                buffer.pop();
                let tmp = v[buffer.parse::<usize>().unwrap()].start();
                println!("{}", tmp);
            },
            "f" => {
                // Start a task
                println!("Please input the finished task's id:");
                buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                buffer.pop();
                let tmp = v[buffer.parse::<usize>().unwrap()].finish();
                println!("{}", tmp);
            },
            "q" => break,
            _ => {
                println!("Unknown Command");
            },
        }
    }
    Ok(())
}

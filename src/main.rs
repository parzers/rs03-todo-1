use std::io;

struct TodoItem
{
    text: String,
    marked: bool
}

fn print_list(list: &Vec<TodoItem>)
{
    for idx in 0..list.len() {
        let item = &list[idx];
        let mark = match item.marked {
            false => '☐',
            true => '☑',
        };
        println!("{} {}: {}", mark, idx + 1, item.text);
    }
}

fn main() {
    println!("TODO LIST");
    let mut list: Vec<TodoItem> = Vec::new();
    loop {
        println!("Commands: add, mark, clear, quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_t = input.trim();
        if input_t == "add" {
            println!("Text for the new item?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let text = input.trim().to_string();
            list.push(TodoItem {text, marked: false});
            print_list(&list);
        }
        if input_t == "mark" {
            println!("Which item do you want to mark (number)?");
            let mut text = String::new();
            io::stdin().read_line(&mut text).unwrap();
            let num: usize = match text.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Number not recognized!");
                    continue;
                }
            };
            let idx = match num.checked_sub(1) {
                Some(x) => x,
                None => {
                    println!("Number has to be greater than zero!");
                    continue;
                }
            };
            if let Some(item) = list.get_mut(idx) {
                item.marked = true;
            } else {
                println!("There is no item with number {}!", num);
            }
            print_list(&list);
        }
        if input_t == "clear" {
            let old_len = list.len();
            list = list.into_iter()
                .filter(|x| x.marked == false)
                .collect();
            println!("{} marked items have been cleared.", old_len - list.len());
            print_list(&list);
        }
        if input_t == "quit" {
            break;
        }
    }
    println!("Thanks, bye!");
}

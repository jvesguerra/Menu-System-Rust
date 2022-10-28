use std::io;

struct MenuItem {
    item_id : u32 ,
    item_name : String ,
    food_establishment : String ,
    item_price : f64 ,
    item_stock : u32
}

// struct Customer {
//     name : String ,
//     orders : Vec < String > ,
//     total_cost : f64
// }

fn main() {

    // Data declarations
    //let mut global_item_id:u32 = 1;
    let mut runner:i32 = 0;
    let menu = "[1] Add a Menu Item\n[2] Order a Menu Item\n[3] Edit a Menu Item\n[4] Delete a Menu Item\n[5] View All Menu Items\n[6] View All Customers\n[7] Exit".to_string();   
    let mut menu_list: Vec<MenuItem> = Vec::new();
    //let mut customer_list =  Vec::new();

    // Program
    while runner != 1 {
        let mut user_input = String::new(); // reset user input

        println!("{}",menu);    // print menu

        // Take user input then match
        println!("Enter choice: ");
        io::stdin().read_line(&mut user_input).expect("Error");
        let user_input : isize = user_input.trim().parse().expect(" error");

        match user_input{
            1 => add_menu_item(&mut menu_list),
            2 => println!("Order Menu Item"),
            3 => edit_menu_item(&mut menu_list),
            4 => delete_menu_item(&mut menu_list),
            5 => view_all_menu_item(&menu_list),
            6 => println!("View All Customers"),
            7 => runner = exit_function(),
            _ => println!("Error"),
        }
    }

}

fn check_menu_item(menu_list: &mut Vec<MenuItem>, item: usize) -> bool{
    let mut x = false;
    for data in menu_list.iter(){
        if item == data.item_id.try_into().unwrap()  {
            x = true;
        }
    }
    return x;
}

fn check_menu_item_u32(menu_list: &mut Vec<MenuItem>, item: u32) -> bool{
    let mut x = false;
    for data in menu_list.iter(){
        if item == data.item_id  {
            x = true;
        }
    }
    return x;
}

fn exit_function() -> i32{
    println!("Goodbye!");
    let runner_f:i32 = 1;
    return runner_f;
}

fn edit_menu_item(menu_list: &mut Vec<MenuItem>){
    let mut new_item_price = String::new();
    let mut new_item_stock = String::new();
    let mut edit_item_index = String::new();

    // prompt user to put item id to be edited
    println!("Enter item id to be edited: ");
    io::stdin().read_line(&mut edit_item_index).expect("Error");
    let edit_item_index : usize = edit_item_index.trim().parse().expect(" error");

     // check if menu item exist
    let x = check_menu_item(menu_list,edit_item_index);

    if x {
        // item price
        println!("Enter new item price: ");
        io::stdin().read_line(&mut new_item_price).expect("Error");
        let new_item_price : f64 = new_item_price.trim().parse().expect("error");

        // item stock
        println!("Enter new item stock: ");
        io::stdin().read_line(&mut new_item_stock).expect("Error");
        let new_item_stock : u32 = new_item_stock.trim().parse().expect("error");

        menu_list[edit_item_index - 1].item_price = new_item_price;
        menu_list[edit_item_index - 1].item_stock = new_item_stock;
    }else{
        println!("Menu item does not exist!");
    }

}

fn delete_menu_item(menu_list:&mut Vec<MenuItem>){
    let mut remove_item_index = String::new();

    // print item ids
    for data in menu_list.iter(){
        println!("\nItem id: {}",data.item_id);
        print!("Item name: {}",data.item_name);
        println!("Food Establishment: {}",data.food_establishment);
    }

    // prompt user to put item id to be removed
    println!("Enter item id to be removed: ");
    io::stdin().read_line(&mut remove_item_index).expect("Error");
    let remove_item_index : usize = remove_item_index.trim().parse().expect(" error");

    // check if menu item exist
    let x = check_menu_item(menu_list,remove_item_index);

    if x {
        // adjust value since vec is 0 index
        menu_list.remove(remove_item_index - 1);
        println!("Successfully deleted menu item");
    }else{
        println!("Menu item does not exist!");
    }

}

fn create_menu_item(item_id:u32,item_name:String,food_establishment:String,item_price:f64,item_stock:u32) -> MenuItem{
    MenuItem{
        item_id:item_id,
        item_name: item_name,
        food_establishment: food_establishment,
        item_price: item_price,
        item_stock:item_stock,
    }
}

fn add_menu_item(menu_list:&mut Vec<MenuItem>){
    //declarations
    let mut new_item_id = String::new();
    let mut new_item_name = String::new();
    let mut new_food_establishment = String::new();
    let mut new_item_price = String::new();
    let mut new_item_stock = String::new();

    //item id
    println!("Enter item id: ");
    io::stdin().read_line(&mut new_item_id).expect("Error");
    let new_item_id : u32 = new_item_id.trim().parse().expect(" error");

    let x = check_menu_item_u32(menu_list,new_item_id);

    if x {
        println!("Item id already exists!");
    }else{
        //item name
        println!("Enter item name: ");
        io::stdin().read_line(&mut new_item_name).expect("Error");

        //food establishment
        println!("Enter food establishment: ");
        io::stdin().read_line(&mut new_food_establishment).expect("Error");

        // item price
        println!("Enter item price: ");
        io::stdin().read_line(&mut new_item_price).expect("Error");
        let new_item_price : f64 = new_item_price.trim().parse().expect("error");

        // item stock
        println!("Enter item stock: ");
        io::stdin().read_line(&mut new_item_stock).expect("Error");
        let new_item_stock : u32 = new_item_stock.trim().parse().expect("error");


        let new_menu_item = create_menu_item(new_item_id,new_item_name,new_food_establishment,new_item_price,new_item_stock);

        menu_list.push(new_menu_item);
    }
}


fn view_all_menu_item(menu_item_list:&Vec<MenuItem>){
    for data in menu_item_list.iter(){
        print!("\nItem id: {} \n",data.item_id);
        print!("Item name: {}",data.item_name);
        print!("Food Establishment: {}",data.food_establishment);
        print!("Item price: {} \n",data.item_price);
        println!("Item stock: {} \n",data.item_stock);
    }
}
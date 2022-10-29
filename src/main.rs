/*
Joshua V. Esguerra
CMSC 124 - B1L

Menu System Rust

- main()
- create functions()
- check functions()
- exit_function()
- view functions()
- order_menu_item()
- add_menu_item()
- edit_menu_item()
- delete_menu_item()
*/

use std::io;
use std::io::Write; 

struct MenuItem {
    item_id : u32 ,
    item_name : String ,
    food_establishment : String ,
    item_price : f64 ,
    item_stock : u32
}

struct Customer {
    name : String,
    orders : Vec<String>,
    total_cost : f64
}

fn main() {

    // Data declarations
    let mut runner:i32 = 0;
    let menu = "\n[1] Add a Menu Item\n[2] Order a Menu Item\n[3] Edit a Menu Item\n[4] Delete a Menu Item\n[5] View All Menu Items\n[6] View All Customers\n[7] Exit\n".to_string();   
    let mut menu_list: Vec<MenuItem> = Vec::new();
    let mut customer_list: Vec<Customer> =  Vec::new();

    // Program
    while runner != 1 {
        let mut user_input = String::new(); // reset user input

        println!("{}",menu);    // print menu

        // Take user input then match
        print!("Enter choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).expect("Error");
        let user_input : isize = user_input.trim().parse().expect(" error");

        match user_input{
            1 => add_menu_item(&mut menu_list),
            2 => order_menu_item(&mut customer_list,&mut menu_list),
            3 => edit_menu_item(&mut menu_list),
            4 => delete_menu_item(&mut menu_list),
            5 => view_all_menu_item(&menu_list),
            6 => view_all_customers(&customer_list),
            7 => runner = exit_function(),
            _ => println!("Error"),
        }
    }

}

// create functions

fn create_customer(name:String,orders:Vec<String>,total_cost: f64) -> Customer{
    Customer{
        name: name,
        orders: orders,
        total_cost: total_cost,
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

// check functions
fn check_customer(customer_list: &mut Vec<Customer>, customer: &String) -> bool{
    let mut x = false;
    for data in customer_list.iter(){
        if customer == &data.name {
            x = true;
        }
    }
    return x;
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

// exit functions
fn exit_function() -> i32{
    println!("\nGoodbye!");
    let runner_f:i32 = 1;
    return runner_f;
}

// view functions
fn view_all_menu_item(menu_item_list:&Vec<MenuItem>){
    if menu_item_list.len() == 0{
        println!("No menu items yet!");
    }else{
        for menu_item in menu_item_list.iter(){
            print!("\nItem id: {} \n",menu_item.item_id);
            print!("Item name: {} \n",menu_item.item_name.trim());
            print!("Food Establishment: {} \n",menu_item.food_establishment.trim());
            print!("Item price: {} \n",menu_item.item_price);
            println!("Item stock: {}",menu_item.item_stock);
        }       
    }

}

fn view_all_customers(customer_list: &Vec<Customer>){
    if customer_list.len() != 0 {
        for customer in customer_list.iter(){
            print!("\nCustomer name: {}",customer.name);
            print!("Menu Items Ordered:");
            for order in customer.orders.iter(){
                print!("\n {}\n",order.trim()); 
            }
            print!("\nTotal Cost: {}\n",customer.total_cost);
        }
    }else{
        println!("There are no customers yet!");
    }

}

// add functions
fn order_menu_item(customer_list: &mut Vec<Customer>, menu_list: &mut Vec<MenuItem>){

    // data declarations
    let mut customer_name = String::new();
    let mut customer_order = String::new();
    let mut order_list: Vec<String> = Vec::new();

    // checks if there are menu items available
    if menu_list.len() != 0 {
        print!("Enter customer name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut customer_name).expect("Error");

        // checks if customer already exists
        let check_customer_exist = check_customer(customer_list,&customer_name);

        println!("MENU ITEMS AVAILABLE");
        for menu_item in menu_list.iter(){
            print!("[{}] {} ({}) - {}",menu_item.item_id, menu_item.item_name.trim(),menu_item.food_establishment.trim(),menu_item.item_price);
        }

        print!("\nEnter menu id to order: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut customer_order).expect("Error");
        let customer_order : usize = customer_order.trim().parse().expect(" error");

        // check if menu item exist
        let x = check_menu_item(menu_list,customer_order);

        if x {
            if check_customer_exist {
                // find position of customer
                let mut final_index = 0;
                for data in customer_list.iter(){
                    if customer_name == data.name {
                        println!("{}", final_index);
                    }else{
                        final_index += 1;
                    }
                }

                println!("Stock: {}",menu_list[customer_order-1].item_stock);
                if menu_list[customer_order-1].item_stock != 0 {
                    customer_list[final_index].orders.push(menu_list[customer_order-1].item_name.to_string());

                    let update_total_cost = customer_list[final_index].total_cost + menu_list[customer_order-1].item_price;
                    let update_total_stock = menu_list[customer_order-1].item_stock - 1;

                    // update total cost
                    customer_list[final_index].total_cost = update_total_cost;

                    //update stock
                    menu_list[customer_order-1].item_stock = update_total_stock;

                    println!("\nSuccessfully ordered menu item {} {}_{}",menu_list[customer_order-1].item_id, menu_list[customer_order-1].item_name.trim(), menu_list[customer_order-1].food_establishment.trim());
                }else{
                    println!("\nMenu item is out of stock!");
                }
            }else{
                if menu_list[customer_order-1].item_stock != 0 {
                    order_list.push(menu_list[customer_order-1].item_name.to_string());

                    //create customer
                    let new_customer= create_customer(customer_name,order_list,menu_list[customer_order-1].item_price);

                    // append new customer to customer list
                    customer_list.push(new_customer);

                    //update stock
                    let update_total_stock = menu_list[customer_order-1].item_stock - 1;
                    menu_list[customer_order-1].item_stock = update_total_stock;

                    println!("\nSuccessfully ordered menu item {} {}_{}",menu_list[customer_order-1].item_id, menu_list[customer_order-1].item_name.trim(), menu_list[customer_order-1].food_establishment.trim());
                }else{
                    println!("\nMenu item is out of stock!");
                }
            }
        }else{
            println!("\nMenu ID not found!");
        }
    }else{
       println!("There are no menu items available"); 
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
    print!("\nEnter item id: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_item_id).expect("Error");
    let new_item_id : u32 = new_item_id.trim().parse().expect(" error");

    let x = check_menu_item_u32(menu_list,new_item_id);

    if x {
        println!("\nMenu ID already exists!");
    }else{
        //item name
        print!("Enter item name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut new_item_name).expect("Error");

        //food establishment
        print!("Enter food establishment: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut new_food_establishment).expect("Error");

        // item price
        print!("Enter item price: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut new_item_price).expect("Error");
        let new_item_price : f64 = new_item_price.trim().parse().expect("error");

        // item stock
        print!("Enter item stock:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut new_item_stock).expect("Error");
        let new_item_stock : u32 = new_item_stock.trim().parse().expect("error");


        let new_menu_item = create_menu_item(new_item_id,new_item_name,new_food_establishment,new_item_price,new_item_stock);

        menu_list.push(new_menu_item);

        println!("\nSuccessfully added menu item!");
    }
}

// edit function
fn edit_menu_item(menu_list: &mut Vec<MenuItem>){
    let mut new_item_price = String::new();
    let mut new_item_stock = String::new();
    let mut edit_item_index = String::new();

    // prompt user to put item id to be edited
    print!("Enter item id to be edited: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut edit_item_index).expect("Error");
    let edit_item_index : usize = edit_item_index.trim().parse().expect(" error");

     // check if menu item exist
    let x = check_menu_item(menu_list,edit_item_index);

    if x {
        // item price
        print!("Enter new item price: ");
        io::stdin().read_line(&mut new_item_price).expect("Error");
        let new_item_price : f64 = new_item_price.trim().parse().expect("error");

        // item stock
        print!("Enter new item stock: ");
        io::stdin().read_line(&mut new_item_stock).expect("Error");
        let new_item_stock : u32 = new_item_stock.trim().parse().expect("error");

        menu_list[edit_item_index - 1].item_price = new_item_price;
        menu_list[edit_item_index - 1].item_stock = new_item_stock;

        println!("Menu Item Successfully Edited!");
    }else{
        println!("Menu item does not exist!");
    }

}

// delete function
fn delete_menu_item(menu_list:&mut Vec<MenuItem>){
    let mut remove_item_index = String::new();

    // print item ids
    for data in menu_list.iter(){
        println!("\nItem id: {}",data.item_id);
        print!("Item name: {}",data.item_name);
        println!("Food Establishment: {}",data.food_establishment);
    }

    // prompt user to put item id to be removed
    print!("Enter item id to be removed: ");
    io::stdout().flush().unwrap();
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







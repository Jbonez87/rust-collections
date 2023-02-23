use std::collections::HashMap;

fn get_all_employees(dict: &HashMap<String, Vec<String>>) -> Vec<String> {
    dict
      .values()
      .flat_map(|v| v.iter())
      .map(Clone::clone)
      .collect()
}

pub fn exec_get(cmd_tokens: &[&str], dict: &HashMap<String, Vec<String>>) -> Result<(), &'static str> {
  if cmd_tokens.len() == 2 {
    let mut employees = if cmd_tokens[1] == "ALL" {
      get_all_employees(dict)
    } else {
      dict
        .get(cmd_tokens[1])
        .ok_or("No such department.")?
        .to_vec()
    };
    employees.sort();
    for employee in employees {
      println!("{}", employee);
    }
    Ok(())
  } else {
    Err("Invalid command.")
  }
}

pub fn exec_add(cmd_tokens: &[&str], dict: &mut HashMap<String, Vec<String>>) -> Result<(), &'static str> {
  if cmd_tokens.len() == 4 {
    dict
      .entry(cmd_tokens[3].to_string())
      .or_default()
      .push(cmd_tokens[1].to_string());
    println!("{} added to the department {}", cmd_tokens[1], cmd_tokens[3]);
    Ok(())
  } else {
    Err("Invalid Command.")
  }
}
// pub fn add_employees(employees: &mut HashMap<&str, &str>) {
//     println!("Enter the employees name");
//     let mut name: String = String::new();
//     stdin()
//         .read_line(&mut name)
//         .expect("Failed to read line");

//     println!("name is {name}");
    
//     println!("Enter the employees department");
//     let mut department: String = String::new();
//     stdin()
//         .read_line(&mut department)
//         .expect("Failed to read line");
    

//     // employees.insert(&name, &department);

//     println!("Add {name} to {department}");
// }
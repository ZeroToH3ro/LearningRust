struct Employee {
    position: Position,
    status: Status
}

enum Position { 
    IT,
    CEO,
    CTO,
    Manager,
    Marketer
}

fn try_access (employee: &Employee) -> Result<(), String>{
    match employee.status { 
        Status::Denied => return Err("Access Denied".to_owned()),
        _ => (),
    }
    match employee.position { 
        Position::IT => Ok(()),
        Position::CTO => Ok(()),
        Position::CEO => Ok(()),
        Position::Manager => Ok(()),
        Position::Marketer => Ok(()),
        _ => Err("Invalid position".to_owned()),
    }
}

enum Status { 
    Denied,
    Active
}

fn main() {
    let manager = Employee {
        position: Position::Manager,
        status: Status::Active,
    };

    let access = try_access(&manager);
    if access.is_ok() {
        println!("Okie");
    }
}
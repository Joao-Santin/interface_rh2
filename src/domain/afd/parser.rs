use crate::app::state::AppState;
use super::afd::RegistryType;

pub fn parse_afd_lines(appstate: &mut AppState, afd_string: String){
    for line in afd_string.lines(){
        if let Some(c) = line.chars().nth(9){
            let n: i8 = c.to_digit(10).unwrap_or(0) as i8;
            if let Some(registry_type) = RegistryType::get_registry_type_by_number(n){
                registry_type.parse_afd(appstate, line)

            }else{
                println!("No type with that number")
            }
        }else{
            println!("")
        }
    }

}

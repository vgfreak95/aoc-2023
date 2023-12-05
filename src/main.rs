use std::fs;

#[derive(Debug)]
struct LinePair {
    // For Deubgging Purpose
    line_found: i32,

    // No reference because direct move occurs
    line_data: String
}


impl LinePair {
    
    /*
    * Best Case O(1) : Digits are on the end of both sides of read line
    * Average Case O(n^2) : Digits are not on the end of both sides of read line 
    * Worst Case: Single Digit within read line, to form 2 digit number -> treb7uchet (77)
    */

    pub fn new(line_found: i32, line_data: String) -> Self {
        Self {line_found, line_data}
    }

    pub fn get_calibration_value(&mut self) -> i32 { 

        let mut left = self.line_data.chars().nth(0).unwrap_or_else(
            || { panic!("Error: Couldn't read first character."); }
        );
        

        let mut right = self.line_data.chars().nth(self.line_data.len()-1).unwrap_or_else(
            || { panic!("Error: Couldn't read last character."); }
        );

        // Check Best Case
        if char::is_digit(left, 10) && char::is_digit(right, 10) {
            println!(
                "Best Case: O(1) case was found on line: {} | First: {} | Last: {}",
                self.line_found, left, right
            );
            let two_digit_value = left.to_string() + &right.to_string();
            return two_digit_value.parse::<i32>().unwrap_or_else(
                |err| { panic!("Best Case: Couldn't parse the calibration value properly: {}", err); }
            );
        }

        // Check Average Case
        let mut far_left = 0;
        let mut far_right = self.line_data.len()-1;

        while far_left < far_right || far_left == far_right {

            // Check the values
            left = self.line_data.chars().nth(far_left).unwrap();
            right = self.line_data.chars().nth(far_right).unwrap();

            if char::is_digit(left, 10) && char::is_digit(right, 10) {
                break;
            }

            if !char::is_digit(left, 10) { far_left+=1; }
            if !char::is_digit(right, 10) { far_right-=1; }

        }

        // Still Average Case
        let two_digit_value = left.to_string() + &right.to_string();
        return two_digit_value.parse::<i32>().unwrap_or_else(
            |err| { panic!(
            "Average or Worst Case: Couldn't parse the calibration value properly:
            {}, Left: {}, Right: {}, LineData: {}",
            err, left, right, self.line_data); }
        );
    }
}

fn main() {

    let test_input = fs::read_to_string("src/real_input.txt")
        .unwrap_or_else(|err| format!("Couldn't open file: {}", err));
    
    let splitted = test_input
        .trim_end()
        .split("\n")
        .into_iter();

    let mut line_pairs: Vec<LinePair> = Vec::new();

    for (line_found, line_data) in splitted.enumerate() {
       line_pairs.push( LinePair::new( line_found as i32, line_data.to_string()) );
    }

    let final_result: i32 = line_pairs
        .into_iter()
        .map(|mut read_line| read_line.get_calibration_value() )
        .sum();

    println!("Final Calibration Value: {}", final_result);
}

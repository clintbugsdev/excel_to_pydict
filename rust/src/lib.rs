extern crate libc;
extern crate pyo3;
extern crate calamine;
extern crate serde;
extern crate serde_json;

use libc::c_char;
use std::ffi::CStr;
use std::path::Path;

use calamine::{open_workbook, Reader, Xlsx};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Product {
    SKU: String,
    designation: String,
    price: f32,
    description: String,
}


#[no_mangle]
pub extern "C" fn print_xlsx_file(f: *const c_char){
    // File
    let f_str = unsafe {
        assert!(!f.is_null());
        CStr::from_ptr(f)
    };
    // File Path
    let f_path = f_str.to_str().unwrap();
    // Panic! If File Doesn't Exists?
    let file_exists: bool = Path::new(f_path).exists();
    if !file_exists {
        panic!("File doesn't exists")
    }
    // Products
    let mut products = Vec::new();
    // Open File
    let mut excel: Xlsx<_> = open_workbook(f_path).expect("Invalid file");
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            // Set SKU
            let sku = row[0].to_string();
            // Set Designation
            let designation = row[1].to_string();
            // Set Price
            let s_price = row[2].to_string();
            // Skip First Row with Cell 1 & 2 w/ Value "SKU" and "Product Designation"
            if sku == "SKU" && designation == "Product Designation" {
                continue;
            }
            // Format Description
            let description = format!("{} ({}), ${}", designation, sku, s_price);
            // Parse Price
            let price = s_price.parse::<f32>().unwrap();
            // Set Product
            let prod = Product {
                SKU: sku,
                designation: designation,
                price: price,
                description: description,
            };
            // Add Product
            products.push(prod);
        }
    } else {
        panic!("Sheet1 is not present")
    }
    // Convert the Products to a JSON string.
    let s_products = format!("print('{}')", serde_json::to_string(&products).unwrap());
    // Initiate Python and Run Print Products
    let py = pyo3::Python::acquire_gil();
    py.python().run(&s_products, None, None).unwrap(); 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    #[should_panic(expected = "File doesn't exists")]
    fn test_no_file() {
        let f = CString::new("no_file.xsls").expect("Test Variable - CString::new failed");
        print_xlsx_file(f.as_ptr());
    }

    #[test]
    #[should_panic(expected = "Invalid file")]
    fn test_invalid_file() {
        let f = CString::new("test/invalid_file.docx").expect("Test Variable - CString::new failed");
        print_xlsx_file(f.as_ptr());
    }

    #[test]
    #[should_panic(expected = "Sheet1 is not present")]
    fn test_no_sheet1() {
        let f = CString::new("test/no_sheet1.xlsx").expect("Test Variable - CString::new failed");
        print_xlsx_file(f.as_ptr());
    }

    #[test]
    fn test_correct_file() {
        let f = CString::new("test/products.xlsx").expect("Test Variable - CString::new failed");
        print_xlsx_file(f.as_ptr());
    }
}

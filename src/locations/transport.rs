use mysql as my;

pub fn get_res_amount_at_location(location: u64, conn: &my::Pool) -> Result<f64, String> {
    match conn.first_exec("SELECT loc_val FROM resource_location WHERE id = ?", (location,)) {
        Ok(Some(row)) => Ok(row.get(0).unwrap()),
        Ok(None) => Err("No such resource location.".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn update_res_amount_at_location(amount: f64, location: u64, conn: &my::Pool) -> Result<(), String> {
    if let Err(e) = conn.prep_exec("UPDATE resource_location SET loc_val = loc_val + ? WHERE id = ?", (amount, location)) {
        Err(e.to_string())
    } else { Ok(()) }
}
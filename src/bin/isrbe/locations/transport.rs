use mysql as my;
use std::borrow::Cow;

pub fn get_res_amount_at_location(location: u64, conn: &my::Pool) -> Result<f64, Cow<'static, str>> {
    match conn.first_exec("SELECT loc_val FROM resource_location WHERE id = ?", (location,)) {
        Ok(Some(row)) => Ok(row.get(0).unwrap()),
        Ok(None) => Err(Cow::Borrowed("No such resource location.")),
        Err(e) => Err(Cow::Owned(e.to_string())),
    }
}

pub fn update_res_amount_at_location(amount: f64, location: u64, conn: &my::Pool) -> Result<(), Cow<'static, str>> {
    if let Err(e) = conn.prep_exec("UPDATE resource_location SET loc_val = loc_val + ? WHERE id = ?", (amount, location)) {
        Err(Cow::Owned(e.to_string()))
    } else { Ok(()) }
}
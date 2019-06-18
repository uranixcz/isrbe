SELECT resource_location.id, loc_val, loc_radius, location.lat, location.lon, quantity.unit
FROM resource_location
JOIN location ON loc_id = location.id
JOIN resource_param ON resource_param.id = resource_location.res_param_id
JOIN param ON param.id = resource_param.param_id
JOIN quantity ON quantity.id = param.qty_id
WHERE res_id = ?
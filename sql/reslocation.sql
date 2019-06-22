SELECT resource_location.id, loc_val, loc_radius, location.lat, location.lon, param.qty_id, ""
FROM resource_location
JOIN location ON resource_location.loc_id = location.id
JOIN resource_param ON resource_param.id = resource_param.id
JOIN param ON param.id = resource_param.param_id
WHERE resource_location.id = ?
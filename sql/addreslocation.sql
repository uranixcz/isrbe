SELECT resource_param.id, param.name, unit
FROM resource_param
JOIN param ON param.id = resource_param.param_id
JOIN quantity ON quantity.id = param.qty_id
WHERE res_id = ?
AND is_movable = 1
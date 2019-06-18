SELECT param.id, param.name, param_float.val, quantity.unit, is_movable
FROM resource_param
JOIN param ON param.id = resource_param.param_id
JOIN param_float ON param_float.res_qty_id = resource_param.id
JOIN quantity ON quantity.id = param.qty_id
WHERE res_id = ?
SELECT param.id, param.name, param_val.val_float, param_val.val_text, param_val.val_res, quantity.unit, is_movable
FROM resource_param
JOIN param ON param.id = resource_param.param_id
LEFT JOIN param_val ON param_val.res_param_id = resource_param.id
JOIN quantity ON quantity.id = param.qty_id
WHERE res_id = ?